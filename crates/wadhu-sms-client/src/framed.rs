//! Length-prefixed JSON framing for Bluetooth lab / Classic RFCOMM.
//!
//! Wire format matches [`apps/wadhu-sms-gateway/docs/BLUETOOTH.md`]:
//! `uint32_be length | utf-8 JSON` (max 64 KiB).
//!
//! Desktop lab: connect to the gateway’s TCP `:18766` simulator.
//! Production nearby clients use Classic RFCOMM SPP UUID
//! `00001101-0000-1000-8000-00805F9B34FB` (native BT in brand apps).

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::ClientError;

const MAX_FRAME: usize = 64 * 1024;

/// Default lab port for the gateway’s framed TCP listener (`127.0.0.1:18766`).
pub const DEFAULT_FRAMED_TCP_PORT: u16 = 18766;

/// Client for length-prefixed JSON ops (`ping`, `send`).
pub struct FramedClient {
    stream: TcpStream,
    token: String,
}

impl FramedClient {
    /// Connect to a framed TCP endpoint (BT lab / desktop simulator).
    ///
    /// `host` examples: `127.0.0.1`, phone USB-tether IP if the lab port is exposed.
    pub fn connect_tcp(host: &str, port: u16, api_token: impl Into<String>) -> Result<Self, ClientError> {
        let addr = format!("{host}:{port}");
        let stream = TcpStream::connect(&addr).map_err(|e| {
            ClientError::Io(format!(
                "framed TCP connect to {addr} failed: {e} — start SMS Gateway on this machine (lab) or point host at the framing port"
            ))
        })?;
        stream
            .set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| ClientError::Io(e.to_string()))?;
        stream
            .set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| ClientError::Io(e.to_string()))?;
        Ok(Self {
            stream,
            token: api_token.into(),
        })
    }

    /// Convenience: `127.0.0.1:18766`.
    pub fn connect_lab(api_token: impl Into<String>) -> Result<Self, ClientError> {
        Self::connect_tcp("127.0.0.1", DEFAULT_FRAMED_TCP_PORT, api_token)
    }

    /// `{"op":"ping"}` → ok/pong.
    pub fn ping(&mut self) -> Result<(), ClientError> {
        let reply = self.roundtrip(r#"{"op":"ping"}"#)?;
        if reply.contains("\"ok\":true") {
            Ok(())
        } else {
            Err(ClientError::Api {
                status: 0,
                body: reply,
            })
        }
    }

    /// Enqueue an SMS job (same fields as HTTP `POST /v1/messages`).
    pub fn send(
        &mut self,
        to: &str,
        body: &str,
        idempotency_key: Option<&str>,
    ) -> Result<String, ClientError> {
        let idem = idempotency_key.unwrap_or("");
        let req = format!(
            r#"{{"op":"send","token":{},"to":{},"body":{},"idempotency_key":{}}}"#,
            json_str(&self.token),
            json_str(to),
            json_str(body),
            json_str(idem),
        );
        let reply = self.roundtrip(&req)?;
        if reply.contains("\"ok\":true") {
            Ok(reply)
        } else {
            Err(ClientError::Api {
                status: 0,
                body: reply,
            })
        }
    }

    fn roundtrip(&mut self, json: &str) -> Result<String, ClientError> {
        write_frame(&mut self.stream, json)?;
        read_frame(&mut self.stream)
    }
}

fn write_frame(w: &mut impl Write, json: &str) -> Result<(), ClientError> {
    let bytes = json.as_bytes();
    if bytes.len() > MAX_FRAME {
        return Err(ClientError::Io("frame too large".into()));
    }
    let len = (bytes.len() as u32).to_be_bytes();
    w.write_all(&len)
        .and_then(|_| w.write_all(bytes))
        .and_then(|_| w.flush())
        .map_err(|e| ClientError::Io(e.to_string()))
}

fn read_frame(r: &mut impl Read) -> Result<String, ClientError> {
    let mut len_buf = [0u8; 4];
    r.read_exact(&mut len_buf)
        .map_err(|e| ClientError::Io(format!("framed read header: {e}")))?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len > MAX_FRAME {
        return Err(ClientError::Io("frame too large".into()));
    }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)
        .map_err(|e| ClientError::Io(format!("framed read body: {e}")))?;
    String::from_utf8(buf).map_err(|e| ClientError::Io(e.to_string()))
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
