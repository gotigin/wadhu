//! Find an SMS Gateway on the LAN without typing an IP.
//!
//! Broadcasts `WADHU_SMS?` to UDP `:18767`. The phone replies with its HTTP port.
//! Then call [`crate::Client::pair`] with the code shown under **Allow a new app**.

use std::net::UdpSocket;
use std::time::{Duration, Instant};

use crate::ClientError;

/// UDP discover port (gateway HTTP stays on 18765).
pub const DISCOVER_PORT: u16 = 18767;

const REQ: &[u8] = b"WADHU_SMS?";
const RESP_PREFIX: &[u8] = b"WADHU_SMS!";

/// A gateway found on the local network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveredGateway {
    /// Reachable HTTP base, e.g. `http://192.168.1.40:18765`.
    pub base_url: String,
    /// Advertised name when present.
    pub name: String,
}

/// Probe the LAN for SMS Gateway instances.
pub fn discover(timeout: Duration) -> Result<Vec<DiscoveredGateway>, ClientError> {
    let sock = UdpSocket::bind("0.0.0.0:0").map_err(|e| {
        ClientError::Io(format!("discover bind failed: {e}"))
    })?;
    sock.set_broadcast(true)
        .map_err(|e| ClientError::Io(e.to_string()))?;
    sock.set_read_timeout(Some(Duration::from_millis(200)))
        .map_err(|e| ClientError::Io(e.to_string()))?;

    let targets = [
        format!("255.255.255.255:{DISCOVER_PORT}"),
        format!("127.0.0.1:{DISCOVER_PORT}"),
    ];
    for t in &targets {
        let _ = sock.send_to(REQ, t);
    }

    let mut found = Vec::new();
    let mut buf = [0u8; 512];
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match sock.recv_from(&mut buf) {
            Ok((n, peer)) if n > RESP_PREFIX.len() && buf.starts_with(RESP_PREFIX) => {
                let body = String::from_utf8_lossy(&buf[RESP_PREFIX.len()..n]);
                let port = json_u16(&body, "port").unwrap_or(18765);
                let name = json_str_field(&body, "name").unwrap_or_else(|| "Wadhu SMS Gateway".into());
                let base_url = format!("http://{}:{port}", peer.ip());
                if !found.iter().any(|g: &DiscoveredGateway| g.base_url == base_url) {
                    found.push(DiscoveredGateway { base_url, name });
                }
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                // keep waiting until deadline
            }
            Err(_) => break,
        }
    }
    if found.is_empty() {
        Err(ClientError::Io(
            "No SMS Gateway found on this Wi‑Fi — open SMS Gateway on the phone and stay on the same network"
                .into(),
        ))
    } else {
        Ok(found)
    }
}

/// Discover and return a single gateway (errors if none or ambiguous).
pub fn discover_one(timeout: Duration) -> Result<DiscoveredGateway, ClientError> {
    let mut list = discover(timeout)?;
    if list.len() > 1 {
        return Err(ClientError::Io(format!(
            "Found {} SMS Gateway phones — keep only one on, or choose in the app",
            list.len()
        )));
    }
    Ok(list.remove(0))
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

fn json_str_field(body: &str, key: &str) -> Option<String> {
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
                    out.push(n);
                }
            }
            c => out.push(c),
        }
    }
    None
}
