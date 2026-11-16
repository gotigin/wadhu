//! HTTP API + static ES landing + Meta webhooks.

use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::thread;

use hmac::{Hmac, Mac};
use serde_json::{Value, json};
use sha2::Sha256;

use crate::config::Config;
use crate::meta::MetaClient;
use crate::privacy_policy::{
    self, ABOUT_MD, DATA_DELETION_MD, PRIVACY_MD, SUPPORT_MD, TERMS_MD,
};
use crate::store::Store;

type HmacSha256 = Hmac<Sha256>;

const MAX_BODY: usize = 2 * 1024 * 1024;

pub struct App {
    pub cfg: Config,
    pub store: Store,
    pub meta: MetaClient,
}

pub fn serve(app: App, addr: SocketAddr) -> Result<(), String> {
    let listener = TcpListener::bind(addr).map_err(|e| format!("bind {addr}: {e}"))?;
    eprintln!("wadhu-connect listening on http://{addr}");
    eprintln!("public origin: {}", app.cfg.public_origin);
    eprintln!(
        "config ready: {} (missing: {:?})",
        app.cfg.ready(),
        app.cfg.missing()
    );
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
    let raw = read_http(&mut stream)?;
    let (method, path, query, headers, body) = parse_request(&raw).ok_or("bad_request")?;

    if method.eq_ignore_ascii_case("OPTIONS") {
        write_response(&mut stream, 204, "text/plain", b"", cors())?;
        let _ = stream.shutdown(Shutdown::Both);
        return Ok(());
    }

    let (code, ctype, resp) = route(app, &method, &path, &query, &headers, &body);
    write_response(&mut stream, code, ctype, &resp, cors())?;
    let _ = stream.shutdown(Shutdown::Both);
    Ok(())
}

fn cors() -> Vec<(&'static str, String)> {
    vec![
        ("Access-Control-Allow-Origin", "*".into()),
        (
            "Access-Control-Allow-Headers",
            "Content-Type, X-Hub-Signature-256, Authorization, X-Api-Key".into(),
        ),
        (
            "Access-Control-Allow-Methods",
            "GET, POST, OPTIONS".into(),
        ),
    ]
}

fn route(
    app: &App,
    method: &str,
    path: &str,
    query: &str,
    headers: &[(String, String)],
    body: &[u8],
) -> (u16, &'static str, Vec<u8>) {
    if path.starts_with("/api/") || path == "/v1/messages" {
        return api(app, method, path, body);
    }
    if path == "/webhook" {
        return webhook(app, method, query, headers, body);
    }
    if path == "/v1/health" {
        return json_ok(json!({
            "ok": true,
            "app": "wadhu-connect",
            "public_origin": app.cfg.public_origin,
            "ready": app.cfg.ready(),
        }));
    }
    if path == "/v1/capabilities" {
        return json_ok(json!({
            "role": "wadhu-connect",
            "product": "WhatsApp Cloud API bridge",
            "api": [
                "/v1/health",
                "/v1/capabilities",
                "/v1/messages",
                "/api/config",
                "/api/status",
                "/api/complete",
                "/api/send-test",
                "/webhook"
            ],
            "privacy": "/privacy",
            "terms": "/terms",
            "data_deletion": "/data-deletion",
            "support": "/support",
            "about": "/about",
            "docs": "https://wadhu.com",
            "integrate": "https://wadhu.com — see apps/wadhu-connect/docs/INTEGRATOR.md"
        }));
    }
    // Same docs/legal/*.md the site presents — formatted HTML, not a dump.
    if method.eq_ignore_ascii_case("GET") {
        match path {
            "/privacy" | "/privacy/" => {
                return (
                    200,
                    "text/html; charset=utf-8",
                    privacy_policy::privacy_html().into_bytes(),
                );
            }
            "/terms" | "/terms/" => {
                return (
                    200,
                    "text/html; charset=utf-8",
                    privacy_policy::terms_html().into_bytes(),
                );
            }
            "/data-deletion" | "/data-deletion/" => {
                return (
                    200,
                    "text/html; charset=utf-8",
                    privacy_policy::data_deletion_html().into_bytes(),
                );
            }
            "/support" | "/support/" => {
                return (
                    200,
                    "text/html; charset=utf-8",
                    privacy_policy::support_html().into_bytes(),
                );
            }
            "/about" | "/about/" => {
                return (
                    200,
                    "text/html; charset=utf-8",
                    privacy_policy::about_html().into_bytes(),
                );
            }
            "/privacy.md" => {
                return (
                    200,
                    "text/markdown; charset=utf-8",
                    PRIVACY_MD.as_bytes().to_vec(),
                );
            }
            "/terms.md" => {
                return (
                    200,
                    "text/markdown; charset=utf-8",
                    TERMS_MD.as_bytes().to_vec(),
                );
            }
            "/data-deletion.md" => {
                return (
                    200,
                    "text/markdown; charset=utf-8",
                    DATA_DELETION_MD.as_bytes().to_vec(),
                );
            }
            "/support.md" => {
                return (
                    200,
                    "text/markdown; charset=utf-8",
                    SUPPORT_MD.as_bytes().to_vec(),
                );
            }
            "/about.md" => {
                return (
                    200,
                    "text/markdown; charset=utf-8",
                    ABOUT_MD.as_bytes().to_vec(),
                );
            }
            _ => {}
        }
    }
    static_file(&app.cfg.public_dir, path)
}

