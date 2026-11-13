//! Persist API token and outbox across process restarts.

use std::fs;
use std::path::PathBuf;

use super::message::{Message, Status};
use super::outbox::Outbox;
use crate::identity::APP_ID;

const STATE_FILE: &str = "gateway-state.json";
const MAX_PERSISTED: usize = 500;

#[derive(Clone, Debug)]
pub struct PersistedState {
    pub api_token: String,
    pub messages: Vec<Message>,
    pub relay_url: String,
    pub relay_device_id: String,
    pub relay_device_secret: String,
    pub bluetooth_enabled: bool,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            api_token: String::new(),
            messages: Vec::new(),
            relay_url: String::new(),
            relay_device_id: String::new(),
            relay_device_secret: String::new(),
            bluetooth_enabled: true,
        }
    }
}

/// Platform data directory for this app id (XDG / APPDATA / home fallback).
pub fn data_dir() -> PathBuf {
    if let Ok(override_dir) = std::env::var("WADHU_SMS_GATEWAY_DATA") {
        return PathBuf::from(override_dir);
    }
    #[cfg(target_os = "android")]
    {
        // Makepad Android process cwd is usable; prefer files dir if exposed.
        if let Ok(files) = std::env::var("ANDROID_DATA") {
            return PathBuf::from(files).join("data").join(APP_ID).join("files");
        }
        return PathBuf::from(".").join("wadhu-sms-gateway-data");
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(base) = std::env::var("APPDATA") {
            return PathBuf::from(base).join(APP_ID);
        }
        return PathBuf::from(".").join(APP_ID);
    }
    #[cfg(not(any(target_os = "android", target_os = "windows")))]
    {
        if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
            return PathBuf::from(xdg).join(APP_ID);
        }
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join(APP_ID);
        }
        PathBuf::from(".").join(APP_ID)
    }
}

pub fn state_path() -> PathBuf {
    data_dir().join(STATE_FILE)
}

pub fn load() -> PersistedState {
    let path = state_path();
    match fs::read_to_string(&path) {
        Ok(raw) => parse_state(&raw).unwrap_or_default(),
        Err(_) => PersistedState::default(),
    }
}

pub fn save(state: &PersistedState) -> Result<(), String> {
    let dir = data_dir();
    fs::create_dir_all(&dir).map_err(|e| format!("create data dir {}: {e}", dir.display()))?;
    let path = dir.join(STATE_FILE);
    let tmp = dir.join(format!("{STATE_FILE}.tmp"));
    let body = serialize_state(state);
    fs::write(&tmp, body.as_bytes()).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    fs::rename(&tmp, &path).map_err(|e| format!("rename {}: {e}", path.display()))?;
    Ok(())
}

pub fn save_from_outbox(
    api_token: &str,
    outbox: &Outbox,
    relay_url: &str,
    relay_device_id: &str,
    relay_device_secret: &str,
    bluetooth_enabled: bool,
) {
    let mut messages = outbox.list(MAX_PERSISTED);
    messages.reverse(); // list() is newest-first; store oldest-first
    let state = PersistedState {
        api_token: api_token.to_string(),
        messages,
        relay_url: relay_url.to_string(),
        relay_device_id: relay_device_id.to_string(),
        relay_device_secret: relay_device_secret.to_string(),
        bluetooth_enabled,
    };
    if let Err(e) = save(&state) {
        eprintln!("sms-gateway persist: {e}");
    }
}

fn serialize_state(state: &PersistedState) -> String {
    let mut msgs = String::new();
    for (i, m) in state.messages.iter().enumerate() {
        if i > 0 {
            msgs.push(',');
        }
        msgs.push_str(&message_json(m));
    }
    format!(
        "{{\n  \"api_token\":{},\n  \"relay_url\":{},\n  \"relay_device_id\":{},\n  \"relay_device_secret\":{},\n  \"bluetooth_enabled\":{},\n  \"messages\":[{msgs}]\n}}\n",
        json_str(&state.api_token),
        json_str(&state.relay_url),
        json_str(&state.relay_device_id),
        json_str(&state.relay_device_secret),
        state.bluetooth_enabled,
    )
}

