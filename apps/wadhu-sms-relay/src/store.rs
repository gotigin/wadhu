//! Durable JSON store for devices, client keys, and messages.
//! Atomic rewrite of a single state file (self-host friendly, no native deps).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Device {
    pub id: String,
    pub secret: String,
    pub label: String,
    pub created_at_ms: u64,
    pub last_seen_ms: u64,
}

#[derive(Clone, Debug)]
pub struct ClientKey {
    pub id: String,
    pub token: String,
    pub label: String,
    pub device_id: Option<String>,
    pub created_at_ms: u64,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: String,
    pub idempotency_key: String,
    pub to: String,
    pub body: String,
    pub status: String,
    pub error: Option<String>,
    pub device_id: Option<String>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
}

#[derive(Clone, Debug, Default)]
struct State {
    devices: HashMap<String, Device>,
    /// secret -> device_id
    device_by_secret: HashMap<String, String>,
    client_keys: HashMap<String, ClientKey>,
    /// token -> client key id
    key_by_token: HashMap<String, String>,
    messages: HashMap<String, Message>,
    by_idem: HashMap<String, String>,
    message_order: Vec<String>,
    pair_codes: HashMap<String, PairCode>,
}

#[derive(Clone, Debug)]
struct PairCode {
    device_id: String,
    device_secret: String,
    expires_at_ms: u64,
}

#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<State>>,
    path: PathBuf,
}

impl Store {
    pub fn open(data_dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
        let path = data_dir.join("relay-state.json");
        let state = if path.exists() {
            let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            parse_state(&raw).unwrap_or_default()
        } else {
            State::default()
        };
        let store = Self {
            inner: Arc::new(Mutex::new(state)),
            path,
        };
        store.persist();
        Ok(store)
    }

    fn persist(&self) {
        let g = self.inner.lock().expect("store");
        let body = serialize_state(&g);
        let tmp = self.path.with_extension("json.tmp");
        if fs::write(&tmp, body.as_bytes()).is_ok() {
            let _ = fs::rename(&tmp, &self.path);
        }
    }

    pub fn create_pairing(&self) -> (String, String, String) {
        let device_id = format!("dev-{}", now_ms());
        let device_secret = format!("dsec-{:x}", now_ns());
        let code = format!("{:06}", (now_ns() % 1_000_000));
        let expires = now_ms() + 10 * 60 * 1000;
        {
            let mut g = self.inner.lock().expect("store");
            g.pair_codes.insert(
                code.clone(),
                PairCode {
                    device_id: device_id.clone(),
                    device_secret: device_secret.clone(),
                    expires_at_ms: expires,
                },
            );
        }
        self.persist();
        (code, device_id, device_secret)
    }

    pub fn redeem_pairing(&self, code: &str, label: &str) -> Result<(String, String), String> {
        let mut g = self.inner.lock().expect("store");
        let pair = g
            .pair_codes
            .remove(code.trim())
            .ok_or_else(|| "invalid_pairing_code".to_string())?;
        if pair.expires_at_ms < now_ms() {
            return Err("pairing_code_expired".into());
        }
        let device = Device {
            id: pair.device_id.clone(),
            secret: pair.device_secret.clone(),
            label: if label.is_empty() {
                "Android gateway".into()
            } else {
                label.into()
            },
            created_at_ms: now_ms(),
            last_seen_ms: 0,
        };
        g.device_by_secret
            .insert(device.secret.clone(), device.id.clone());
        g.devices.insert(device.id.clone(), device.clone());
        drop(g);
        self.persist();
        Ok((device.id, device.secret))
    }

    /// Register device credentials directly (used when phone already has id+secret from pair create).
    pub fn ensure_device(&self, id: &str, secret: &str, label: &str) {
        let mut g = self.inner.lock().expect("store");
        if g.devices.contains_key(id) {
            return;
        }
        let device = Device {
            id: id.to_string(),
            secret: secret.to_string(),
            label: label.to_string(),
            created_at_ms: now_ms(),
            last_seen_ms: 0,
        };
        g.device_by_secret
            .insert(secret.to_string(), id.to_string());
        g.devices.insert(id.to_string(), device);
        drop(g);
        self.persist();
    }