fn api(app: &App, method: &str, path: &str, body: &[u8]) -> (u16, &'static str, Vec<u8>) {
    match (method, path) {
        ("GET", "/api/config") => json_ok(public_config(app)),
        ("GET", "/api/status") => {
            let conn = app.store.read_connection();
            let token = conn
                .as_ref()
                .and_then(|c| c.get("businessToken"))
                .and_then(|v| v.as_str());
            let phone = conn
                .as_ref()
                .and_then(|c| c.get("phoneNumberId"))
                .and_then(|v| v.as_str());
            if token.is_none() || phone.is_none() {
                return json_ok(json!({
                    "connected": false,
                    "reason": "No Cloud API connection stored yet",
                    "connection": public_connection(app),
                    "recentWebhooks": app.store.recent_webhooks(10),
                }));
            }
            let status = app.meta.phone_status(phone.unwrap(), token.unwrap());
            if status.ok {
                let mut patch = json!({});
                if let Some(o) = patch.as_object_mut() {
                    if let Some(v) = status.json.get("platform_type") {
                        o.insert("platformType".into(), v.clone());
                    }
                    if let Some(v) = status.json.get("is_on_biz_app") {
                        o.insert("isOnBizApp".into(), v.clone());
                    }
                    if let Some(v) = status.json.get("display_phone_number") {
                        o.insert("displayPhoneNumber".into(), v.clone());
                    }
                    if let Some(v) = status.json.get("verified_name") {
                        o.insert("verifiedName".into(), v.clone());
                    }
                    if let Some(v) = status.json.get("quality_rating") {
                        o.insert("qualityRating".into(), v.clone());
                    }
                    if let Some(v) = status.json.get("status") {
                        o.insert("phoneStatus".into(), v.clone());
                    }
                }
                app.store.write_connection(patch);
            }
            json_ok(json!({
                "connected": status.ok,
                "graph": { "ok": status.ok, "status": status.status, "json": status.json },
                "connection": public_connection(app),
                "recentWebhooks": app.store.recent_webhooks(10),
            }))
        }
        ("POST", "/api/session") => {
            let v: Value = serde_json::from_slice(body).unwrap_or_else(|_| json!({}));
            app.store.log_session(v.clone());
            if let Some(data) = v.get("data") {
                let mut patch = json!({ "status": v.get("event").cloned().unwrap_or(json!("session")) });
                if let Some(o) = patch.as_object_mut() {
                    if let Some(id) = data.get("phone_number_id") {
                        o.insert("phoneNumberId".into(), id.clone());
                    }
                    if let Some(id) = data.get("waba_id") {
                        o.insert("wabaId".into(), id.clone());
                    }
                    if let Some(id) = data.get("business_id") {
                        o.insert("businessId".into(), id.clone());
                    }
                    if let Some(ev) = v.get("event") {
                        o.insert("finishEvent".into(), ev.clone());
                    }
                }
                app.store.write_connection(patch);
            }
            json_ok(json!({ "ok": true }))
        }
        ("POST", "/api/complete") => complete_onboarding(app, body),
        ("POST", "/api/send-test") | ("POST", "/v1/messages") | ("POST", "/api/messages") => {
            send_test(app, body)
        }
        _ => json_err(404, "not_found"),
    }
}

