//! Running gateway: outbox + radio + LAN HTTP + Bluetooth + optional relay + drain.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::bluetooth::{self, BluetoothHandle};
use super::http;
use super::message::Status;
use super::outbox::Outbox;
use super::persist::{self, PersistedState};
use super::radio::{default_radio, SmsRadio};
use super::relay_client::{self, RelayConfig, RelayHandle};

#[derive(Clone)]
pub struct GatewayHandle {
    pub outbox: Outbox,
    pub radio: Arc<dyn SmsRadio>,
    pub api_token: String,
    pub listen: SocketAddr,
    pub bluetooth: BluetoothHandle,
    pub relay: RelayHandle,
    persist_meta: Arc<Mutex<PersistMeta>>,
}

#[derive(Clone, Default)]
struct PersistMeta {
    relay_url: String,
    relay_device_id: String,
    relay_device_secret: String,
    bluetooth_enabled: bool,
}

pub struct Gateway {
    pub handle: GatewayHandle,
}

impl Gateway {
    pub fn start(listen: SocketAddr) -> Result<Self, String> {
        let loaded = persist::load();
        let api_token = if loaded.api_token.is_empty() {
            generate_token()
        } else {
            loaded.api_token.clone()
        };

        let outbox = Outbox::new();
        outbox.restore(loaded.messages.clone());

        let radio: Arc<dyn SmsRadio> = Arc::from(default_radio());
        let bluetooth_enabled = loaded.bluetooth_enabled;
        let bluetooth = bluetooth::spawn(outbox.clone(), api_token.clone(), bluetooth_enabled);

        let relay_cfg = RelayConfig {
            url: loaded.relay_url.clone(),
            device_id: loaded.relay_device_id.clone(),
            device_secret: loaded.relay_device_secret.clone(),
        };
        let relay = relay_client::spawn(outbox.clone(), relay_cfg);

        let persist_meta = Arc::new(Mutex::new(PersistMeta {
            relay_url: loaded.relay_url,
            relay_device_id: loaded.relay_device_id,
            relay_device_secret: loaded.relay_device_secret,
            bluetooth_enabled,
        }));

        let handle = GatewayHandle {
            outbox: outbox.clone(),
            radio: Arc::clone(&radio),
            api_token: api_token.clone(),
            listen,
            bluetooth,
            relay,
            persist_meta: Arc::clone(&persist_meta),
        };

        // Persist on outbox changes and wake the UI so status chips / Recent
        // update as soon as drain or LAN/BT accept mutates the queue.
        let token_p = api_token.clone();
        let outbox_p = outbox.clone();
        let meta_p = Arc::clone(&persist_meta);
        outbox.set_on_change(Arc::new(move || {
            let meta = meta_p.lock().expect("persist meta").clone();
            persist::save_from_outbox(
                &token_p,
                &outbox_p,
                &meta.relay_url,
                &meta.relay_device_id,
                &meta.relay_device_secret,
                meta.bluetooth_enabled,
            );
            makepad_widgets::makepad_platform::thread::SignalToUI::set_ui_signal();
        }));

        // Initial save so token lands on disk even with empty outbox.
        {
            let meta = persist_meta.lock().expect("persist meta").clone();
            persist::save_from_outbox(
                &api_token,
                &outbox,
                &meta.relay_url,
                &meta.relay_device_id,
                &meta.relay_device_secret,
                meta.bluetooth_enabled,
            );
        }

        let bound = http::spawn(handle.clone(), listen)?;
        let mut handle = handle;
        handle.listen = bound;

        let drain_outbox = outbox.clone();
        let drain_radio = Arc::clone(&radio);
        thread::Builder::new()
            .name("sms-gateway-drain".into())
            .spawn(move || drain_loop(drain_outbox, drain_radio))
            .map_err(|e| e.to_string())?;

        Ok(Self { handle })
    }

    /// Update relay pairing and persist. Restarts are picked up by the relay loop.
    pub fn configure_relay(&self, url: String, device_id: String, device_secret: String) {
        {
            let mut meta = self.handle.persist_meta.lock().expect("persist meta");
            meta.relay_url = url.clone();
            meta.relay_device_id = device_id.clone();
            meta.relay_device_secret = device_secret.clone();
        }
        self.handle.relay.update_config(RelayConfig {
            url: url.clone(),
            device_id: device_id.clone(),
            device_secret: device_secret.clone(),
        });
        persist::save_from_outbox(
            &self.handle.api_token,
            &self.handle.outbox,
            &url,
            &device_id,
            &device_secret,
            self.handle
                .persist_meta
                .lock()
                .expect("meta")
                .bluetooth_enabled,
        );
    }

    pub fn summary_line(&self) -> String {
        let caps = self.handle.radio.capabilities();
        let (q, s, f) = self.handle.outbox.counts();
        let bt = self.handle.bluetooth.snapshot();
        let relay = self.handle.relay.snapshot();
        format!(
            "API http://{}  ·  radio {} ({})  ·  bt {}  ·  relay {}  ·  queue q={q} ok={s} fail={f}",
            self.handle.listen,
            caps.kind,
            if caps.available { "up" } else { "down" },
            if bt.listening { "up" } else { "down" },
            if relay.connected { "up" } else { "down" },
        )
    }

    pub fn persisted_snapshot(&self) -> PersistedState {
        let meta = self.handle.persist_meta.lock().expect("meta").clone();
        PersistedState {
            api_token: self.handle.api_token.clone(),
            messages: {
                let mut m = self.handle.outbox.list(500);
                m.reverse();
                m
            },
            relay_url: meta.relay_url,
            relay_device_id: meta.relay_device_id,
            relay_device_secret: meta.relay_device_secret,
            bluetooth_enabled: meta.bluetooth_enabled,
        }
    }
}

fn drain_loop(outbox: Outbox, radio: Arc<dyn SmsRadio>) {
    loop {
        if let Some(msg) = outbox.next_queued() {
            outbox.set_status(&msg.id, Status::Submitting, None);
            match radio.send(&msg.to, &msg.body) {
                Ok(()) => {
                    outbox.set_status(&msg.id, Status::Submitted, None);
                }
                Err(e) => {
                    outbox.set_status(&msg.id, Status::Failed, Some(humanize_send_error(&e)));
                }
            }
            // Drain the rest of the queue without sleeping between jobs.
            continue;
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn humanize_send_error(raw: &str) -> String {
    let lower = raw.to_ascii_lowercase();
    if lower.contains("permission") {
        return "SMS permission not granted — tap Allow SMS in the app".into();
    }
    if lower.contains("sim_not_ready") || lower.contains("sim not ready") {
        return format!("SIM not ready ({raw}). Check SIM / airplane mode / signal");
    }
    if lower.contains("no_telephony") {
        return "This device has no telephony (tablet without SIM?)".into();
    }
    if lower.contains("refused") {
        return "Phone radio refused the send (permission, SIM, or carrier)".into();
    }
    if lower.contains("not running on android") || lower.contains("only available on android") {
        return "Desktop build has no SIM — run on Android to send".into();
    }
    raw.to_string()
}

fn generate_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(1);
    format!("wadhu-sms-{t:x}")
}