fn message_json(m: &Message) -> String {
    format!(
        "{{\"id\":{},\"idempotency_key\":{},\"to\":{},\"body\":{},\"status\":{},\"error\":{},\"created_at_ms\":{},\"updated_at_ms\":{}}}",
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

fn parse_state(raw: &str) -> Option<PersistedState> {
    let api_token = json_get(raw, "api_token").unwrap_or_default();
    let relay_url = json_get(raw, "relay_url").unwrap_or_default();
    let relay_device_id = json_get(raw, "relay_device_id").unwrap_or_default();
    let relay_device_secret = json_get(raw, "relay_device_secret").unwrap_or_default();
    let bluetooth_enabled = raw.contains("\"bluetooth_enabled\":true")
        || !raw.contains("\"bluetooth_enabled\":false");
    let messages = parse_messages_array(raw);
    Some(PersistedState {
        api_token,
        messages,
        relay_url,
        relay_device_id,
        relay_device_secret,
        bluetooth_enabled,
    })
}

fn parse_messages_array(raw: &str) -> Vec<Message> {
    let Some(start) = raw.find("\"messages\"") else {
        return Vec::new();
    };
    let rest = &raw[start..];
    let Some(bracket) = rest.find('[') else {
        return Vec::new();
    };
    let mut depth = 0i32;
    let bytes = rest.as_bytes();
    let mut end = bracket;
    for i in bracket..rest.len() {
        match bytes[i] {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    let arr = &rest[bracket + 1..end];
    let mut out = Vec::new();
    let mut obj_start: Option<usize> = None;
    let mut obj_depth = 0i32;
    for (i, b) in arr.bytes().enumerate() {
        match b {
            b'{' => {
                if obj_depth == 0 {
                    obj_start = Some(i);
                }
                obj_depth += 1;
            }
            b'}' => {
                obj_depth -= 1;
                if obj_depth == 0 {
                    if let Some(s) = obj_start.take() {
                        if let Some(m) = parse_message(&arr[s..=i]) {
                            out.push(m);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

fn parse_message(raw: &str) -> Option<Message> {
    let id = json_get(raw, "id")?;
    let idempotency_key = json_get(raw, "idempotency_key").unwrap_or_default();
    let to = json_get(raw, "to")?;
    let body = json_get(raw, "body").unwrap_or_default();
    let status = status_from_str(&json_get(raw, "status").unwrap_or_else(|| "queued".into()));
    let error = json_get(raw, "error").filter(|e| e != "null" && !e.is_empty());
    let created_at_ms = json_get(raw, "created_at_ms")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let updated_at_ms = json_get(raw, "updated_at_ms")
        .and_then(|s| s.parse().ok())
        .unwrap_or(created_at_ms);
    Some(Message {
        id,
        idempotency_key,
        to,
        body,
        status,
        error,
        created_at_ms,
        updated_at_ms,
    })
}

fn status_from_str(s: &str) -> Status {
    match s {
        "queued" => Status::Queued,
        "submitting" | "sending" => Status::Submitting,
        "submitted" => Status::Submitted,
        "delivered" => Status::Delivered,
        "failed" => Status::Failed,
        "expired" => Status::Expired,
        "cancelled" => Status::Cancelled,
        "indeterminate" => Status::Indeterminate,
        _ => Status::Queued,
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
    if after.starts_with("null") {
        return None;
    }
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
        let end = after
            .find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace())
            .unwrap_or(after.len());
        Some(after[..end].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_state() {
        let dir = std::env::temp_dir().join(format!(
            "wadhu-sms-gateway-persist-test-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        // SAFETY: test-only env override for isolated path.
        unsafe {
            std::env::set_var("WADHU_SMS_GATEWAY_DATA", &dir);
        }

        let mut state = PersistedState::default();
        state.api_token = "tok-abc".into();
        state.relay_url = "http://127.0.0.1:9876".into();
        state.messages.push(Message::new(
            "k1".into(),
            "+1555".into(),
            "hello".into(),
        ));
        save(&state).unwrap();
        let loaded = load();
        assert_eq!(loaded.api_token, "tok-abc");
        assert_eq!(loaded.relay_url, "http://127.0.0.1:9876");
        assert_eq!(loaded.messages.len(), 1);
        assert_eq!(loaded.messages[0].to, "+1555");

        let _ = fs::remove_dir_all(&dir);
        unsafe {
            std::env::remove_var("WADHU_SMS_GATEWAY_DATA");
        }
    }

    #[test]
    fn data_dir_is_non_empty() {
        let p = data_dir();
        assert!(!p.as_os_str().is_empty());
    }
}
