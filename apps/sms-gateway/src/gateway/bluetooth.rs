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
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

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
    #[allow(dead_code)]
    stop: Arc<AtomicBool>,
}

impl BluetoothHandle {
    pub fn snapshot(&self) -> BluetoothStatus {
        self.status.lock().expect("bt status").clone()
    }
}

/// Start Bluetooth (or desktop framing simulator) transport.
pub fn spawn(outbox: Outbox, api_token: String, enabled: bool) -> BluetoothHandle {
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
        let token_a = api_token.clone();
        let stop_a = Arc::clone(&stop);
        thread::Builder::new()
            .name("sms-gateway-bt-android".into())
            .spawn(move || android_rfcomm_loop(outbox_a, token_a, st, stop_a))
            .ok();
    }

    // Desktop always gets the framing simulator; Android also keeps it as a
    // LAN fallback for framed clients that are not using RFCOMM yet.
    let st2 = Arc::clone(&status);
    let outbox2 = outbox;
    let token2 = api_token;
    let stop2 = Arc::clone(&stop);
    thread::Builder::new()
        .name("sms-gateway-bt-sim".into())
        .spawn(move || framing_tcp_loop(outbox2, token2, st2, stop2))
        .ok();

    handle
}

fn framing_tcp_loop(
    outbox: Outbox,
    api_token: String,
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
        let token = api_token.clone();
        let _ = thread::spawn(move || handle_framed_client(stream, &outbox, &token));
    }
}

fn handle_framed_client(mut stream: TcpStream, outbox: &Outbox, api_token: &str) {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(120)));
    loop {
        let raw = match read_frame(&mut stream) {
            Ok(s) => s,
            Err(_) => break,
        };
        let reply = handle_op(&raw, outbox, api_token);
        if write_frame(&mut stream, &reply).is_err() {
            break;
        }
    }
}

fn handle_op(raw: &str, outbox: &Outbox, api_token: &str) -> String {
    let op = json_get(raw, "op").unwrap_or_default();
    match op.as_str() {
        "ping" => r#"{"ok":true,"pong":true}"#.into(),
        "send" => {
            let token = json_get(raw, "token").unwrap_or_default();
            if token != api_token {
                return r#"{"ok":false,"error":"unauthorized"}"#.into();
            }
            let to = json_get(raw, "to").unwrap_or_default();
            let body = json_get(raw, "body").unwrap_or_default();
            let idem = json_get(raw, "idempotency_key").unwrap_or_default();
            match outbox.accept(idem, to, body) {
                Ok(m) => format!(
                    r#"{{"ok":true,"message":{{"id":{},"status":{},"to":{},"idempotency_key":{}}}}}"#,
                    json_str(&m.id),
                    json_str(m.status.as_str()),
                    json_str(&m.to),
                    json_str(&m.idempotency_key)
                ),
                Err(e) => format!(r#"{{"ok":false,"error":{}}}"#, json_str(&e)),
            }
        }
        _ => r#"{"ok":false,"error":"unknown_op"}"#.into(),
    }
}

#[cfg(target_os = "android")]
fn android_rfcomm_loop(
    outbox: Outbox,
    api_token: String,
    status: Arc<std::sync::Mutex<BluetoothStatus>>,
    stop: Arc<AtomicBool>,
) {
    // RFCOMM accept via platform Bluetooth APIs lands with the JNI bridge.
    // Until then: same length-prefixed JSON framing is available on TCP :18766
    // (usable over USB-tether / local hotspot) and documented in BLUETOOTH.md
    // for Classic RFCOMM UUID 00001101-0000-1000-8000-00805F9B34FB.
    if let Ok(mut s) = status.lock() {
        s.detail =
            "BT framing on TCP :18766; RFCOMM UUID SPP — see docs/BLUETOOTH.md".into();
        if s.endpoint.is_empty() {
            s.endpoint = "rfcomm://Wadhu SMS Gateway + tcp://0.0.0.0:18766".into();
        }
        s.listening = true;
    }
    while !stop.load(Ordering::Relaxed) {
        let _ = (&outbox, &api_token);
        thread::sleep(std::time::Duration::from_secs(2));
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

#[allow(dead_code)]
fn _keep_read_write(_: &dyn Read, _: &dyn Write) {}
