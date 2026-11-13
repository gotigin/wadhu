//! HTTP API + device WebSocket for sms-relay.

use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::hub::Hub;
use crate::store::Store;

const MAX_BODY: usize = 64 * 1024;

#[derive(Clone)]
pub struct App {
    pub store: Store,
    pub hub: Hub,
    pub admin_token: String,
}

pub fn serve(app: App, addr: SocketAddr) -> Result<(), String> {
    let listener = TcpListener::bind(addr).map_err(|e| format!("bind {addr}: {e}"))?;
    eprintln!("wadhu-sms-relay listening on http://{addr}");
    let app = Arc::new(app);
    for stream in listener.incoming().flatten() {
        let app = Arc::clone(&app);
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream, &app) {
                eprintln!("conn error: {e}");
            }
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, app: &App) -> Result<(), String> {
    let raw = read_http_head_and_body(&mut stream)?;
    let (method, path, headers, body) = parse_request(&raw).ok_or("bad_request")?;

    if path.starts_with("/v1/device/ws") {
        return handle_device_ws(stream, app, &path, &headers);
    }

    let (code, ctype, resp) = route(app, &method, &path, &headers, body);
    write_response(&mut stream, code, ctype, &resp)?;
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}

fn route(
    app: &App,
    method: &str,
    path: &str,
    headers: &[(String, String)],
    body: &str,
) -> (u16, &'static str, String) {
    match (method, path) {
        ("GET", "/v1/health") => {
            let online = app.hub.online();
            let devices = app.store.list_devices().len();
            (
                200,
                "application/json",
                format!(
                    r#"{{"ok":true,"app":"wadhu-sms-relay","devices":{devices},"online":{},"online_ids":[{}]}}"#,
                    online.len(),
                    online
                        .iter()
                        .map(|id| j(id))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            )
        }
        ("GET", "/v1/capabilities") => (
            200,
            "application/json",
            r#"{"role":"relay","transports":["remote"],"api":["/v1/health","/v1/capabilities","/v1/messages","/v1/devices","/v1/devices/pair","/v1/keys"],"device_ws":"/v1/device/ws"}"#.into(),
        ),
        ("POST", "/v1/devices/pair") => {
            if !admin_ok(headers, &app.admin_token) {
                return unauthorized();
            }
            let (code, device_id, device_secret) = app.store.create_pairing();
            // Pre-register so the phone can connect immediately with these credentials.
            app.store
                .ensure_device(&device_id, &device_secret, "pending pair");
            (
                200,
                "application/json",
                format!(
                    r#"{{"pairing_code":{},"device_id":{},"device_secret":{},"expires_in_sec":600,"hint":"Enter relay URL + device_id + device_secret in sms-gateway. pairing_code is optional redeem path."}}"#,
                    j(&code),
                    j(&device_id),
                    j(&device_secret)
                ),
            )
        }
        ("POST", "/v1/devices/redeem") => {
            // Optional: redeem short code → same credentials (already issued by pair).
            if !admin_ok(headers, &app.admin_token) {
                return unauthorized();
            }
            let code = json_get(body, "code").unwrap_or_default();
            let label = json_get(body, "label").unwrap_or_default();
            match app.store.redeem_pairing(&code, &label) {
                Ok((id, secret)) => (
                    200,
                    "application/json",
                    format!(
                        r#"{{"device_id":{},"device_secret":{}}}"#,
                        j(&id),
                        j(&secret)
                    ),
                ),
                Err(e) => (400, "application/json", format!(r#"{{"error":{}}}"#, j(&e))),
            }
        }
        ("GET", "/v1/devices") => {
            if !admin_ok(headers, &app.admin_token) {
                return unauthorized();
            }
            let list: Vec<String> = app
                .store
                .list_devices()
                .into_iter()
                .map(|d| {
                    format!(
                        r#"{{"id":{},"label":{},"online":{},"last_seen_ms":{}}}"#,
                        j(&d.id),
                        j(&d.label),
                        app.hub.is_online(&d.id),
                        d.last_seen_ms
                    )
                })
                .collect();
            (
                200,
                "application/json",
                format!(r#"{{"devices":[{}]}}"#, list.join(",")),
            )
        }
        ("POST", "/v1/keys") => {
            if !admin_ok(headers, &app.admin_token) {
                return unauthorized();
            }
            let label = json_get(body, "label").unwrap_or_default();
            let device_id = json_get(body, "device_id");
            let key = app.store.create_client_key(&label, device_id);
            (
                200,
                "application/json",
                format!(
                    r#"{{"id":{},"token":{},"label":{},"device_id":{}}}"#,
                    j(&key.id),
                    j(&key.token),
                    j(&key.label),
                    key.device_id
                        .as_ref()
                        .map(|d| j(d))
                        .unwrap_or_else(|| "null".into())
                ),
            )
        }
        ("POST", "/v1/messages") => {
            let Some(client) = client_ok(headers, app) else {
                return unauthorized();
            };
            let to = json_get(body, "to").unwrap_or_default();
            let text = json_get(body, "body").unwrap_or_default();
            let idem = json_get(body, "idempotency_key").unwrap_or_default();
            let preferred = client.device_id.clone();
            let Some(device_id) = app.store.pick_device(preferred.as_deref()) else {
                return (
                    503,
                    "application/json",
                    r#"{"error":"no_devices_registered"}"#.into(),
                );
            };
            match app
                .store
                .accept_message(idem, to, text, Some(device_id.clone()))
            {
                Ok(m) => {
                    let job = format!(
                        r#"{{"type":"job","id":{},"to":{},"body":{},"idempotency_key":{}}}"#,
                        j(&m.id),
                        j(&m.to),
                        j(&m.body),
                        j(&m.idempotency_key)
                    );
                    match app.hub.send_job(&device_id, job) {
                        Ok(()) => {
                            app.store.set_status(&m.id, "submitting", None);
                            let m = app.store.get_message(&m.id).unwrap_or(m);
                            (202, "application/json", message_json(&m))
                        }
                        Err(e) => {
                            app.store.set_status(&m.id, "queued", Some(e.clone()));
                            let m = app.store.get_message(&m.id).unwrap_or(m);
                            // Still 202 — device may connect later; client polls status.
                            let _ = e;
                            (202, "application/json", message_json(&m))
                        }
                    }
                }
                Err(e) => (400, "application/json", format!(r#"{{"error":{}}}"#, j(&e))),
            }
        }
        ("GET", "/v1/messages") => {
            if client_ok(headers, app).is_none() && !admin_ok(headers, &app.admin_token) {
                return unauthorized();
            }
            let list: Vec<String> = app
                .store
                .list_messages(50)
                .iter()
                .map(message_json)
                .collect();
            (
                200,
                "application/json",
                format!(r#"{{"messages":[{}]}}"#, list.join(",")),
            )
        }
        ("GET", p) if p.starts_with("/v1/messages/") => {
            if client_ok(headers, app).is_none() && !admin_ok(headers, &app.admin_token) {
                return unauthorized();
            }
            let id = &p["/v1/messages/".len()..];
            match app.store.get_message(id) {
                Some(m) => (200, "application/json", message_json(&m)),
                None => (404, "application/json", r#"{"error":"not_found"}"#.into()),
            }
        }
        _ => (404, "application/json", r#"{"error":"not_found"}"#.into()),
    }
}

fn handle_device_ws(
    mut stream: TcpStream,
    app: &App,
    path: &str,
    headers: &[(String, String)],
) -> Result<(), String> {
    let device_id = query_param(path, "device_id").ok_or("missing device_id")?;
    let secret = bearer_token(headers).ok_or("missing bearer")?;
    if !app.store.auth_device(&device_id, &secret) {
        write_response(
            &mut stream,
            401,
            "application/json",
            r#"{"error":"unauthorized"}"#,
        )?;
        return Ok(());
    }

    // WebSocket upgrade
    let key = header(headers, "sec-websocket-key").ok_or("missing ws key")?;
    let accept = ws_accept_key(&key);
    let up = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\n\r\n"
    );
    stream.write_all(up.as_bytes()).map_err(|e| e.to_string())?;

    let rx = app.hub.register(device_id.clone());
    app.store.touch_device(&device_id);
    let _ = ws_send_text(&mut stream, r#"{"type":"hello_ok"}"#);

    stream
        .set_read_timeout(Some(Duration::from_millis(500)))
        .ok();

    loop {
        // Push queued jobs from hub
        while let Ok(job) = rx.try_recv() {
            if ws_send_text(&mut stream, &job).is_err() {
                app.hub.unregister(&device_id);
                return Ok(());
            }
        }
        match ws_read_text(&mut stream) {
            Ok(msg) => {
                app.store.touch_device(&device_id);
                handle_device_message(app, &device_id, &msg);
            }
            Err(e) if e == "timeout" || e == "would_block" => continue,
            Err(_) => break,
        }
    }
    app.hub.unregister(&device_id);
    Ok(())
}

fn handle_device_message(app: &App, _device_id: &str, raw: &str) {
    let ty = json_get(raw, "type").unwrap_or_default();
    if ty == "status" {
        let id = json_get(raw, "id").unwrap_or_default();
        let status = json_get(raw, "status").unwrap_or_default();
        let error = json_get(raw, "error");
        if !id.is_empty() && !status.is_empty() {
            app.store.set_status(&id, &status, error);
        }
    }
}

fn message_json(m: &crate::store::Message) -> String {
    format!(
        r#"{{"id":{},"idempotency_key":{},"to":{},"body":{},"status":{},"error":{},"device_id":{},"created_at_ms":{},"updated_at_ms":{}}}"#,
        j(&m.id),
        j(&m.idempotency_key),
        j(&m.to),
        j(&m.body),
        j(&m.status),
        m.error.as_ref().map(|e| j(e)).unwrap_or_else(|| "null".into()),
        m.device_id
            .as_ref()
            .map(|d| j(d))
            .unwrap_or_else(|| "null".into()),
        m.created_at_ms,
        m.updated_at_ms
    )
}

fn unauthorized() -> (u16, &'static str, String) {
    (401, "application/json", r#"{"error":"unauthorized"}"#.into())
}

fn admin_ok(headers: &[(String, String)], admin: &str) -> bool {
    if admin.is_empty() {
        return false;
    }
    bearer_token(headers).as_deref() == Some(admin)
        || header(headers, "x-api-key").as_deref() == Some(admin)
}

fn client_ok(headers: &[(String, String)], app: &App) -> Option<crate::store::ClientKey> {
    let token = bearer_token(headers).or_else(|| header(headers, "x-api-key"))?;
    app.store.auth_client(&token)
}

fn bearer_token(headers: &[(String, String)]) -> Option<String> {
    let v = header(headers, "authorization")?;
    v.strip_prefix("Bearer ").map(|s| s.to_string()).or(Some(v))
}

fn header(headers: &[(String, String)], name: &str) -> Option<String> {
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.clone())
}

fn query_param(path: &str, key: &str) -> Option<String> {
    let q = path.split_once('?')?.1;
    for part in q.split('&') {
        if let Some((k, v)) = part.split_once('=') {
            if k == key {
                return Some(url_decode(v));
            }
        }
    }
    None
}

fn url_decode(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            let hex = format!("{h1}{h2}");
            if let Ok(b) = u8::from_str_radix(&hex, 16) {
                out.push(b as char);
            }
        } else if c == '+' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

fn write_response(
    stream: &mut TcpStream,
    code: u16,
    ctype: &str,
    body: &str,
) -> Result<(), String> {
    let reason = match code {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        503 => "Service Unavailable",
        _ => "Error",
    };
    let packet = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream
        .write_all(packet.as_bytes())
        .map_err(|e| e.to_string())
}

fn read_http_head_and_body(stream: &mut TcpStream) -> Result<String, String> {
    let mut data = Vec::new();
    let mut buf = [0u8; 2048];
    stream
        .set_read_timeout(Some(Duration::from_secs(15)))
        .ok();
    loop {
        let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        data.extend_from_slice(&buf[..n]);
        if data.len() > MAX_BODY + 4096 {
            return Err("too_large".into());
        }
        if let Some(i) = data.windows(4).position(|w| w == b"\r\n\r\n") {
            let head = std::str::from_utf8(&data[..i]).map_err(|e| e.to_string())?;
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
                let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
                if n == 0 {
                    break;
                }
                data.extend_from_slice(&buf[..n]);
            }
            break;
        }
    }
    String::from_utf8(data).map_err(|e| e.to_string())
}

fn parse_request(raw: &str) -> Option<(String, String, Vec<(String, String)>, &str)> {
    let (head, body) = raw.split_once("\r\n\r\n").unwrap_or((raw, ""));
    let mut lines = head.lines();
    let req = lines.next()?;
    let mut parts = req.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();
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

fn ws_accept_key(key: &str) -> String {
    // RFC6455: base64( SHA1( key + magic ) )
    let mut data = key.as_bytes().to_vec();
    data.extend_from_slice(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let hash = sha1(&data);
    base64_encode(&hash)
}

fn sha1(message: &[u8]) -> [u8; 20] {
    // Compact SHA-1 (public domain style)
    fn rotl(x: u32, n: u32) -> u32 {
        x.rotate_left(n)
    }
    let mut h0: u32 = 0x6745_2301;
    let mut h1: u32 = 0xEFCD_AB89;
    let mut h2: u32 = 0x98BA_DCFE;
    let mut h3: u32 = 0x1032_5476;
    let mut h4: u32 = 0xC3D2_E1F0;
    let bit_len = (message.len() as u64) * 8;
    let mut msg = message.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = rotl(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A82_7999),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };
            let temp = rotl(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = rotl(b, 30);
            b = a;
            a = temp;
        }
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }
    let mut out = [0u8; 20];
    out[0..4].copy_from_slice(&h0.to_be_bytes());
    out[4..8].copy_from_slice(&h1.to_be_bytes());
    out[8..12].copy_from_slice(&h2.to_be_bytes());
    out[12..16].copy_from_slice(&h3.to_be_bytes());
    out[16..20].copy_from_slice(&h4.to_be_bytes());
    out
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

fn ws_send_text(stream: &mut TcpStream, text: &str) -> Result<(), String> {
    let payload = text.as_bytes();
    let mut frame = Vec::new();
    frame.push(0x81);
    if payload.len() < 126 {
        frame.push(payload.len() as u8);
    } else if payload.len() <= 65535 {
        frame.push(126);
        frame.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    } else {
        return Err("too_large".into());
    }
    frame.extend_from_slice(payload);
    stream.write_all(&frame).map_err(|e| e.to_string())
}

fn ws_read_text(stream: &mut TcpStream) -> Result<String, String> {
    let mut hdr = [0u8; 2];
    match stream.read_exact(&mut hdr) {
        Ok(()) => {}
        Err(e)
            if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::TimedOut =>
        {
            return Err("timeout".into());
        }
        Err(e) => return Err(e.to_string()),
    }
    let opcode = hdr[0] & 0x0f;
    let masked = hdr[1] & 0x80 != 0;
    let mut len = (hdr[1] & 0x7f) as usize;
    if len == 126 {
        let mut ext = [0u8; 2];
        stream.read_exact(&mut ext).map_err(|e| e.to_string())?;
        len = u16::from_be_bytes(ext) as usize;
    } else if len == 127 {
        let mut ext = [0u8; 8];
        stream.read_exact(&mut ext).map_err(|e| e.to_string())?;
        len = u64::from_be_bytes(ext) as usize;
    }
    let mut mask = [0u8; 4];
    if masked {
        stream.read_exact(&mut mask).map_err(|e| e.to_string())?;
    }
    let mut payload = vec![0u8; len];
    if len > 0 {
        stream
            .read_exact(&mut payload)
            .map_err(|e| e.to_string())?;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }
    match opcode {
        0x1 => String::from_utf8(payload).map_err(|e| e.to_string()),
        0x8 => Err("closed".into()),
        0x9 => {
            // ping → pong
            let mut frame = vec![0x8A, payload.len() as u8];
            frame.extend_from_slice(&payload);
            stream.write_all(&frame).map_err(|e| e.to_string())?;
            Err("timeout".into())
        }
        _ => Err("timeout".into()),
    }
}

fn j(s: &str) -> String {
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
