//! Wadhu SMS Relay — self-hostable remote control plane for sms-gateway devices.

mod hub;
mod server;
mod store;

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

use hub::Hub;
use server::App;
use store::Store;

fn main() {
    let bind: SocketAddr = env::var("SMS_RELAY_BIND")
        .unwrap_or_else(|_| "0.0.0.0:9876".into())
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("invalid SMS_RELAY_BIND: {e}");
            std::process::exit(2);
        });

    let data_dir = env::var("SMS_RELAY_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| default_data_dir());

    let admin_token = env::var("SMS_RELAY_ADMIN_TOKEN").unwrap_or_else(|_| {
        let t = format!("admin-{:x}", now_ns());
        eprintln!("SMS_RELAY_ADMIN_TOKEN not set — generated ephemeral admin token:");
        eprintln!("  {t}");
        eprintln!("Set SMS_RELAY_ADMIN_TOKEN for stable admin access across restarts.");
        t
    });

    let store = match Store::open(&data_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to open data dir {}: {e}", data_dir.display());
            std::process::exit(1);
        }
    };

    eprintln!("data dir: {}", data_dir.display());
    let app = App {
        store,
        hub: Hub::new(),
        admin_token,
    };

    if let Err(e) = server::serve(app, bind) {
        eprintln!("server error: {e}");
        std::process::exit(1);
    }
}

fn default_data_dir() -> PathBuf {
    if let Ok(xdg) = env::var("XDG_DATA_HOME") {
        return PathBuf::from(xdg).join("wadhu-sms-relay");
    }
    if let Ok(home) = env::var("HOME") {
        return PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("wadhu-sms-relay");
    }
    PathBuf::from("./wadhu-sms-relay-data")
}

fn now_ns() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(1)
}
