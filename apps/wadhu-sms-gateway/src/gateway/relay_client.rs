//! Outbound WebSocket client to a self-hosted `sms-relay`.
//!
//! Local LAN/BT send never depends on this connection.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::message::Status;
use super::sync_lock::lock_mutex;
use super::outbox::Outbox;
use super::ws::WsClient;

#[derive(Clone, Debug, Default)]
pub struct RelayConfig {
    pub url: String,
    pub device_id: String,
    pub device_secret: String,
}

#[derive(Clone, Debug)]
pub struct RelayStatus {
    pub configured: bool,
    pub connected: bool,
    pub detail: String,
    pub last_error: String,
}

impl Default for RelayStatus {
    fn default() -> Self {
        Self {
            configured: false,
            connected: false,
            detail: "not configured".into(),
            last_error: String::new(),
        }
    }
}

#[derive(Clone)]
pub struct RelayHandle {
    pub status: Arc<Mutex<RelayStatus>>,
    config: Arc<Mutex<RelayConfig>>,
    stop: Arc<AtomicBool>,
}

impl RelayHandle {
    pub fn snapshot(&self) -> RelayStatus {
        lock_mutex(&self.status).clone()
    }

    pub fn config(&self) -> RelayConfig {
        lock_mutex(&self.config).clone()
    }

    pub fn update_config(&self, cfg: RelayConfig) {
        *lock_mutex(&self.config) = cfg;
    }

    /// Signal relay worker threads to exit (used on gateway shutdown).
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

pub fn spawn(outbox: Outbox, config: RelayConfig) -> RelayHandle {
    let status = Arc::new(Mutex::new(RelayStatus {
        configured: !config.url.is_empty()
            && !config.device_id.is_empty()
            && !config.device_secret.is_empty(),
        connected: false,
        detail: if config.url.is_empty() {
            "not configured".into()
        } else {
            "connecting…".into()
        },
        last_error: String::new(),
    }));
    let config = Arc::new(Mutex::new(config));
    let stop = Arc::new(AtomicBool::new(false));
    let handle = RelayHandle {
        status: Arc::clone(&status),
        config: Arc::clone(&config),
        stop: Arc::clone(&stop),
    };

    let outbox_c = outbox.clone();
    let status_c = Arc::clone(&status);
    let config_c = Arc::clone(&config);
    let stop_c = Arc::clone(&stop);
    thread::Builder::new()
        .name("sms-gateway-relay".into())
        .spawn(move || relay_loop(outbox_c, config_c, status_c, stop_c))
        .ok();

    // Status reporter: when local outbox messages change status, push to relay if connected.
    // The relay loop itself reports after accept; drain also notifies via a watcher thread.
    let outbox_w = outbox;
    let status_w = Arc::clone(&status);
    let config_w = config;
    let stop_w = stop;
    thread::Builder::new()
        .name("sms-gateway-relay-status".into())
        .spawn(move || status_watch_noop(outbox_w, config_w, status_w, stop_w))
        .ok();

    handle
}

fn status_watch_noop(
    _outbox: Outbox,
    _config: Arc<Mutex<RelayConfig>>,
    _status: Arc<Mutex<RelayStatus>>,
    stop: Arc<AtomicBool>,
) {
    // Placeholder for future push-on-change; relay_loop reports after each job.
    while !stop.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(5));
    }
}

fn relay_loop(
    outbox: Outbox,
    config: Arc<Mutex<RelayConfig>>,
    status: Arc<Mutex<RelayStatus>>,
    stop: Arc<AtomicBool>,
) {
    let mut backoff = 1u64;
    while !stop.load(Ordering::Relaxed) {
        let cfg = lock_mutex(&config).clone();
        if cfg.url.is_empty() || cfg.device_id.is_empty() || cfg.device_secret.is_empty() {
            if let Ok(mut s) = status.lock() {
                s.configured = false;
                s.connected = false;
                s.detail = "not configured".into();
            }
            thread::sleep(Duration::from_secs(2));
            continue;
        }
        if let Ok(mut s) = status.lock() {
            s.configured = true;
            s.detail = format!("dialing {}", cfg.url);
        }
        match connect_and_serve(&cfg, &outbox, &status, &stop) {
            Ok(()) => {
                backoff = 1;
            }
            Err(e) => {
                if let Ok(mut s) = status.lock() {
                    s.connected = false;
                    s.last_error = e.clone();
                    s.detail = format!("disconnected: {e}");
                }
                thread::sleep(Duration::from_secs(backoff.min(30)));
                backoff = (backoff * 2).min(30);
            }
        }
    }
}

