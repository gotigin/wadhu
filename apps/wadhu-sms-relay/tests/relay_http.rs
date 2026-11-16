//! Integration: admin pair + client key + message accept.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

struct RelayProc {
    child: Child,
    addr: SocketAddr,
    admin: String,
    data: std::path::PathBuf,
}

impl Drop for RelayProc {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_dir_all(&self.data);
    }
}

fn start_relay() -> RelayProc {
    let data = std::env::temp_dir().join(format!("wadhu-relay-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&data);
    std::fs::create_dir_all(&data).unwrap();
    let admin = "test-admin-token".to_string();
    // Bind port 0 via SMS_RELAY_BIND is not supported for discovery easily —
    // pick a high port based on pid.
    let port = 20000 + (std::process::id() % 20000) as u16;
    let bind = format!("127.0.0.1:{port}");
    let addr: SocketAddr = bind.parse().unwrap();
    let child = Command::new(env!("CARGO_BIN_EXE_wadhu-sms-relay"))
        .env("SMS_RELAY_BIND", &bind)
        .env("SMS_RELAY_DATA", &data)
        .env("SMS_RELAY_ADMIN_TOKEN", &admin)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn relay");
    // Wait until health responds.
    for _ in 0..50 {
        std::thread::sleep(Duration::from_millis(50));
        if let Ok(body) = try_get(addr, "/v1/health", None) {
            if body.contains("wadhu-sms-relay") {
                return RelayProc {
                    child,
                    addr,
                    admin,
                    data,
                };
            }
        }
    }
    panic!("relay did not become ready on {addr}");
}

#[test]
fn pair_key_and_accept_message() {
    let relay = start_relay();
    let health = http_get(relay.addr, "/v1/health", None);
    assert!(health.contains("wadhu-sms-relay"), "{health}");

    let pair = http_post(
        relay.addr,
        "/v1/devices/pair",
        Some(&relay.admin),
        "{}",
    );
    assert!(pair.contains("device_id"), "{pair}");
    assert!(pair.contains("device_secret"), "{pair}");

    let key_resp = http_post(
        relay.addr,
        "/v1/keys",
        Some(&relay.admin),
        r#"{"label":"test"}"#,
    );
    let token = extract_str(&key_resp, "token").expect("token");

    let accepted = http_post(
        relay.addr,
        "/v1/messages",
        Some(&token),
        r#"{"to":"+15551212","body":"via relay","idempotency_key":"r1"}"#,
    );
    assert!(accepted.contains("msg-"), "{accepted}");
    assert!(
        accepted.contains("queued") || accepted.contains("submitting"),
        "{accepted}"
    );
}

fn try_get(addr: SocketAddr, path: &str, token: Option<&str>) -> Result<String, ()> {
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(200)).map_err(|_| ())?;
    stream.set_read_timeout(Some(Duration::from_millis(500))).ok();
    let auth = token
        .map(|t| format!("Authorization: Bearer {t}\r\n"))
        .unwrap_or_default();
    let req = format!("GET {path} HTTP/1.1\r\nHost: localhost\r\n{auth}Connection: close\r\n\r\n");
    stream.write_all(req.as_bytes()).map_err(|_| ())?;
    Ok(read_all(&mut stream))
}

fn http_get(addr: SocketAddr, path: &str, token: Option<&str>) -> String {
    try_get(addr, path, token).expect("get")
}

fn http_post(addr: SocketAddr, path: &str, token: Option<&str>, body: &str) -> String {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(3))).ok();
    let auth = token
        .map(|t| format!("Authorization: Bearer {t}\r\n"))
        .unwrap_or_default();
    let req = format!(
        "POST {path} HTTP/1.1\r\nHost: localhost\r\n{auth}Content-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(req.as_bytes()).unwrap();
    read_all(&mut stream)
}

fn read_all(stream: &mut TcpStream) -> String {
    let mut out = Vec::new();
    let mut buf = [0u8; 4096];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
            Err(_) => break,
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn extract_str(http: &str, key: &str) -> Option<String> {
    let body = http.split("\r\n\r\n").nth(1)?;
    let needle = format!("\"{key}\":\"");
    let i = body.find(&needle)? + needle.len();
    let rest = &body[i..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}
