//! SMS Gateway — the non-UI half (outbox, radio, local HTTP API).
//!
//! This app **is** the gateway other apps connect to. See
//! `docs/SMS_GATEWAY.md` and `docs/LOCAL_SMS.md`.

#[cfg(target_os = "android")]
pub use makepad_widgets;

pub mod gateway;
pub mod identity;
pub mod permissions;

// Android packaging compiles this library as the cdylib.
#[cfg(target_os = "android")]
#[path = "main.rs"]
mod shell;
