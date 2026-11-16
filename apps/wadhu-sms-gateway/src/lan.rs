//! Discover this device's LAN IPv4 so operators can hand clients a real URL.

use std::net::{Ipv4Addr, UdpSocket};

/// Best-effort local IPv4 (Wi‑Fi / Ethernet), skipping loopback.
///
/// Uses the kernel's route selection via a UDP "connect" — same approach as
/// other Gotigin Android apps. Returns `None` offline or on weird networks.
pub fn local_ipv4() -> Option<Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("10.255.255.255:1").ok()?;
    match socket.local_addr().ok()?.ip() {
        std::net::IpAddr::V4(ip) if !ip.is_loopback() && !ip.is_unspecified() => Some(ip),
        _ => None,
    }
}

/// HTTP base URL clients should call (`http://ip:port` or placeholder).
pub fn listen_base_url(port: u16) -> String {
    if let Some(ip) = local_ipv4() {
        format!("http://{ip}:{port}")
    } else if cfg!(target_os = "android") {
        format!("http://<wifi-ip>:{port}")
    } else {
        format!("http://127.0.0.1:{port}")
    }
}
