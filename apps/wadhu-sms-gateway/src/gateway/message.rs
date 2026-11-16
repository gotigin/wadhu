//! Outbound SMS message types for the gateway API.

use std::time::{SystemTime, UNIX_EPOCH};

/// Lifecycle of one accepted message (see `docs/LOCAL_SMS.md`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Queued,
    Submitting,
    Submitted,
    Delivered,
    Failed,
    Expired,
    Cancelled,
    Indeterminate,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Submitting => "submitting",
            Self::Submitted => "submitted",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: String,
    pub idempotency_key: String,
    pub to: String,
    pub body: String,
    pub status: Status,
    pub error: Option<String>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
}

impl Message {
    pub fn new(idempotency_key: String, to: String, body: String) -> Self {
        let now = now_ms();
        Self {
            id: new_id(),
            idempotency_key,
            to,
            body,
            status: Status::Queued,
            error: None,
            created_at_ms: now,
            updated_at_ms: now,
        }
    }
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn new_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(1);
    let n = SEQ.fetch_add(1, Ordering::Relaxed);
    format!("msg-{}-{n}", now_ms())
}