    pub fn auth_device(&self, id: &str, secret: &str) -> bool {
        let g = self.inner.lock().expect("store");
        g.devices
            .get(id)
            .map(|d| d.secret == secret)
            .unwrap_or(false)
            || g.device_by_secret.get(secret).map(|i| i == id).unwrap_or(false)
    }

    pub fn touch_device(&self, id: &str) {
        let mut g = self.inner.lock().expect("store");
        if let Some(d) = g.devices.get_mut(id) {
            d.last_seen_ms = now_ms();
        }
        drop(g);
        self.persist();
    }

    pub fn list_devices(&self) -> Vec<Device> {
        let g = self.inner.lock().expect("store");
        g.devices.values().cloned().collect()
    }

    pub fn create_client_key(&self, label: &str, device_id: Option<String>) -> ClientKey {
        let key = ClientKey {
            id: format!("key-{}", now_ms()),
            token: format!("ck-{:x}", now_ns()),
            label: if label.is_empty() {
                "client".into()
            } else {
                label.into()
            },
            device_id,
            created_at_ms: now_ms(),
        };
        {
            let mut g = self.inner.lock().expect("store");
            g.key_by_token.insert(key.token.clone(), key.id.clone());
            g.client_keys.insert(key.id.clone(), key.clone());
        }
        self.persist();
        key
    }

    pub fn auth_client(&self, token: &str) -> Option<ClientKey> {
        let g = self.inner.lock().expect("store");
        let id = g.key_by_token.get(token)?;
        g.client_keys.get(id).cloned()
    }

    pub fn accept_message(
        &self,
        idempotency_key: String,
        to: String,
        body: String,
        device_id: Option<String>,
    ) -> Result<Message, String> {
        if to.trim().is_empty() {
            return Err("to is required".into());
        }
        if body.is_empty() {
            return Err("body is required".into());
        }
        let key = if idempotency_key.trim().is_empty() {
            format!("auto-{}", now_ms())
        } else {
            idempotency_key
        };
        let msg = {
            let mut g = self.inner.lock().expect("store");
            if let Some(id) = g.by_idem.get(&key) {
                return Ok(g.messages.get(id).expect("msg").clone());
            }
            let now = now_ms();
            let msg = Message {
                id: format!("msg-{}-{}", now, g.message_order.len() + 1),
                idempotency_key: key.clone(),
                to: to.trim().into(),
                body,
                status: "queued".into(),
                error: None,
                device_id,
                created_at_ms: now,
                updated_at_ms: now,
            };
            g.by_idem.insert(key, msg.id.clone());
            g.message_order.push(msg.id.clone());
            g.messages.insert(msg.id.clone(), msg.clone());
            msg
        };
        self.persist();
        Ok(msg)
    }

    pub fn set_status(&self, id: &str, status: &str, error: Option<String>) -> Option<Message> {
        let msg = {
            let mut g = self.inner.lock().expect("store");
            let m = g.messages.get_mut(id)?;
            m.status = status.to_string();
            m.error = error;
            m.updated_at_ms = now_ms();
            m.clone()
        };
        self.persist();
        Some(msg)
    }

    pub fn get_message(&self, id: &str) -> Option<Message> {
        self.inner.lock().ok()?.messages.get(id).cloned()
    }

    pub fn list_messages(&self, limit: usize) -> Vec<Message> {
        let g = self.inner.lock().expect("store");
        g.message_order
            .iter()
            .rev()
            .take(limit)
            .filter_map(|id| g.messages.get(id).cloned())
            .collect()
    }

