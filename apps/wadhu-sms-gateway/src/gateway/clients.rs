//! Connected apps: pairing codes, per-app tokens, block list, send counts.

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const PAIR_TTL_MS: u64 = 5 * 60 * 1000;

#[derive(Clone, Debug)]
pub struct RegisteredClient {
    pub id: String,
    pub label: String,
    pub token: String,
    pub blocked: bool,
    pub sent_count: u64,
    pub created_at_ms: u64,
}

#[derive(Clone, Debug)]
struct PairingSession {
    code: String,
    expires_at_ms: u64,
}

#[derive(Default)]
struct Inner {
    clients: Vec<RegisteredClient>,
    pairing: Option<PairingSession>,
}

/// Shared registry used by HTTP and the operator UI.
#[derive(Clone, Default)]
pub struct Clients {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthKind {
    /// Operator / advanced key (gateway UI master token).
    Operator,
    /// Paired brand app.
    Client { id: String, label: String },
}

impl Clients {
    pub fn from_list(clients: Vec<RegisteredClient>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                clients,
                pairing: None,
            })),
        }
    }

    pub fn snapshot(&self) -> Vec<RegisteredClient> {
        self.inner.lock().expect("clients").clients.clone()
    }

    pub fn pairing_code(&self) -> Option<(String, u64)> {
        let mut g = self.inner.lock().expect("clients");
        if let Some(p) = &g.pairing {
            if now_ms() < p.expires_at_ms {
                return Some((p.code.clone(), p.expires_at_ms));
            }
            g.pairing = None;
        }
        None
    }

    /// Start (or refresh) a 6-digit allow code. Valid ~5 minutes.
    pub fn begin_pairing(&self) -> String {
        let code = format!("{:06}", (now_ms() % 1_000_000) as u32);
        let expires_at_ms = now_ms().saturating_add(PAIR_TTL_MS);
        let mut g = self.inner.lock().expect("clients");
        g.pairing = Some(PairingSession {
            code: code.clone(),
            expires_at_ms,
        });
        code
    }

    /// Exchange a valid code for a new per-app token.
    pub fn pair(&self, code: &str, label: &str) -> Result<RegisteredClient, String> {
        let code = code.trim();
        let label = label.trim();
        if code.is_empty() {
            return Err("code_required".into());
        }
        if label.is_empty() {
            return Err("label_required".into());
        }
        let mut g = self.inner.lock().expect("clients");
        let Some(p) = g.pairing.as_ref() else {
            return Err("no_pairing".into());
        };
        if now_ms() >= p.expires_at_ms {
            g.pairing = None;
            return Err("code_expired".into());
        }
        if p.code != code {
            return Err("bad_code".into());
        }
        g.pairing = None;
        let id = format!("app-{:x}", now_ms());
        let token = format!("wadhu-app-{:x}-{:x}", now_ms(), label.len() as u64 * 31 + 7);
        let c = RegisteredClient {
            id: id.clone(),
            label: label.to_string(),
            token: token.clone(),
            blocked: false,
            sent_count: 0,
            created_at_ms: now_ms(),
        };
        g.clients.push(c.clone());
        Ok(c)
    }

    pub fn authorize(&self, presented: &str, operator_token: &str) -> Result<AuthKind, AuthFail> {
        if presented == operator_token {
            return Ok(AuthKind::Operator);
        }
        let g = self.inner.lock().expect("clients");
        for c in &g.clients {
            if c.token == presented {
                if c.blocked {
                    return Err(AuthFail::Blocked);
                }
                return Ok(AuthKind::Client {
                    id: c.id.clone(),
                    label: c.label.clone(),
                });
            }
        }
        Err(AuthFail::Unauthorized)
    }

    pub fn record_send(&self, client_id: &str) {
        let mut g = self.inner.lock().expect("clients");
        if let Some(c) = g.clients.iter_mut().find(|c| c.id == client_id) {
            c.sent_count = c.sent_count.saturating_add(1);
        }
    }

    pub fn set_blocked(&self, id_or_label: &str, blocked: bool) -> Result<(), String> {
        let needle = id_or_label.trim();
        if needle.is_empty() {
            return Err("name_required".into());
        }
        let mut g = self.inner.lock().expect("clients");
        let Some(c) = g
            .clients
            .iter_mut()
            .find(|c| c.id == needle || c.label.eq_ignore_ascii_case(needle))
        else {
            return Err("not_found".into());
        };
        c.blocked = blocked;
        Ok(())
    }

    /// After operator key rotate: clear all app tokens so they must re-pair.
    pub fn revoke_all_tokens(&self) {
        let mut g = self.inner.lock().expect("clients");
        g.clients.clear();
        g.pairing = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthFail {
    Unauthorized,
    Blocked,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_block_and_auth() {
        let clients = Clients::default();
        let code = clients.begin_pairing();
        let c = clients.pair(&code, "Moklanjo").unwrap();
        assert!(matches!(
            clients.authorize(&c.token, "op"),
            Ok(AuthKind::Client { .. })
        ));
        clients.set_blocked("Moklanjo", true).unwrap();
        assert_eq!(clients.authorize(&c.token, "op"), Err(AuthFail::Blocked));
        clients.record_send(&c.id);
        assert_eq!(clients.snapshot()[0].sent_count, 1);
    }
}