fn complete_onboarding(app: &App, body: &[u8]) -> (u16, &'static str, Vec<u8>) {
    if !app.cfg.ready() {
        return json_status(
            400,
            json!({
                "ok": false,
                "error": "META_APP_ID / META_APP_SECRET / META_CONFIG_ID not configured",
                "missing": app.cfg.missing(),
            }),
        );
    }
    let v: Value = serde_json::from_slice(body).unwrap_or_else(|_| json!({}));
    let code = v.get("code").and_then(|c| c.as_str()).unwrap_or("");
    if code.is_empty() {
        return json_status(400, json!({ "ok": false, "error": "Missing authorization code" }));
    }
    let phone_number_id = v
        .get("phoneNumberId")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string());
    let waba_id = v
        .get("wabaId")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string());
    let business_id = v
        .get("businessId")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string());
    let finish_event = v.get("finishEvent").cloned();

    let exchanged = app.meta.exchange_code(code, None);
    if !exchanged.ok {
        app.store.write_connection(json!({
            "status": "token_exchange_failed",
            "lastError": exchanged.json,
        }));
        return json_status(
            502,
            json!({
                "ok": false,
                "step": "token_exchange",
                "graph": { "ok": false, "status": exchanged.status, "json": exchanged.json },
            }),
        );
    }
    let business_token = exchanged
        .json
        .get("access_token")
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string();
    if business_token.is_empty() {
        return json_status(502, json!({ "ok": false, "error": "No access_token in exchange" }));
    }

    let mut patch = json!({
        "status": "token_exchanged",
        "businessToken": business_token,
        "tokenExpiresIn": exchanged.json.get("expires_in"),
    });
    if let Some(o) = patch.as_object_mut() {
        if let Some(p) = &phone_number_id {
            o.insert("phoneNumberId".into(), json!(p));
        }
        if let Some(w) = &waba_id {
            o.insert("wabaId".into(), json!(w));
        }
        if let Some(b) = &business_id {
            o.insert("businessId".into(), json!(b));
        }
        if let Some(ev) = finish_event {
            o.insert("finishEvent".into(), ev);
        }
    }
    let conn = app.store.write_connection(patch);

    let mut results = json!({
        "tokenExchange": { "ok": true, "expiresIn": exchanged.json.get("expires_in") }
    });

    if let Some(waba) = conn.get("wabaId").and_then(|x| x.as_str()) {
        let sub = app.meta.subscribe_app(waba, &business_token);
        results.as_object_mut().unwrap().insert(
            "subscribe".into(),
            json!({ "ok": sub.ok, "status": sub.status, "json": sub.json }),
        );
    }

    if let Some(phone) = conn.get("phoneNumberId").and_then(|x| x.as_str()) {
        let pin = format!("{}", 100_000 + (now_ns() % 900_000));
        let reg = app.meta.register_phone(phone, &business_token, &pin);
        let status = app.meta.phone_status(phone, &business_token);
        if status.ok {
            app.store.write_connection(json!({
                "status": "cloud_api_connected",
                "platformType": status.json.get("platform_type"),
                "isOnBizApp": status.json.get("is_on_biz_app"),
                "displayPhoneNumber": status.json.get("display_phone_number"),
                "verifiedName": status.json.get("verified_name"),
            }));
        }
        let hist = app.meta.request_history_sync(phone, &business_token);
        let o = results.as_object_mut().unwrap();
        o.insert(
            "register".into(),
            json!({ "ok": reg.ok, "status": reg.status, "json": reg.json }),
        );
        o.insert("registerPinUsedOnce".into(), json!(pin));
        o.insert(
            "phoneStatus".into(),
            json!({ "ok": status.ok, "status": status.status, "json": status.json }),
        );
        o.insert(
            "historySync".into(),
            json!({ "ok": hist.ok, "status": hist.status, "json": hist.json }),
        );
    }

    json_ok(json!({
        "ok": true,
        "connection": public_connection(app),
        "results": results,
    }))
}

