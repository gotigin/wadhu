//! Bluetooth transport for SMS jobs.
//!
//! On Android, RFCOMM via JNI when available. On desktop, a TCP listener on
//! port 18766 speaks the same length-prefixed JSON framing so third parties
//! can integrate and test without a phone.
//!
//! Client framing (see `docs/BLUETOOTH.md`):
//! ```text
//! → {"op":"send","to":"+91…","body":"…","idempotency_key":"…","token":"…"}
//! ← {"ok":true,"message":{…}}  or  {"ok":false,"error":"…"}
//! ```

use std::io::{Read, Write};
#[cfg(not(target_os = "android"))]
use std::net::{SocketAddr, TcpListener};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use super::clients::{AuthFail, AuthKind, Clients};
use super::frame::{read_frame, write_frame};
use super::outbox::Outbox;

pub const BT_SIM_PORT: u16 = 18766;

#[derive(Clone)]
pub struct BluetoothStatus {
    pub enabled: bool,
    pub listening: bool,
    pub endpoint: String,
    pub detail: String,
}

#[derive(Clone)]
pub struct BluetoothHandle {
    pub status: Arc<std::sync::Mutex<BluetoothStatus>>,
    stop: Arc<AtomicBool>,
}

impl BluetoothHandle {
    pub fn snapshot(&self) -> BluetoothStatus {
        self.status.lock().expect("bt status").clone()
    }

    /// Signal the listen loop to exit (used on gateway shutdown).
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// Start Bluetooth (or desktop framing simulator) transport.
pub fn spawn(
    outbox: Outbox,
    api_token: Arc<std::sync::Mutex<String>>,
    clients: Clients,
    enabled: bool,
) -> BluetoothHandle {
    let status = Arc::new(std::sync::Mutex::new(BluetoothStatus {
        enabled,
        listening: false,
        endpoint: String::new(),
        detail: if enabled {
            "starting".into()
        } else {
            "disabled".into()
        },
    }));
    let stop = Arc::new(AtomicBool::new(false));
    let handle = BluetoothHandle {
        status: Arc::clone(&status),
        stop: Arc::clone(&stop),
    };

    if !enabled {
        return handle;
    }

    #[cfg(target_os = "android")]
    {
        let st = Arc::clone(&status);
        let outbox_a = outbox.clone();
        let token_a = Arc::clone(&api_token);
        let clients_a = clients.clone();
        let stop_a = Arc::clone(&stop);
        thread::Builder::new()
            .name("sms-gateway-bt-android".into())
            .spawn(move || android_rfcomm_loop(outbox_a, token_a, clients_a, st, stop_a))
            .ok();
    }

    // Desktop / lab: length-prefixed framing over TCP :18766 (same JSON as RFCOMM).
    // Android primary offline BT path is Classic RFCOMM; TCP lab stays desktop-side.
    #[cfg(not(target_os = "android"))]
    {
        let st2 = Arc::clone(&status);
        let outbox2 = outbox;
        let token2 = api_token;
        let clients2 = clients;
        let stop2 = Arc::clone(&stop);
        thread::Builder::new()
            .name("sms-gateway-bt-sim".into())
            .spawn(move || framing_tcp_loop(outbox2, token2, clients2, st2, stop2))
            .ok();
    }

    handle
}

#[cfg(not(target_os = "android"))]
fn framing_tcp_loop(
    outbox: Outbox,
    api_token: Arc<std::sync::Mutex<String>>,
    clients: Clients,
    status: Arc<std::sync::Mutex<BluetoothStatus>>,
    stop: Arc<AtomicBool>,
) {
    let preferred: SocketAddr = format!("127.0.0.1:{BT_SIM_PORT}").parse().unwrap();
    let listener = match TcpListener::bind(preferred) {
        Ok(l) => l,
        Err(_) => match TcpListener::bind("127.0.0.1:0") {
            Ok(l) => l,
            Err(e) => {
                if let Ok(mut s) = status.lock() {
                    s.detail = format!("framing listen failed: {e}");
                }
                return;
            }
        },
    };
    let bound = listener.local_addr().ok();
    if let Ok(mut s) = status.lock() {
        s.listening = true;
        s.endpoint = bound
            .map(|a| format!("tcp://{a} (BT framing)"))
            .unwrap_or_else(|| format!("tcp://127.0.0.1:{BT_SIM_PORT} (BT framing)"));
        s.detail = "RFCOMM framing over TCP (desktop/sim); same JSON as Bluetooth".into();
    }
    for stream in listener.incoming().flatten() {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        let outbox = outbox.clone();
        let token = Arc::clone(&api_token);
        let clients = clients.clone();
        let _ = thread::spawn(move || handle_framed_client(stream, &outbox, &token, &clients));
    }
}

fn handle_framed_client(
    mut stream: impl Read + Write,
    outbox: &Outbox,
    api_token: &Arc<std::sync::Mutex<String>>,
    clients: &Clients,
) {
    loop {
        let raw = match read_frame(&mut stream) {
            Ok(s) => s,
            Err(_) => break,
        };
        let reply = handle_op(&raw, outbox, api_token, clients);
        if write_frame(&mut stream, &reply).is_err() {
            break;
        }
    }
}

fn handle_op(
    raw: &str,
    outbox: &Outbox,
    api_token: &Arc<std::sync::Mutex<String>>,
    clients: &Clients,
) -> String {
    let op = json_get(raw, "op").unwrap_or_default();
    match op.as_str() {
        "ping" => r#"{"ok":true,"pong":true}"#.into(),
        "send" => {
            let token = json_get(raw, "token").unwrap_or_default();
            let operator = api_token.lock().expect("token").clone();
            let auth = match clients.authorize(&token, &operator) {
                Ok(a) => a,
                Err(AuthFail::Blocked) => {
                    return r#"{"ok":false,"error":"blocked"}"#.into();
                }
                Err(AuthFail::Unauthorized) => {
                    return r#"{"ok":false,"error":"unauthorized"}"#.into();
                }
            };
            let to = json_get(raw, "to").unwrap_or_default();
            let body = json_get(raw, "body").unwrap_or_default();
            let idem = json_get(raw, "idempotency_key").unwrap_or_default();
            match outbox.accept(idem, to, body) {
                Ok(m) => {
                    if let AuthKind::Client { id, .. } = auth {
                        clients.record_send(&id);
                    }
                    format!(
                        r#"{{"ok":true,"message":{{"id":{},"status":{},"to":{},"idempotency_key":{}}}}}"#,
                        json_str(&m.id),
                        json_str(m.status.as_str()),
                        json_str(&m.to),
                        json_str(&m.idempotency_key)
                    )
                }
                Err(e) => format!(r#"{{"ok":false,"error":{}}}"#, json_str(&e)),
            }
        }
        _ => r#"{"ok":false,"error":"unknown_op"}"#.into(),
    }
}

#[cfg(target_os = "android")]
fn android_rfcomm_loop(
    outbox: Outbox,
    api_token: Arc<std::sync::Mutex<String>>,
    clients: Clients,
    status: Arc<std::sync::Mutex<BluetoothStatus>>,
    stop: Arc<AtomicBool>,
) {
    use makepad_widgets::makepad_platform::os::linux::android::android_jni::{
        to_java_bt_rfcomm_disconnect_client, to_java_bt_rfcomm_start,
        to_java_bt_rfcomm_status, to_java_bt_rfcomm_stop, to_java_bt_rfcomm_wait_client,
    };

    let started = unsafe { to_java_bt_rfcomm_start() };
    if let Ok(mut s) = status.lock() {
        s.endpoint = "rfcomm://Wadhu SMS Gateway (SPP)".into();
        s.detail = started.clone();
        s.listening = started == "listening" || started == "connected";
    }
    if started != "listening" && started != "connected" {
        // Keep polling start in case the user grants BT permission later.
        while !stop.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_secs(3));
            let again = unsafe { to_java_bt_rfcomm_start() };
            if let Ok(mut s) = status.lock() {
                s.detail = again.clone();
                s.listening = again == "listening" || again == "connected";
            }
            if again == "listening" || again == "connected" {
                break;
            }
        }
    }

