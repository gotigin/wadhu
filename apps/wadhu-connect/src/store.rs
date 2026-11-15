//! Local connection + event persistence (JSON / JSONL).

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};

#[derive(Clone)]
pub struct Store {
    inner: Arc<Mutex<StoreInner>>,
}

struct StoreInner {
    connection_path: PathBuf,
    events_path: PathBuf,
    sessions_path: PathBuf,
}

impl Store {
    pub fn open(data_dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
        Ok(Self {
            inner: Arc::new(Mutex::new(StoreInner {
                connection_path: data_dir.join("connection.json"),
                events_path: data_dir.join("webhook-events.jsonl"),
                sessions_path: data_dir.join("es-sessions.jsonl"),
            })),
        })
    }

    pub fn read_connection(&self) -> Option<Value> {
        let g = self.inner.lock().ok()?;
        read_connection_file(&g.connection_path)
    }

    pub fn write_connection(&self, patch: Value) -> Value {
        let g = self.inner.lock().expect("store lock");
        let mut next = read_connection_file(&g.connection_path).unwrap_or_else(|| json!({}));
        if let Some(obj) = patch.as_object() {
            let dst = next.as_object_mut().expect("connection object");
            for (k, v) in obj {
                if !v.is_null() {
                    dst.insert(k.clone(), v.clone());
                }
            }
        }
        if let Some(obj) = next.as_object_mut() {
            obj.insert("updatedAt".into(), json!(chrono_now()));
        }
        let pretty = serde_json::to_string_pretty(&next).unwrap_or_else(|_| "{}".into());
        let _ = fs::write(&g.connection_path, pretty);
        next
    }

    pub fn log_session(&self, event: Value) {
        self.append_jsonl("sessions", event);
    }

    pub fn log_webhook(&self, event: Value) {
        self.append_jsonl("webhooks", event);
    }

    fn append_jsonl(&self, which: &str, mut event: Value) {
        let Ok(g) = self.inner.lock() else {
            return;
        };
        if let Some(obj) = event.as_object_mut() {
            obj.insert("at".into(), json!(chrono_now()));
        }
        let path = if which == "sessions" {
            &g.sessions_path
        } else {
            &g.events_path
        };
        let line = serde_json::to_string(&event).unwrap_or_else(|_| "{}".into());
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(f, "{line}");
        }
    }

    pub fn recent_webhooks(&self, limit: usize) -> Value {
        let Ok(g) = self.inner.lock() else {
            return json!([]);
        };
        let Ok(raw) = fs::read_to_string(&g.events_path) else {
            return json!([]);
        };
        let lines: Vec<&str> = raw.lines().filter(|l| !l.is_empty()).collect();
        let start = lines.len().saturating_sub(limit);
        let mut out = Vec::new();
        for line in &lines[start..] {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                out.push(v);
            }
        }
        Value::Array(out)
    }
}

fn read_connection_file(path: &Path) -> Option<Value> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

fn chrono_now() -> String {
    // RFC3339-ish UTC without extra deps.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}