fn send_test(app: &App, body: &[u8]) -> (u16, &'static str, Vec<u8>) {
    let conn = match app.store.read_connection() {
        Some(c) => c,
        None => return json_status(400, json!({ "ok": false, "error": "Not connected" })),
    };
    let token = match conn.get("businessToken").and_then(|t| t.as_str()) {
        Some(t) => t,
        None => return json_status(400, json!({ "ok": false, "error": "Not connected" })),
    };
    let phone = match conn.get("phoneNumberId").and_then(|t| t.as_str()) {
        Some(t) => t,
        None => return json_status(400, json!({ "ok": false, "error": "Not connected" })),
    };
    let v: Value = serde_json::from_slice(body).unwrap_or_else(|_| json!({}));
    let to_raw = v.get("to").and_then(|t| t.as_str()).unwrap_or("");
    let to: String = to_raw.chars().filter(|c| c.is_ascii_digit()).collect();
    if to.is_empty() {
        return json_status(
            400,
            json!({ "ok": false, "error": "Provide { \"to\": \"<E.164 digits without +>\" }" }),
        );
    }
    let text = v
        .get("text")
        .and_then(|t| t.as_str())
        .unwrap_or("Wadhu Connect Cloud API test")
        .to_string();
    let sent = app.meta.send_text(phone, token, &to, &text);
    json_status(
        if sent.ok { 200 } else { 502 },
        json!({ "ok": sent.ok, "graph": { "ok": sent.ok, "status": sent.status, "json": sent.json } }),
    )
}

fn webhook(
    app: &App,
    method: &str,
    query: &str,
    headers: &[(String, String)],
    body: &[u8],
) -> (u16, &'static str, Vec<u8>) {
    if method.eq_ignore_ascii_case("GET") {
        let mode = q(query, "hub.mode");
        let token = q(query, "hub.verify_token");
        let challenge = q(query, "hub.challenge");
        if mode.as_deref() == Some("subscribe")
            && token.as_deref() == Some(app.cfg.verify_token.as_str())
        {
            return (
                200,
                "text/plain",
                challenge.unwrap_or_default().into_bytes(),
            );
        }
        return (403, "text/plain", b"Forbidden".to_vec());
    }
    if method.eq_ignore_ascii_case("POST") {
        let sig = header(headers, "x-hub-signature-256");
        let verified = verify_signature(&app.cfg.app_secret, body, sig.as_deref());
        let payload: Value = serde_json::from_slice(body).unwrap_or_else(|_| {
            json!({
                "parseError": true,
                "raw": String::from_utf8_lossy(body).chars().take(500).collect::<String>(),
            })
        });
        app.store.log_webhook(json!({
            "verified": verified.0,
            "verifyReason": verified.1,
            "payload": payload,
        }));
        return json_ok(json!({ "success": true }));
    }
    (405, "text/plain", b"Method Not Allowed".to_vec())
}

fn verify_signature(secret: &str, body: &[u8], header: Option<&str>) -> (bool, String) {
    if secret.is_empty() {
        return (false, "META_APP_SECRET not set".into());
    }
    let Some(h) = header else {
        return (false, "missing X-Hub-Signature-256".into());
    };
    let Some(hex_sig) = h.strip_prefix("sha256=") else {
        return (false, "missing X-Hub-Signature-256".into());
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return (false, "bad secret".into());
    };
    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());
    if expected.eq_ignore_ascii_case(hex_sig) {
        (true, "ok".into())
    } else {
        (false, "signature mismatch".into())
    }
}

fn public_config(app: &App) -> Value {
    json!({
        "ready": app.cfg.ready(),
        "missing": app.cfg.missing(),
        "appId": if app.cfg.app_id.is_empty() { Value::Null } else { json!(app.cfg.app_id) },
        "configId": if app.cfg.config_id.is_empty() { Value::Null } else { json!(app.cfg.config_id) },
        "graphVersion": app.cfg.graph_version,
        "publicOrigin": app.cfg.public_origin,
        "webhookPath": "/webhook",
        "coexistence": {
            "featureType": "whatsapp_business_app_onboarding",
            "sessionInfoVersion": "3",
        },
        "connection": public_connection(app),
    })
}

fn public_connection(app: &App) -> Value {
    let Some(c) = app.store.read_connection() else {
        return Value::Null;
    };
    json!({
        "status": c.get("status"),
        "wabaId": c.get("wabaId"),
        "phoneNumberId": c.get("phoneNumberId"),
        "businessId": c.get("businessId"),
        "platformType": c.get("platformType"),
        "isOnBizApp": c.get("isOnBizApp"),
        "finishEvent": c.get("finishEvent"),
        "updatedAt": c.get("updatedAt"),
        "hasBusinessToken": c.get("businessToken").and_then(|t| t.as_str()).map(|s| !s.is_empty()).unwrap_or(false),
        // Never expose the business phone on the public status card.
        "hasDisplayPhone": c
            .get("displayPhoneNumber")
            .and_then(|v| v.as_str())
            .map(|s| !s.is_empty())
            .unwrap_or(false),
        "verifiedName": c.get("verifiedName"),
    })
}