fn connect_and_serve(
    cfg: &RelayConfig,
    outbox: &Outbox,
    status: &Arc<Mutex<RelayStatus>>,
    stop: &Arc<AtomicBool>,
) -> Result<(), String> {
    let (host, port, use_path_prefix) = parse_relay_url(&cfg.url)?;
    let path = format!(
        "{}/v1/device/ws?device_id={}",
        use_path_prefix.trim_end_matches('/'),
        url_encode(&cfg.device_id)
    );
    let mut ws = WsClient::connect(
        &host,
        port,
        &path,
        &[("Authorization", &format!("Bearer {}", cfg.device_secret))],
    )?;
    ws.send_text(&format!(
        r#"{{"type":"hello","device_id":{}}}"#,
        json_str(&cfg.device_id)
    ))?;
    if let Ok(mut s) = status.lock() {
        s.connected = true;
        s.detail = format!("online → {host}:{port}");
        s.last_error.clear();
    }

    while !stop.load(Ordering::Relaxed) {
        let msg = ws.read_text()?;
        handle_relay_message(&msg, outbox, &mut ws)?;
    }
    Ok(())
}

fn handle_relay_message(raw: &str, outbox: &Outbox, ws: &mut WsClient) -> Result<(), String> {
    let ty = json_get(raw, "type").unwrap_or_default();
    match ty.as_str() {
        "ping" => {
            ws.send_text(r#"{"type":"pong"}"#)?;
        }
        "job" => {
            let id = json_get(raw, "id").unwrap_or_default();
            let to = json_get(raw, "to").unwrap_or_default();
            let body = json_get(raw, "body").unwrap_or_default();
            let idem = json_get(raw, "idempotency_key").unwrap_or_else(|| id.clone());
            match outbox.accept_with_id(id.clone(), idem, to, body) {
                Ok(m) => {
                    ws.send_text(&format!(
                        r#"{{"type":"status","id":{},"status":{}}}"#,
                        json_str(&m.id),
                        json_str(m.status.as_str())
                    ))?;
                    // Spawn a short watcher to report terminal status.
                    let outbox2 = outbox.clone();
                    // We cannot easily share ws across threads; poll once after drain window via
                    // reporting in a blocking wait here for this job's terminal state (bounded).
                    for _ in 0..50 {
                        thread::sleep(Duration::from_millis(100));
                        if let Some(cur) = outbox2.get(&m.id) {
                            if matches!(
                                cur.status,
                                Status::Submitted
                                    | Status::Failed
                                    | Status::Delivered
                                    | Status::Cancelled
                                    | Status::Expired
                                    | Status::Indeterminate
                            ) {
                                let err = cur
                                    .error
                                    .as_ref()
                                    .map(|e| format!(r#","error":{}"#, json_str(e)))
                                    .unwrap_or_default();
                                ws.send_text(&format!(
                                    r#"{{"type":"status","id":{},"status":{}{}}}"#,
                                    json_str(&cur.id),
                                    json_str(cur.status.as_str()),
                                    err
                                ))?;
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    ws.send_text(&format!(
                        r#"{{"type":"status","id":{},"status":"failed","error":{}}}"#,
                        json_str(&id),
                        json_str(&e)
                    ))?;
                }
            }
        }
        "hello_ok" | "pong" => {}
        _ => {}
    }
    Ok(())
}

/// Parse `http://host:port` or `ws://host:port` or `host:port` into (host, port, path_prefix).
fn parse_relay_url(url: &str) -> Result<(String, u16, String), String> {
    let u = url.trim();
    let u = u
        .strip_prefix("https://")
        .or_else(|| u.strip_prefix("http://"))
        .or_else(|| u.strip_prefix("wss://"))
        .or_else(|| u.strip_prefix("ws://"))
        .unwrap_or(u);
    let (hostport, path) = match u.split_once('/') {
        Some((h, p)) => (h, format!("/{p}")),
        None => (u, String::new()),
    };
    let (host, port) = if let Some((h, p)) = hostport.split_once(':') {
        (
            h.to_string(),
            p.parse::<u16>().map_err(|_| format!("bad port in {url}"))?,
        )
    } else {
        (hostport.to_string(), 9876)
    };
    if host.is_empty() {
        return Err("empty relay host".into());
    }
    Ok((host, port, path))
}

fn url_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
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
    if after.starts_with("null") {
        return None;
    }
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