    while !stop.load(Ordering::Relaxed) {
        let st = unsafe { to_java_bt_rfcomm_status() };
        if let Ok(mut s) = status.lock() {
            s.detail = st.clone();
            s.listening = st == "listening" || st == "connected";
        }
        if !unsafe { to_java_bt_rfcomm_wait_client(2000) } {
            continue;
        }
        if let Ok(mut s) = status.lock() {
            s.detail = "connected".into();
            s.listening = true;
        }
        let stream = AndroidBtStream;
        handle_framed_client(stream, &outbox, &api_token, &clients);
        unsafe {
            to_java_bt_rfcomm_disconnect_client();
        }
        if let Ok(mut s) = status.lock() {
            let st = unsafe { to_java_bt_rfcomm_status() };
            s.detail = st;
            s.listening = true;
        }
    }
    unsafe {
        to_java_bt_rfcomm_stop();
    }
}

/// JNI-backed byte stream for one RFCOMM client session.
#[cfg(target_os = "android")]
struct AndroidBtStream;

#[cfg(target_os = "android")]
impl Read for AndroidBtStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let n = buf.len().min(65536) as i32;
        let got = unsafe {
            makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_bt_rfcomm_read_exact(
                n, 120_000,
            )
        };
        match got {
            Some(bytes) if !bytes.is_empty() => {
                let take = bytes.len().min(buf.len());
                buf[..take].copy_from_slice(&bytes[..take]);
                Ok(take)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "bt rfcomm read failed",
            )),
        }
    }
}

#[cfg(target_os = "android")]
impl Write for AndroidBtStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let ok = unsafe {
            makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_bt_rfcomm_write_all(
                buf,
            )
        };
        if ok {
            Ok(buf.len())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "bt rfcomm write failed",
            ))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_get(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let i = body.find(&needle)?;
    let after = body[i + needle.len()..].trim_start().strip_prefix(':')?.trim_start();
    let rest = after.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => {
                if let Some(n) = chars.next() {
                    out.push(match n {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        c => c,
                    });
                }
            }
            c => out.push(c),
        }
    }
    None
}
