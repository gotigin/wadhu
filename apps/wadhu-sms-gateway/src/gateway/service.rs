//! Running gateway: outbox + radio + LAN HTTP + Bluetooth + optional relay + drain.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::bluetooth::{self, BluetoothHandle};
use super::sync_lock::lock_mutex;
use super::clients::Clients;
use super::discover;
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
    /// Operator key (advanced). Prefer per-app tokens from pairing for brand apps.
    pub api_token: Arc<Mutex<String>>,
    pub clients: Clients,
    pub listen: SocketAddr,
    pub bluetooth: BluetoothHandle,
    pub relay: RelayHandle,
    pub(crate) persist_meta: Arc<Mutex<PersistMeta>>,
}

#[derive(Clone, Default)]
pub(crate) struct PersistMeta {
    pub(crate) relay_url: String,
    pub(crate) relay_device_id: String,
    pub(crate) relay_device_secret: String,
    pub(crate) bluetooth_enabled: bool,
}

pub struct Gateway {
    pub handle: GatewayHandle,
    discover_stop: Arc<AtomicBool>,
}

impl Gateway {
    pub fn start(listen: SocketAddr) -> Result<Self, String> {
        let loaded = persist::load();
        let api_token = if loaded.api_token.is_empty() {
            generate_token()
        } else {
            loaded.api_token.clone()
        };
        let api_token = Arc::new(Mutex::new(api_token));

        let outbox = Outbox::new();
        outbox.restore(loaded.messages.clone());
        let clients = Clients::from_list(loaded.clients.clone());

        let radio: Arc<dyn SmsRadio> = Arc::from(default_radio());
        let bluetooth_enabled = loaded.bluetooth_enabled;
        let bluetooth = bluetooth::spawn(
            outbox.clone(),
            Arc::clone(&api_token),
            clients.clone(),
            bluetooth_enabled,
        );

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
            api_token: Arc::clone(&api_token),
            clients: clients.clone(),
            listen,
            bluetooth,
            relay,
            persist_meta: Arc::clone(&persist_meta),
        };

        let token_p = Arc::clone(&api_token);
        let outbox_p = outbox.clone();
        let meta_p = Arc::clone(&persist_meta);
        let clients_p = clients.clone();
        outbox.set_on_change(Arc::new(move || {
            persist_now(&token_p, &outbox_p, &meta_p, &clients_p);
            makepad_widgets::makepad_platform::thread::SignalToUI::set_ui_signal();
        }));

        persist_now(&api_token, &outbox, &persist_meta, &clients);

        let bound = http::spawn(handle.clone(), listen)?;
        let mut handle = handle;
        handle.listen = bound;

        let discover_stop = Arc::new(AtomicBool::new(false));
        discover::spawn(bound.port(), Arc::clone(&discover_stop));

        let drain_outbox = outbox.clone();
        let drain_radio = Arc::clone(&radio);
        thread::Builder::new()
            .name("sms-gateway-drain".into())
            .spawn(move || drain_loop(drain_outbox, drain_radio))
            .map_err(|e| e.to_string())?;

        Ok(Self {
            handle,
            discover_stop,
        })
    }

    pub fn operator_token(&self) -> String {
        lock_mutex(&self.handle.api_token).clone()
    }

    pub fn begin_pairing(&self) -> String {
        let code = self.handle.clients.begin_pairing();
        self.persist();
        makepad_widgets::makepad_platform::thread::SignalToUI::set_ui_signal();
        code
    }

    pub fn set_client_blocked(&self, name: &str, blocked: bool) -> Result<(), String> {
        self.handle.clients.set_blocked(name, blocked)?;
        self.persist();
        makepad_widgets::makepad_platform::thread::SignalToUI::set_ui_signal();
        Ok(())
    }

    /// New operator key; clears paired apps (they must Allow again).
    pub fn rotate_operator_key(&self) -> String {
        let next = generate_token();
        *lock_mutex(&self.handle.api_token) = next.clone();
        self.handle.clients.revoke_all_tokens();
        self.persist();
        makepad_widgets::makepad_platform::thread::SignalToUI::set_ui_signal();
        next
    }

    pub fn persist(&self) {
        persist_now(
            &self.handle.api_token,
            &self.handle.outbox,
            &self.handle.persist_meta,
            &self.handle.clients,
        );
    }

    /// Update relay pairing and persist. Restarts are picked up by the relay loop.
    pub fn configure_relay(&self, url: String, device_id: String, device_secret: String) {
        {
            let mut meta = lock_mutex(&self.handle.persist_meta);
            meta.relay_url = url.clone();
            meta.relay_device_id = device_id.clone();
            meta.relay_device_secret = device_secret.clone();
        }
        self.handle.relay.update_config(RelayConfig {
            url,
            device_id,
            device_secret,
        });
        self.persist();
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
        let meta = lock_mutex(&self.handle.persist_meta).clone();
        PersistedState {
            api_token: self.operator_token(),
            messages: self.handle.outbox.snapshot_for_persist(500),
            relay_url: meta.relay_url,
            relay_device_id: meta.relay_device_id,
            relay_device_secret: meta.relay_device_secret,
            bluetooth_enabled: meta.bluetooth_enabled,
            clients: self.handle.clients.snapshot(),
        }
    }

    /// Stop discover / Bluetooth / relay worker loops.
    pub fn shutdown(&self) {
        self.discover_stop.store(true, Ordering::Relaxed);
        self.handle.bluetooth.stop();
        self.handle.relay.stop();
    }
}

impl Drop for Gateway {
    fn drop(&mut self) {
        self.persist();
        self.shutdown();
    }
}

fn persist_now(
    api_token: &Arc<Mutex<String>>,
    outbox: &Outbox,
    meta: &Arc<Mutex<PersistMeta>>,
    clients: &Clients,
) {
    let token = lock_mutex(api_token).clone();
    let meta = lock_mutex(meta).clone();
    persist::save_from_outbox(
        &token,
        outbox,
        &meta.relay_url,
        &meta.relay_device_id,
        &meta.relay_device_secret,
        meta.bluetooth_enabled,
        &clients.snapshot(),
    );
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
