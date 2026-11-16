//! Connected Android devices (WebSocket sessions).

use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

pub type DeviceTx = Sender<String>;

#[derive(Clone, Default)]
pub struct Hub {
    inner: Arc<Mutex<HashMap<String, DeviceTx>>>,
}

impl Hub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, device_id: String) -> mpsc::Receiver<String> {
        let (tx, rx) = mpsc::channel();
        let mut g = self.inner.lock().expect("hub");
        g.insert(device_id, tx);
        rx
    }

    pub fn unregister(&self, device_id: &str) {
        let mut g = self.inner.lock().expect("hub");
        g.remove(device_id);
    }

    pub fn send_job(&self, device_id: &str, json: String) -> Result<(), String> {
        let g = self.inner.lock().expect("hub");
        let tx = g
            .get(device_id)
            .ok_or_else(|| "device_offline".to_string())?;
        tx.send(json).map_err(|_| "device_offline".to_string())
    }

    pub fn online(&self) -> Vec<String> {
        self.inner.lock().expect("hub").keys().cloned().collect()
    }

    pub fn is_online(&self, device_id: &str) -> bool {
        self.inner.lock().expect("hub").contains_key(device_id)
    }
}
