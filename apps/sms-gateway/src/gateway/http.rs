//! Minimal local HTTP API so client apps can talk to this gateway.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use super::outbox::Outbox;
use super::radio::SmsRadio;
use super::service::GatewayHandle;

const MAX_BODY: usize = 64 * 1024;

pub fn spawn(handle: GatewayHandle, addr: SocketAddr) -> Result<SocketAddr, String> {
    let listener = TcpListener::bind(addr).map_err(|e| format!("bind {addr}: {e}"))?;
    let bound = listener.local_addr().map_err(|e| e.to_string())?;
    let handle = Arc::new(handle);
    thread::Builder::new()
        .name("sms-gateway-http".into())
        .spawn(move || {
            for stream in listener.incoming().flatten() {
                let h = Arc::clone(&handle);
                let _ = thread::spawn(move || handle_client(stream, &h));
            }
        })
        .map_err(|e| e.to_string())?;
    Ok(bound)
}

fn handle_client(mut stream: TcpStream, handle: &GatewayHandle) {
    let Ok(raw) = read_http_message(&mut stream) else {
        return;
    };
    let Some((method, path, headers, body)) = parse_request(&raw) else {
        let _ = write_response(&mut stream, 400, "application/json", r#"{"error":"bad_request"}"#);
        return;
    };

    // Browser preflight — no auth (Access-Control-Request-* only).
    if method.eq_ignore_ascii_case("OPTIONS") {
        let _ = write_response(&mut stream, 204, "text/plain", "");
        let _ = stream.shutdown(std::net::Shutdown::Both);
        return;
    }

    if !authorize(&headers, &handle.api_token) {
        let _ = write_response(&mut stream, 401, "application/json", r#"{"error":"unauthorized"}"#);
        return;
    }

    let (code, ctype, body) = route(handle, &method, &path, body);
    let _ = write_response(&mut stream, code, ctype, &body);
    let _ = stream.shutdown(std::net::Shutdown::Both);
}

fn authorize(headers: &[(String, String)], token: &str) -> bool {
    for (k, v) in headers {
        let k = k.to_ascii_lowercase();
        if k == "x-api-key" && v == token {
            return true;
        }
        if k == "authorization" {
            let v = v.trim();
            if let Some(rest) = v.strip_prefix("Bearer ") {
                if rest == token {
                    return true;
                }
            }
            if v == token {
                return true;
            }
        }
    }
    false
}

fn route(handle: &GatewayHandle, method: &str, path: &str, body: &str) -> (u16, &'static str, String) {
    match (method, path) {
        ("GET", "/v1/health") => {
            let caps = handle.radio.capabilities();
            let (q, s, f) = handle.outbox.counts();
            let bt = handle.bluetooth.snapshot();
            let relay = handle.relay.snapshot();
            (
                200,
                "application/json",
                format!(
                    r#"{{"ok":true,"app":"wadhu-sms-gateway","radio":{{"kind":"{}","available":{},"detail":{}}},"transports":{{"lan":true,"bluetooth":{{"listening":{},"endpoint":{}}},"relay":{{"configured":{},"connected":{},"detail":{}}}}},"queue":{{"pending":{q},"submitted":{s},"failed":{f}}}}}"#,
                    caps.kind,
                    caps.available,
                    json_str(&caps.detail),
                    bt.listening,
                    json_str(&bt.endpoint),
                    relay.configured,
                    relay.connected,
                    json_str(&relay.detail)
                ),
            )
        }
        ("GET", "/v1/capabilities") => {
            let caps = handle.radio.capabilities();
            (
                200,
                "application/json",
                format!(
                    r#"{{"radio":{{"kind":"{}","label":{},"available":{},"detail":{},"supports_send":{},"supports_multipart":{}}},"transports":["lan","bluetooth","relay"],"api":["/v1/health","/v1/capabilities","/v1/messages"]}}"#,
                    caps.kind,
                    json_str(&caps.label),
                    caps.available,
                    json_str(&caps.detail),
                    caps.supports_send,
                    caps.supports_multipart
                ),
            )
        }
        ("POST", "/v1/messages") => accept_message(&handle.outbox, body),
        ("GET", p) if p.starts_with("/v1/messages/") => {
            let id = &p["/v1/messages/".len()..];
            match handle.outbox.get(id) {
                Some(m) => (200, "application/json", message_json(&m)),
                None => (404, "application/json", r#"{"error":"not_found"}"#.into()),
            }
        }
        ("GET", "/v1/messages") => {
            let list: Vec<String> = handle
                .outbox
                .list(50)
                .iter()
                .map(message_json)
                .collect();
            (
                200,
                "application/json",
                format!(r#"{{"messages":[{}]}}"#, list.join(",")),
            )
        }
        _ => (404, "application/json", r#"{"error":"not_found"}"#.into()),
    }
}

fn accept_message(outbox: &Outbox, body: &str) -> (u16, &'static str, String) {
    let to = json_get(body, "to").unwrap_or_default();
    let text = json_get(body, "body").unwrap_or_default();
    let idem = json_get(body, "idempotency_key").unwrap_or_default();
    match outbox.accept(idem, to, text) {
        Ok(m) => (202, "application/json", message_json(&m)),
        Err(e) => (
            400,
            "application/json",
            format!(r#"{{"error":{}}}"#, json_str(&e)),
        ),
    }
}

fn message_json(m: &super::message::Message) -> String {
    format!(
        r#"{{"id":{},"idempotency_key":{},"to":{},"body":{},"status":{},"error":{},"created_at_ms":{},"updated_at_ms":{}}}"#,
        json_str(&m.id),
        json_str(&m.idempotency_key),
        json_str(&m.to),
        json_str(&m.body),
        json_str(m.status.as_str()),
        m.error
            .as_ref()
            .map(|e| json_str(e))
            .unwrap_or_else(|| "null".into()),
        m.created_at_ms,
        m.updated_at_ms
    )
}

fn read_http_message(stream: &mut TcpStream) -> Result<String, ()> {
    let mut data = Vec::new();
    let mut buf = [0u8; 2048];
    loop {
        let n = stream.read(&mut buf).map_err(|_| ())?;
        if n == 0 {
            break;
        }
        data.extend_from_slice(&buf[..n]);
        if data.len() > MAX_BODY + 4096 {
            return Err(());
        }
        if let Some(i) = find_headers_end(&data) {
            let head = std::str::from_utf8(&data[..i]).map_err(|_| ())?;
            let mut content_len = 0usize;
            for line in head.lines().skip(1) {
                if let Some((k, v)) = line.split_once(':') {
                    if k.eq_ignore_ascii_case("content-length") {
                        content_len = v.trim().parse().unwrap_or(0).min(MAX_BODY);
                    }
                }
            }
            let body_start = i + 4;
            while data.len() < body_start + content_len {
                let n = stream.read(&mut buf).map_err(|_| ())?;
                if n == 0 {
                    break;
                }
                data.extend_from_slice(&buf[..n]);
            }
            break;
        }
    }
    String::from_utf8(data).map_err(|_| ())
}

fn find_headers_end(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|w| w == b"\r\n\r\n")
}

fn parse_request(raw: &str) -> Option<(String, String, Vec<(String, String)>, &str)> {
    let (head, body) = raw.split_once("\r\n\r\n").unwrap_or((raw, ""));
    let mut lines = head.lines();
    let req = lines.next()?;
    let mut parts = req.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.split('?').next()?.to_string();
    let mut headers = Vec::new();
    let mut content_len = 0usize;
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim().to_string();
            let v = v.trim().to_string();
            if k.eq_ignore_ascii_case("content-length") {
                content_len = v.parse().unwrap_or(0).min(MAX_BODY);
            }
            headers.push((k, v));
        }
    }
    let body = if content_len == 0 {
        body
    } else {
        &body[..body.len().min(content_len)]
    };
    Some((method, path, headers, body))
}

fn cors_headers() -> &'static str {
    "Access-Control-Allow-Origin: *\r\n\
     Access-Control-Allow-Headers: Authorization, Content-Type, X-Api-Key\r\n\
     Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n"
}

fn write_response(stream: &mut TcpStream, code: u16, ctype: &str, body: &str) -> std::io::Result<()> {
    let reason = match code {
        200 => "OK",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        _ => "Error",
    };
    // Buffer the full response then write_all — partial `write!` to a socket
    // can leave clients hanging on incomplete HTTP heads.
    let packet = format!(
        "HTTP/1.1 {code} {reason}\r\n{cors}Content-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len(),
        cors = cors_headers(),
    );
    stream.write_all(packet.as_bytes())
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

/// Tiny JSON string field extractor (enough for our flat POST body).
fn json_get(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let i = body.find(&needle)?;
    let after = &body[i + needle.len()..];
    let after = after.trim_start();
    let after = after.strip_prefix(':')?.trim_start();
    if let Some(rest) = after.strip_prefix('"') {
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
    } else {
        None
    }
}

#[allow(dead_code)]
fn _radio_type_use(_: &dyn SmsRadio) {}
