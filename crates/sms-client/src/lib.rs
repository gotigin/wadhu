//! Brand-neutral HTTP helpers for Wadhu SMS Gateway / SMS Relay.
//!
//! Talk to either endpoint with the same message schema:
//!
//! - Device LAN: `http://<phone>:18765`
//! - Self-hosted relay: `http://<relay>:9876`
//!
//! # Platforms
//!
//! | Target | API |
//! |--------|-----|
//! | Linux / Windows / macOS / Android / iOS | Sync methods (`health`, `accept`, …) over HTTP/1.1 |
//! | `wasm32` (browsers / Edge) | Sync methods return [`ClientError::Unsupported`]; use `*_async` (fetch) |
//!
//! Browser pages calling a phone on LAN also need CORS on the gateway (Wadhu
//! sms-gateway answers `OPTIONS` + `Access-Control-Allow-*`).
//!
//! # Example
//!
//! ```
//! use wadhu_sms_client::{AcceptRequest, Client};
//!
//! // Prefer the phone LAN Listen URL from SMS Gateway (same Wi‑Fi).
//! let client = Client::new("http://192.168.1.40:18765", "wadhu-sms-demo");
//! let _ = AcceptRequest {
//!     to: "+15551212".into(),
//!     body: "hello".into(),
//!     idempotency_key: Some("demo-1".into()),
//! };
//! // client.accept(&_).unwrap();  // requires a live gateway
//! assert_eq!(client.base_url(), "http://192.168.1.40:18765");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::fmt;
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

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
    /// Sync HTTP is not available on this target (use the async API on wasm).
    Unsupported(&'static str),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::BadUrl(s) => write!(f, "bad url: {s}"),
            ClientError::Io(s) => write!(f, "io: {s}"),
            ClientError::Api { status, body } => write!(f, "api {status}: {body}"),
            ClientError::Unsupported(s) => write!(f, "unsupported: {s}"),
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
    /// Error detail when status is `failed`.
    pub error: Option<String>,
}

/// HTTP client for gateway or relay `/v1` APIs.
#[derive(Clone, Debug)]
pub struct Client {
    base: String,
    token: String,
    timeout: Duration,
}

impl Client {
    /// Create a client. `base` is `http://host:port` (or `https://` on wasm) with no trailing slash.
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

    /// API token (Bearer / X-Api-Key).
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Request timeout (native sockets; wasm uses browser defaults).
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// `GET /v1/health` raw JSON body.
    pub fn health(&self) -> Result<String, ClientError> {
        self.request("GET", "/v1/health", None)
    }

    /// `GET /v1/capabilities` raw JSON body.
    pub fn capabilities(&self) -> Result<String, ClientError> {
        self.request("GET", "/v1/capabilities", None)
    }

    /// `POST /v1/messages`.
    pub fn accept(&self, req: &AcceptRequest) -> Result<Message, ClientError> {
        let raw = self.request("POST", "/v1/messages", Some(&accept_body(req)))?;
        parse_message(&raw).ok_or_else(|| ClientError::Api {
            status: 0,
            body: raw,
        })
    }

    /// `GET /v1/messages/{id}`.
    pub fn get_message(&self, id: &str) -> Result<Message, ClientError> {
        let path = format!("/v1/messages/{id}");
        let raw = self.request("GET", &path, None)?;
        parse_message(&raw).ok_or_else(|| ClientError::Api {
            status: 0,
            body: raw,
        })
    }

    /// `GET /v1/messages/{id}` status string.
    pub fn status(&self, id: &str) -> Result<String, ClientError> {
        Ok(self.get_message(id)?.status)
    }

    /// `GET /v1/messages` — recent list.
    pub fn list_messages(&self) -> Result<Vec<Message>, ClientError> {
        let raw = self.request("GET", "/v1/messages", None)?;
        Ok(parse_message_list(&raw))
    }

