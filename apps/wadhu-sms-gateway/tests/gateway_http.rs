//! Integration: local HTTP API accepts messages without the Makepad UI.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use wadhu_sms_gateway::gateway::Gateway;

#[test]
fn health_and_accept_round_trip() {
    let dir = std::env::temp_dir().join(format!(
        "wadhu-sms-gateway-http-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe {
        std::env::set_var("WADHU_SMS_GATEWAY_DATA", &dir);
    }

    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let gw = Gateway::start(addr).expect("start gateway");
    let token = gw.operator_token();
    let listen = gw.handle.listen;

    std::thread::sleep(Duration::from_millis(300));

    let health = http_exchange(listen, &token, "GET", "/v1/health", None);
    assert!(health.contains("\"ok\":true"), "{health}");
    assert!(health.contains("wadhu-sms-gateway"), "{health}");

    let body = r#"{"to":"+15551212","body":"hello from test","idempotency_key":"t1"}"#;
    let accepted = http_exchange(listen, &token, "POST", "/v1/messages", Some(body));
    assert!(
        accepted.contains("\"status\":\"queued\"")
            || accepted.contains("\"status\":\"submitted\"")
            || accepted.contains("\"status\":\"failed\""),
        "{accepted}"
    );
    assert!(accepted.contains("msg-"), "{accepted}");

    std::thread::sleep(Duration::from_millis(500));
    let id = extract_id(&accepted).expect("id");
    let got = http_exchange(listen, &token, "GET", &format!("/v1/messages/{id}"), None);
    assert!(
        got.contains("failed") || got.contains("submitted") || got.contains("queued"),
        "{got}"
    );
    drop(gw);
    let _ = std::fs::remove_dir_all(&dir);
    unsafe {
        std::env::remove_var("WADHU_SMS_GATEWAY_DATA");
    }
}

fn http_exchange(
    addr: SocketAddr,
    token: &str,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> String {
    let mut stream = TcpStream::connect(addr).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .ok();
    stream
        .set_write_timeout(Some(Duration::from_secs(3)))
        .ok();
    let req = if let Some(body) = body {
        format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nX-Api-Key: {token}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    } else {
        format!(
            "{method} {path} HTTP/1.1\r\nHost: localhost\r\nX-Api-Key: {token}\r\nConnection: close\r\n\r\n"
        )
    };
    stream.write_all(req.as_bytes()).unwrap();
    let _ = stream.flush();
    let mut out = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(e) => panic!("read failed: {e}"),
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn extract_id(http: &str) -> Option<String> {
    let body = http.split("\r\n\r\n").nth(1)?;
    let key = "\"id\":\"";
    let i = body.find(key)? + key.len();
    let rest = &body[i..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

#[test]
fn pair_code_issues_app_token_without_prior_auth() {
    let dir = std::env::temp_dir().join(format!(
        "wadhu-sms-gateway-pair-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe {
        std::env::set_var("WADHU_SMS_GATEWAY_DATA", &dir);
    }

    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let gw = Gateway::start(addr).expect("start gateway");
    let listen = gw.handle.listen;
    let code = gw.begin_pairing();
    std::thread::sleep(Duration::from_millis(200));

    let body = format!(r#"{{"code":"{code}","label":"DemoApp"}}"#);
    let mut stream = TcpStream::connect(listen).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .ok();
    let req = format!(
        "POST /v1/pair HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(req.as_bytes()).unwrap();
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
    assert!(text.contains("\"ok\":true"), "{text}");
    assert!(text.contains("wadhu-app-"), "{text}");
    assert!(text.contains("DemoApp"), "{text}");

    drop(gw);
    let _ = std::fs::remove_dir_all(&dir);
    unsafe {
        std::env::remove_var("WADHU_SMS_GATEWAY_DATA");
    }
}

#[test]
fn options_preflight_returns_cors_without_auth() {
    let dir = std::env::temp_dir().join(format!(
        "wadhu-sms-gateway-cors-test-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe {
        std::env::set_var("WADHU_SMS_GATEWAY_DATA", &dir);
    }

    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let gw = Gateway::start(addr).expect("start gateway");
    let listen = gw.handle.listen;
    std::thread::sleep(Duration::from_millis(200));

    let mut stream = TcpStream::connect(listen).expect("connect");
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .ok();
    let req = "OPTIONS /v1/messages HTTP/1.1\r\nHost: localhost\r\nOrigin: http://example.com\r\nAccess-Control-Request-Method: POST\r\nConnection: close\r\n\r\n";
    stream.write_all(req.as_bytes()).unwrap();
    let mut out = Vec::new();
    let mut buf = [0u8; 2048];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }
    let text = String::from_utf8_lossy(&out);
    assert!(text.contains("204"), "{text}");
    assert!(text.contains("Access-Control-Allow-Origin: *"), "{text}");
    assert!(text.contains("Access-Control-Allow-Methods:"), "{text}");

    drop(gw);
    let _ = std::fs::remove_dir_all(&dir);
    unsafe {
        std::env::remove_var("WADHU_SMS_GATEWAY_DATA");
    }
}
