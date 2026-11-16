//! SMS Gateway service: outbox, radio, LAN HTTP, Bluetooth framing, optional relay.

pub mod bluetooth;
pub mod clients;
pub mod discover;
pub mod frame;
pub mod http;
pub mod message;
pub mod outbox;
pub mod persist;
pub mod radio;
pub mod relay_client;
pub mod service;
pub mod ws;

pub use service::{Gateway, GatewayHandle};