fn static_file(public_dir: &Path, path: &str) -> (u16, &'static str, Vec<u8>) {
    let rel = match path {
        "/" | "/onboard" | "/onboard/" => "index.html",
        other => other.trim_start_matches('/'),
    };
    if rel.contains("..") {
        return (400, "text/plain", b"Bad path".to_vec());
    }
    let file = public_dir.join(rel);
    match fs::read(&file) {
        Ok(bytes) => (200, content_type(&file), bytes),
        Err(_) => (404, "text/plain", b"Not found".to_vec()),
    }
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn json_ok(v: Value) -> (u16, &'static str, Vec<u8>) {
    (
        200,
        "application/json; charset=utf-8",
        v.to_string().into_bytes(),
    )
}

fn json_status(code: u16, v: Value) -> (u16, &'static str, Vec<u8>) {
    (
        code,
        "application/json; charset=utf-8",
        v.to_string().into_bytes(),
    )
}

fn json_err(code: u16, msg: &str) -> (u16, &'static str, Vec<u8>) {
    json_status(code, json!({ "error": msg }))
}

fn q(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let mut it = pair.splitn(2, '=');
        let k = it.next()?;
        let v = it.next().unwrap_or("");
        if k == key {
            return Some(url_decode(v));
        }
    }
    None
}

fn header(headers: &[(String, String)], name: &str) -> Option<String> {
    headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case(name))
        .map(|(_, v)| v.clone())
}

fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let h = u8::from_str_radix(
                    std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or("00"),
                    16,
                )
                .unwrap_or(0);
                out.push(h);
                i += 3;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn now_ns() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(1)
}

fn read_http(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
    let mut buf = vec![0u8; 16 * 1024];
    let mut data = Vec::new();
    loop {
        let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        data.extend_from_slice(&buf[..n]);
        if let Some(pos) = find_headers_end(&data) {
            let headers = &data[..pos];
            let content_length = parse_content_length(headers).unwrap_or(0);
            if content_length > MAX_BODY {
                return Err("body too large".into());
            }
            while data.len() < pos + content_length {
                let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
                if n == 0 {
                    break;
                }
                data.extend_from_slice(&buf[..n]);
            }
            return Ok(data);
        }
        if data.len() > 64 * 1024 {
            return Err("headers too large".into());
        }
    }
    Ok(data)
}

fn find_headers_end(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|w| w == b"\r\n\r\n").map(|p| p + 4)
}

fn parse_content_length(headers: &[u8]) -> Option<usize> {
    let text = std::str::from_utf8(headers).ok()?;
    for line in text.lines() {
        if let Some(v) = line
            .split_once(':')
            .filter(|(k, _)| k.eq_ignore_ascii_case("content-length"))
            .map(|(_, v)| v.trim())
        {
            return v.parse().ok();
        }
    }
    None
}

fn parse_request(raw: &[u8]) -> Option<(String, String, String, Vec<(String, String)>, Vec<u8>)> {
    let pos = find_headers_end(raw)?;
    let head = std::str::from_utf8(&raw[..pos]).ok()?;
    let mut lines = head.split("\r\n");
    let req = lines.next()?;
    let mut parts = req.split_whitespace();
    let method = parts.next()?.to_string();
    let target = parts.next()?;
    let (path, query) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target.to_string(), String::new()),
    };
    let mut headers = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }
    let body = raw[pos..].to_vec();
    Some((method, path, query, headers, body))
}

fn write_response(
    stream: &mut TcpStream,
    code: u16,
    ctype: &str,
    body: &[u8],
    extra: Vec<(&'static str, String)>,
) -> Result<(), String> {
    let reason = match code {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        502 => "Bad Gateway",
        _ => "Error",
    };
    let mut head = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    );
    for (k, v) in extra {
        head.push_str(&format!("{k}: {v}\r\n"));
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes()).map_err(|e| e.to_string())?;
    stream.write_all(body).map_err(|e| e.to_string())?;
    Ok(())
}
