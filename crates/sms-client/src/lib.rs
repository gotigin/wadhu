//! Brand-neutral HTTP helpers for Wadhu SMS Gateway / SMS Relay.
//!
//! Talk to either endpoint with the same message schema:
//!
//! - Device LAN: `http://<phone>:18765`
//! - Self-hosted relay: `http://<relay>:9876`
//!
//! # Example
//!
//! ```
//! use wadhu_sms_client::{AcceptRequest, Client};
//!
//! let client = Client::new("http://127.0.0.1:18765", "wadhu-sms-demo");
//! let _ = AcceptRequest {
//!     to: "+15551212".into(),
//!     body: "hello".into(),
//!     idempotency_key: Some("demo-1".into()),
//! };
//! // client.accept(&_).unwrap();  // requires a live gateway
//! assert_eq!(client.base_url(), "http://127.0.0.1:18765");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

/// Why a client call failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    /// Bad base URL.
    BadUrl(String),
    /// Transport / I/O failure.
    Io(String),
    /// HTTP non-success or malformed body.
    Api {
        /// Status code when known.
        status: u16,
        /// Response body or reason.
        body: String,
    },
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::BadUrl(s) => write!(f, "bad url: {s}"),
            ClientError::Io(s) => write!(f, "io: {s}"),
            ClientError::Api { status, body } => write!(f, "api {status}: {body}"),
        }
    }
}

impl std::error::Error for ClientError {}

/// Body for `POST /v1/messages`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AcceptRequest {
    /// Destination number.
    pub to: String,
    /// Message text.
    pub body: String,
    /// Optional idempotency key (retries return the original message).
    pub idempotency_key: Option<String>,
}

/// Minimal view of an accepted / listed message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    /// Gateway/relay message id.
    pub id: String,
    /// Lifecycle status string (`queued`, `submitted`, …).
    pub status: String,
    /// Destination.
    pub to: String,
}

/// HTTP client for gateway or relay `/v1` APIs.
#[derive(Clone, Debug)]
pub struct Client {
    base: String,
    token: String,
    timeout: Duration,
}

impl Client {
    /// Create a client. `base` is `http://host:port` with no trailing slash.
    pub fn new(base: impl Into<String>, token: impl Into<String>) -> Self {
        let mut base = base.into();
        while base.ends_with('/') {
            base.pop();
        }
        Self {
            base,
            token: token.into(),
            timeout: Duration::from_secs(10),
        }
    }

    /// Configured base URL.
    pub fn base_url(&self) -> &str {
        &self.base
    }

    /// `GET /v1/health` raw JSON body (after HTTP headers).
    pub fn health(&self) -> Result<String, ClientError> {
        self.request("GET", "/v1/health", None)
    }

    /// `POST /v1/messages`.
    pub fn accept(&self, req: &AcceptRequest) -> Result<Message, ClientError> {
        let idem = req
            .idempotency_key
            .as_deref()
            .unwrap_or("");
        let body = format!(
            r#"{{"to":{},"body":{},"idempotency_key":{}}}"#,
            json_str(&req.to),
            json_str(&req.body),
            json_str(idem)
        );
        let raw = self.request("POST", "/v1/messages", Some(&body))?;
        let id = json_get(&raw, "id").ok_or_else(|| ClientError::Api {
            status: 0,
            body: raw.clone(),
        })?;
        let status = json_get(&raw, "status").unwrap_or_else(|| "queued".into());
        let to = json_get(&raw, "to").unwrap_or_else(|| req.to.clone());
        Ok(Message { id, status, to })
    }

    /// `GET /v1/messages/{id}` status string.
    pub fn status(&self, id: &str) -> Result<String, ClientError> {
        let path = format!("/v1/messages/{id}");
        let raw = self.request("GET", &path, None)?;
        json_get(&raw, "status").ok_or(ClientError::Api {
            status: 0,
            body: raw,
        })
    }

    fn request(&self, method: &str, path: &str, body: Option<&str>) -> Result<String, ClientError> {
        let (host, port) = parse_http_base(&self.base)?;
        let mut stream = TcpStream::connect((host.as_str(), port)).map_err(|e| {
            ClientError::Io(e.to_string())
        })?;
        stream.set_read_timeout(Some(self.timeout)).ok();
        stream.set_write_timeout(Some(self.timeout)).ok();
        let req = if let Some(body) = body {
            format!(
                "{method} {path} HTTP/1.1\r\nHost: {host}:{port}\r\nAuthorization: Bearer {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                self.token,
                body.len()
            )
        } else {
            format!(
                "{method} {path} HTTP/1.1\r\nHost: {host}:{port}\r\nAuthorization: Bearer {}\r\nConnection: close\r\n\r\n",
                self.token
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
}

fn parse_http_base(base: &str) -> Result<(String, u16), ClientError> {
    let u = base
        .strip_prefix("http://")
        .ok_or_else(|| ClientError::BadUrl("only http:// is supported in this crate".into()))?;
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

fn json_get(body: &str, key: &str) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_base_url() {
        let (h, p) = parse_http_base("http://127.0.0.1:18765").unwrap();
        assert_eq!(h, "127.0.0.1");
        assert_eq!(p, 18765);
    }

    #[test]
    fn reject_https_for_now() {
        assert!(matches!(
            parse_http_base("https://example.com"),
            Err(ClientError::BadUrl(_))
        ));
    }
}
