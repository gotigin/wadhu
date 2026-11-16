//! Durable outbox for accepted SMS jobs (in-memory + optional persist hook).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::message::{now_ms, Message, Status};

#[derive(Clone, Default)]
pub struct Outbox {
    inner: Arc<Mutex<Inner>>,
}

#[derive(Default)]
struct Inner {
    by_id: HashMap<String, Message>,
    by_idem: HashMap<String, String>,
    order: Vec<String>,
    on_change: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Outbox {
    pub fn new() -> Self {
        Self::default()
    }

    /// Restore previously persisted messages (e.g. after restart).
    pub fn restore(&self, messages: Vec<Message>) {
        let mut g = self.inner.lock().expect("outbox lock");
        for mut m in messages {
            // Incomplete radio handoffs should not auto-resend after restart.
            if m.status == Status::Submitting {
                m.status = Status::Indeterminate;
                m.error = Some(
                    "interrupted while submitting; not auto-resent (idempotency)".into(),
                );
            }
            g.by_idem.insert(m.idempotency_key.clone(), m.id.clone());
            if !g.order.iter().any(|id| id == &m.id) {
                g.order.push(m.id.clone());
            }
            g.by_id.insert(m.id.clone(), m);
        }
    }

    pub fn set_on_change(&self, hook: Arc<dyn Fn() + Send + Sync>) {
        let mut g = self.inner.lock().expect("outbox lock");
        g.on_change = Some(hook);
    }

    fn take_hook(g: &Inner) -> Option<Arc<dyn Fn() + Send + Sync>> {
        g.on_change.clone()
    }

    fn fire_hook(hook: Option<Arc<dyn Fn() + Send + Sync>>) {
        if let Some(hook) = hook {
            hook();
        }
    }

    /// Accept a message. Same idempotency key returns the original message.
    pub fn accept(&self, idempotency_key: String, to: String, body: String) -> Result<Message, String> {
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

        let (msg, hook) = {
            let mut g = self.inner.lock().expect("outbox lock");
            if let Some(id) = g.by_idem.get(&key) {
                return Ok(g.by_id.get(id).expect("idem map").clone());
            }
            let msg = Message::new(key.clone(), to.trim().to_string(), body);
            g.by_idem.insert(key, msg.id.clone());
            g.order.push(msg.id.clone());
            g.by_id.insert(msg.id.clone(), msg.clone());
            let hook = Self::take_hook(&g);
            (msg, hook)
        };
        Self::fire_hook(hook);
        Ok(msg)
    }

    /// Accept with an externally assigned id (relay job push).
    pub fn accept_with_id(
        &self,
        id: String,
        idempotency_key: String,
        to: String,
        body: String,
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
        let (msg, hook) = {
            let mut g = self.inner.lock().expect("outbox lock");
            if let Some(existing) = g.by_id.get(&id) {
                return Ok(existing.clone());
            }
            if let Some(eid) = g.by_idem.get(&key) {
                return Ok(g.by_id.get(eid).expect("idem map").clone());
            }
            let now = now_ms();
            let msg = Message {
                id: id.clone(),
                idempotency_key: key.clone(),
                to: to.trim().to_string(),
                body,
                status: Status::Queued,
                error: None,
                created_at_ms: now,
                updated_at_ms: now,
            };
            g.by_idem.insert(key, id.clone());
            g.order.push(id);
            g.by_id.insert(msg.id.clone(), msg.clone());
            let hook = Self::take_hook(&g);
            (msg, hook)
        };
        Self::fire_hook(hook);
        Ok(msg)
    }

    pub fn get(&self, id: &str) -> Option<Message> {
        self.inner.lock().ok()?.by_id.get(id).cloned()
    }

    pub fn list(&self, limit: usize) -> Vec<Message> {
        let g = self.inner.lock().expect("outbox lock");
        g.order
            .iter()
            .rev()
            .take(limit)
            .filter_map(|id| g.by_id.get(id).cloned())
            .collect()
    }

    pub fn next_queued(&self) -> Option<Message> {
        let g = self.inner.lock().expect("outbox lock");
        g.order.iter().find_map(|id| {
            let m = g.by_id.get(id)?;
            (m.status == Status::Queued).then(|| m.clone())
        })
    }

    pub fn set_status(&self, id: &str, status: Status, error: Option<String>) -> Option<Message> {
        let (cloned, hook) = {
            let mut g = self.inner.lock().expect("outbox lock");
            let m = g.by_id.get_mut(id)?;
            m.status = status;
            m.error = error;
            m.updated_at_ms = now_ms();
            let cloned = m.clone();
            let hook = Self::take_hook(&g);
            (cloned, hook)
        };
        Self::fire_hook(hook);
        Some(cloned)
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        let g = self.inner.lock().expect("outbox lock");
        let mut queued = 0;
        let mut failed = 0;
        let mut submitted = 0;
        for m in g.by_id.values() {
            match m.status {
                Status::Queued | Status::Submitting => queued += 1,
                Status::Failed => failed += 1,
                Status::Submitted | Status::Delivered => submitted += 1,
                _ => {}
            }
        }
        (queued, submitted, failed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idempotency_returns_same_message() {
        let box_ = Outbox::new();
        let a = box_
            .accept("k1".into(), "+15551212".into(), "hi".into())
            .unwrap();
        let b = box_
            .accept("k1".into(), "+15551212".into(), "hi".into())
            .unwrap();
        assert_eq!(a.id, b.id);
        assert_eq!(box_.list(10).len(), 1);
    }

    #[test]
    fn drain_moves_queued() {
        let box_ = Outbox::new();
        let m = box_.accept("k".into(), "+1".into(), "x".into()).unwrap();
        assert!(box_.next_queued().is_some());
        box_.set_status(&m.id, Status::Submitted, None);
        assert!(box_.next_queued().is_none());
    }

    #[test]
    fn restore_marks_submitting_indeterminate() {
        let box_ = Outbox::new();
        let mut m = Message::new("k".into(), "+1".into(), "x".into());
        m.status = Status::Submitting;
        box_.restore(vec![m.clone()]);
        let got = box_.get(&m.id).unwrap();
        assert_eq!(got.status, Status::Indeterminate);
    }
}
