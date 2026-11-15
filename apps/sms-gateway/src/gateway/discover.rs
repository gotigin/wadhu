//! LAN discovery beacon so brand apps find the gateway without typing an IP.
//!
//! Client broadcasts `WADHU_SMS?` to UDP `255.255.255.255:18767` (and/or the
//! subnet broadcast). Gateway replies `WADHU_SMS!{json}` with name + HTTP port.

use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// UDP port for discover requests / replies (HTTP stays on 18765).
pub const DISCOVER_PORT: u16 = 18767;

const REQ: &[u8] = b"WADHU_SMS?";
const RESP_PREFIX: &[u8] = b"WADHU_SMS!";

/// Advertise this gateway on the LAN until `stop` is set.
pub fn spawn(http_port: u16, stop: Arc<AtomicBool>) {
    thread::Builder::new()
        .name("sms-gateway-discover".into())
        .spawn(move || {
            let Ok(sock) = UdpSocket::bind(format!("0.0.0.0:{DISCOVER_PORT}")) else {
                eprintln!("sms-gateway discover: bind :{DISCOVER_PORT} failed");
                return;
            };
            let _ = sock.set_broadcast(true);
            let _ = sock.set_read_timeout(Some(Duration::from_millis(500)));
            let reply = format!(
                r#"WADHU_SMS!{{"name":"Wadhu SMS Gateway","port":{http_port},"pair":true}}"#
            );
            let reply = reply.into_bytes();
            let mut buf = [0u8; 256];
            while !stop.load(Ordering::Relaxed) {
                match sock.recv_from(&mut buf) {
                    Ok((n, peer)) if n >= REQ.len() && &buf[..REQ.len()] == REQ => {
                        let _ = sock.send_to(&reply, peer);
                    }
                    _ => {}
                }
            }
        })
        .ok();
}

/// Client-side: find gateways on the LAN (used by tests / docs; brand crates
/// also implement the same probe in `wadhu-sms-client`).
#[allow(dead_code)]
pub fn probe_once(timeout: Duration) -> Vec<(String, u16)> {
    let Ok(sock) = UdpSocket::bind("0.0.0.0:0") else {
        return Vec::new();
    };
    let _ = sock.set_broadcast(true);
    let _ = sock.set_read_timeout(Some(timeout));
    let _ = sock.send_to(REQ, format!("255.255.255.255:{DISCOVER_PORT}"));
    let mut found = Vec::new();
    let mut buf = [0u8; 512];
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        match sock.recv_from(&mut buf) {
            Ok((n, peer)) if n > RESP_PREFIX.len() && buf.starts_with(RESP_PREFIX) => {
                let body = String::from_utf8_lossy(&buf[RESP_PREFIX.len()..n]);
                let port = json_u16(&body, "port").unwrap_or(18765);
                let host = peer.ip().to_string();
                if !found.iter().any(|(h, p)| h == &host && *p == port) {
                    found.push((host, port));
                }
            }
            _ => break,
        }
    }
    found
}

fn json_u16(body: &str, key: &str) -> Option<u16> {
    let needle = format!("\"{key}\":");
    let i = body.find(&needle)?;
    let rest = body[i + needle.len()..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}