    fn request(&self, method: &str, path: &str, body: Option<&str>) -> Result<String, ClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            native::request(self, method, path, body)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (method, path, body);
            Err(ClientError::Unsupported(
                "sync HTTP is unavailable on wasm32; use health_async / accept_async / …",
            ))
        }
    }

    /// Async `GET /v1/health` (wasm / any future async backends).
    #[cfg(target_arch = "wasm32")]
    pub async fn health_async(&self) -> Result<String, ClientError> {
        wasm::request(self, "GET", "/v1/health", None).await
    }

    /// Async `GET /v1/capabilities`.
    #[cfg(target_arch = "wasm32")]
    pub async fn capabilities_async(&self) -> Result<String, ClientError> {
        wasm::request(self, "GET", "/v1/capabilities", None).await
    }

    /// Async `POST /v1/messages`.
    #[cfg(target_arch = "wasm32")]
    pub async fn accept_async(&self, req: &AcceptRequest) -> Result<Message, ClientError> {
        let raw = wasm::request(self, "POST", "/v1/messages", Some(&accept_body(req))).await?;
        parse_message(&raw).ok_or_else(|| ClientError::Api {
            status: 0,
            body: raw,
        })
    }

    /// Async `GET /v1/messages/{id}`.
    #[cfg(target_arch = "wasm32")]
    pub async fn get_message_async(&self, id: &str) -> Result<Message, ClientError> {
        let path = format!("/v1/messages/{id}");
        let raw = wasm::request(self, "GET", &path, None).await?;
        parse_message(&raw).ok_or_else(|| ClientError::Api {
            status: 0,
            body: raw,
        })
    }

    /// Async status string for one message.
    #[cfg(target_arch = "wasm32")]
    pub async fn status_async(&self, id: &str) -> Result<String, ClientError> {
        Ok(self.get_message_async(id).await?.status)
    }

    /// Async recent list.
    #[cfg(target_arch = "wasm32")]
    pub async fn list_messages_async(&self) -> Result<Vec<Message>, ClientError> {
        let raw = wasm::request(self, "GET", "/v1/messages", None).await?;
        Ok(parse_message_list(&raw))
    }
}

fn accept_body(req: &AcceptRequest) -> String {
    let idem = req.idempotency_key.as_deref().unwrap_or("");
    format!(
        r#"{{"to":{},"body":{},"idempotency_key":{}}}"#,
        json_str(&req.to),
        json_str(&req.body),
        json_str(idem)
    )
}

fn parse_message(raw: &str) -> Option<Message> {
    let id = json_get(raw, "id")?;
    let status = json_get(raw, "status").unwrap_or_else(|| "queued".into());
    let to = json_get(raw, "to").unwrap_or_default();
    let error = json_get(raw, "error").filter(|e| e != "null" && !e.is_empty());
    Some(Message {
        id,
        status,
        to,
        error,
    })
}

fn parse_message_list(raw: &str) -> Vec<Message> {
    // Enough for `{"messages":[{…},{…}]}` — split on `},{` boundaries inside the array.
    let Some(start) = raw.find('[') else {
        return Vec::new();
    };
    let Some(end) = raw.rfind(']') else {
        return Vec::new();
    };
    if end <= start + 1 {
        return Vec::new();
    }
    let inner = &raw[start + 1..end];
    if inner.trim().is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut chunk_start = 0usize;
    let bytes = inner.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    let piece = inner[chunk_start..=i].trim();
                    if let Some(m) = parse_message(piece) {
                        out.push(m);
                    }
                    // skip comma after object
                    chunk_start = i + 1;
                }
            }
            _ => {}
        }
    }
    out
}

pub(crate) fn json_str(s: &str) -> String {
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

pub(crate) fn json_get(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let i = body.find(&needle)?;
    let after = body[i + needle.len()..]
        .trim_start()
        .strip_prefix(':')?
        .trim_start();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_base_url_native() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (h, p) = native::parse_http_base("http://127.0.0.1:18765").unwrap();
            assert_eq!(h, "127.0.0.1");
            assert_eq!(p, 18765);
        }
    }

    #[test]
    fn reject_https_on_native_tcp() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            assert!(matches!(
                native::parse_http_base("https://example.com"),
                Err(ClientError::BadUrl(_))
            ));
        }
    }

    #[test]
    fn parse_message_round_trip_fields() {
        let raw = r#"{"id":"msg-1","status":"submitted","to":"+1555","error":null}"#;
        let m = parse_message(raw).unwrap();
        assert_eq!(m.id, "msg-1");
        assert_eq!(m.status, "submitted");
        assert_eq!(m.to, "+1555");
        assert!(m.error.is_none());
    }

    #[test]
    fn parse_list_two() {
        let raw = r#"{"messages":[{"id":"a","status":"queued","to":"+1"},{"id":"b","status":"failed","to":"+2","error":"x"}]}"#;
        let list = parse_message_list(raw);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].id, "a");
        assert_eq!(list[1].error.as_deref(), Some("x"));
    }
}
