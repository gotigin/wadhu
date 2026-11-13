//! Sync HTTP/1.1 over `TcpStream` (desktop + Android/iOS native).

use std::io::{Read, Write};
use std::net::TcpStream;

use crate::{Client, ClientError};

pub(crate) fn request(
    client: &Client,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<String, ClientError> {
    let (host, port) = parse_http_base(&client.base)?;
    let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|e| {
        ClientError::Io(e.to_string())
    })?;
    stream.set_read_timeout(Some(client.timeout)).ok();
    stream.set_write_timeout(Some(client.timeout)).ok();
    let token = &client.token;
    let req = if let Some(body) = body {
        format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}:{port}\r\nAuthorization: Bearer {token}\r\nX-Api-Key: {token}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    } else {
        format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}:{port}\r\nAuthorization: Bearer {token}\r\nX-Api-Key: {token}\r\nConnection: close\r\n\r\n"
        )
    };
    stream
        .write_all(req.as_bytes())
        .map_err(|e| ClientError::Io(e.to_string()))?;
    let mut out = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }
    let text = String::from_utf8_lossy(&out);
    let (head, body) = text.split_once("\r\n\r\n").unwrap_or((&text, ""));
    let status = head
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    if !(200..300).contains(&status) {
        return Err(ClientError::Api {
            status,
            body: body.to_string(),
        });
    }
    Ok(body.to_string())
}

pub(crate) fn parse_http_base(base: &str) -> Result<(String, u16), ClientError> {
    let u = base.strip_prefix("http://").ok_or_else(|| {
        ClientError::BadUrl(
            "native client supports http:// only (use wasm async fetch for https://)".into(),
        )
    })?;
    let (hostport, _) = u.split_once('/').unwrap_or((u, ""));
    if let Some((h, p)) = hostport.split_once(':') {
        let port: u16 = p
            .parse()
            .map_err(|_| ClientError::BadUrl(format!("bad port in {base}")))?;
        Ok((h.to_string(), port))
    } else {
        Ok((hostport.to_string(), 80))
    }
}
