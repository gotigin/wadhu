//! Minimal RFC6455 WebSocket client (text frames) over plain TCP.
//! TLS (wss://) can sit behind a reverse proxy that terminates TLS; the phone
//! dials `ws://` to the relay bind address or `ws://host` behind local proxy.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct WsClient {
    stream: TcpStream,
}

impl WsClient {
    pub fn connect(host: &str, port: u16, path: &str, extra_headers: &[(&str, &str)]) -> Result<Self, String> {
        let mut stream = TcpStream::connect((host, port)).map_err(|e| format!("connect: {e}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(60)))
            .ok();
        stream
            .set_write_timeout(Some(Duration::from_secs(30)))
            .ok();

        let key = make_ws_key();
        let mut req = format!(
            "GET {path} HTTP/1.1\r\nHost: {host}:{port}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n"
        );
        for (k, v) in extra_headers {
            req.push_str(&format!("{k}: {v}\r\n"));
        }
        req.push_str("\r\n");
        stream.write_all(req.as_bytes()).map_err(|e| e.to_string())?;

        let mut buf = Vec::new();
        let mut tmp = [0u8; 1024];
        loop {
            let n = stream.read(&mut tmp).map_err(|e| e.to_string())?;
            if n == 0 {
                return Err("ws handshake closed".into());
            }
            buf.extend_from_slice(&tmp[..n]);
            if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
            if buf.len() > 16 * 1024 {
                return Err("ws handshake too large".into());
            }
        }
        let head = String::from_utf8_lossy(&buf);
        if !head.starts_with("HTTP/1.1 101") && !head.contains(" 101 ") {
            return Err(format!("ws handshake failed: {}", head.lines().next().unwrap_or("")));
        }
        Ok(Self { stream })
    }

    pub fn send_text(&mut self, text: &str) -> Result<(), String> {
        let payload = text.as_bytes();
        let mut frame = Vec::new();
        frame.push(0x81); // FIN + text
        let mask_bit = 0x80u8;
        if payload.len() < 126 {
            frame.push(mask_bit | payload.len() as u8);
        } else if payload.len() <= 65535 {
            frame.push(mask_bit | 126);
            frame.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        } else {
            return Err("ws payload too large".into());
        }
        let mask = [0x12, 0x34, 0x56, 0x78];
        frame.extend_from_slice(&mask);
        for (i, b) in payload.iter().enumerate() {
            frame.push(b ^ mask[i % 4]);
        }
        self.stream.write_all(&frame).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn read_text(&mut self) -> Result<String, String> {
        loop {
            let mut hdr = [0u8; 2];
            self.stream.read_exact(&mut hdr).map_err(|e| e.to_string())?;
            let opcode = hdr[0] & 0x0f;
            let masked = hdr[1] & 0x80 != 0;
            let mut len = (hdr[1] & 0x7f) as usize;
            if len == 126 {
                let mut ext = [0u8; 2];
                self.stream.read_exact(&mut ext).map_err(|e| e.to_string())?;
                len = u16::from_be_bytes(ext) as usize;
            } else if len == 127 {
                let mut ext = [0u8; 8];
                self.stream.read_exact(&mut ext).map_err(|e| e.to_string())?;
                len = u64::from_be_bytes(ext) as usize;
            }
            let mut mask = [0u8; 4];
            if masked {
                self.stream.read_exact(&mut mask).map_err(|e| e.to_string())?;
            }
            let mut payload = vec![0u8; len];
            if len > 0 {
                self.stream
                    .read_exact(&mut payload)
                    .map_err(|e| e.to_string())?;
            }
            if masked {
                for (i, b) in payload.iter_mut().enumerate() {
                    *b ^= mask[i % 4];
                }
            }
            match opcode {
                0x1 => {
                    return String::from_utf8(payload).map_err(|e| e.to_string());
                }
                0x8 => return Err("ws closed".into()),
                0x9 => {
                    // ping → pong
                    let mut frame = Vec::new();
                    frame.push(0x8A);
                    frame.push(0x80 | payload.len() as u8);
                    let m = [1, 2, 3, 4];
                    frame.extend_from_slice(&m);
                    for (i, b) in payload.iter().enumerate() {
                        frame.push(b ^ m[i % 4]);
                    }
                    self.stream.write_all(&frame).map_err(|e| e.to_string())?;
                }
                0xA => {} // pong
                _ => {}
            }
        }
    }
}

fn make_ws_key() -> String {
    // 16 random-ish bytes, base64
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(1);
    let mut raw = [0u8; 16];
    for (i, b) in raw.iter_mut().enumerate() {
        *b = ((t >> (i * 4)) as u8).wrapping_add(i as u8);
    }
    base64_encode(&raw)
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let a = chunk[0] as u32;
        let b = chunk.get(1).copied().unwrap_or(0) as u32;
        let c = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (a << 16) | (b << 8) | c;
        out.push(T[((n >> 18) & 63) as usize] as char);
        out.push(T[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(T[((n >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(T[(n & 63) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}