    pub fn pick_device(&self, preferred: Option<&str>) -> Option<String> {
        let g = self.inner.lock().expect("store");
        if let Some(id) = preferred {
            if g.devices.contains_key(id) {
                return Some(id.to_string());
            }
        }
        g.devices.keys().next().cloned()
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn now_ns() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(1)
}

fn serialize_state(s: &State) -> String {
    let mut devices = String::new();
    for (i, d) in s.devices.values().enumerate() {
        if i > 0 {
            devices.push(',');
        }
        devices.push_str(&format!(
            "{{\"id\":{},\"secret\":{},\"label\":{},\"created_at_ms\":{},\"last_seen_ms\":{}}}",
            j(&d.id),
            j(&d.secret),
            j(&d.label),
            d.created_at_ms,
            d.last_seen_ms
        ));
    }
    let mut keys = String::new();
    for (i, k) in s.client_keys.values().enumerate() {
        if i > 0 {
            keys.push(',');
        }
        let dev = k
            .device_id
            .as_ref()
            .map(|d| j(d))
            .unwrap_or_else(|| "null".into());
        keys.push_str(&format!(
            "{{\"id\":{},\"token\":{},\"label\":{},\"device_id\":{dev},\"created_at_ms\":{}}}",
            j(&k.id),
            j(&k.token),
            j(&k.label),
            k.created_at_ms
        ));
    }
    let mut msgs = String::new();
    for (i, id) in s.message_order.iter().enumerate() {
        if i > 0 {
            msgs.push(',');
        }
        if let Some(m) = s.messages.get(id) {
            let err = m
                .error
                .as_ref()
                .map(|e| j(e))
                .unwrap_or_else(|| "null".into());
            let dev = m
                .device_id
                .as_ref()
                .map(|d| j(d))
                .unwrap_or_else(|| "null".into());
            msgs.push_str(&format!(
                "{{\"id\":{},\"idempotency_key\":{},\"to\":{},\"body\":{},\"status\":{},\"error\":{err},\"device_id\":{dev},\"created_at_ms\":{},\"updated_at_ms\":{}}}",
                j(&m.id),
                j(&m.idempotency_key),
                j(&m.to),
                j(&m.body),
                j(&m.status),
                m.created_at_ms,
                m.updated_at_ms
            ));
        }
    }
    format!(
        "{{\"devices\":[{devices}],\"client_keys\":[{keys}],\"messages\":[{msgs}]}}\n"
    )
}

fn parse_state(raw: &str) -> Option<State> {
    let mut state = State::default();
    // Minimal extraction: walk device/client/message objects via "id" fields in arrays.
    for obj in iter_objects_in_array(raw, "devices") {
        let id = json_get(&obj, "id")?;
        let secret = json_get(&obj, "secret")?;
        let label = json_get(&obj, "label").unwrap_or_default();
        let created_at_ms = json_get(&obj, "created_at_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let last_seen_ms = json_get(&obj, "last_seen_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        state
            .device_by_secret
            .insert(secret.clone(), id.clone());
        state.devices.insert(
            id.clone(),
            Device {
                id,
                secret,
                label,
                created_at_ms,
                last_seen_ms,
            },
        );
    }
    for obj in iter_objects_in_array(raw, "client_keys") {
        let id = json_get(&obj, "id")?;
        let token = json_get(&obj, "token")?;
        let label = json_get(&obj, "label").unwrap_or_default();
        let device_id = json_get(&obj, "device_id");
        let created_at_ms = json_get(&obj, "created_at_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        state.key_by_token.insert(token.clone(), id.clone());
        state.client_keys.insert(
            id.clone(),
            ClientKey {
                id,
                token,
                label,
                device_id,
                created_at_ms,
            },
        );
    }
    for obj in iter_objects_in_array(raw, "messages") {
        let id = json_get(&obj, "id")?;
        let idem = json_get(&obj, "idempotency_key").unwrap_or_default();
        let to = json_get(&obj, "to").unwrap_or_default();
        let body = json_get(&obj, "body").unwrap_or_default();
        let status = json_get(&obj, "status").unwrap_or_else(|| "queued".into());
        let error = json_get(&obj, "error");
        let device_id = json_get(&obj, "device_id");
        let created_at_ms = json_get(&obj, "created_at_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let updated_at_ms = json_get(&obj, "updated_at_ms")
            .and_then(|s| s.parse().ok())
            .unwrap_or(created_at_ms);
        state.by_idem.insert(idem.clone(), id.clone());
        state.message_order.push(id.clone());
        state.messages.insert(
            id.clone(),
            Message {
                id,
                idempotency_key: idem,
                to,
                body,
                status,
                error,
                device_id,
                created_at_ms,
                updated_at_ms,
            },
        );
    }
    Some(state)
}

fn iter_objects_in_array(raw: &str, key: &str) -> Vec<String> {
    let Some(start) = raw.find(&format!("\"{key}\"")) else {
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
                        out.push(arr[s..=i].to_string());
                    }
                }
            }
            _ => {}
        }
    }
    out
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
