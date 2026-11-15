//! Wadhu Connect — WhatsApp Cloud API Tech Provider bridge.
//!
//! Production host: https://connect.wadhu.com
//! Local: `cargo wadhu-connect-dev` → http://127.0.0.1:3847

mod config;
mod http;
mod meta;
mod store;

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

use config::Config;
use http::App;
use meta::MetaClient;
use store::Store;

fn main() {
    let app_root = app_root();
    let cfg = Config::from_env(&app_root);
    let bind: SocketAddr = format!("{}:{}", cfg.host, cfg.port)
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("invalid HOST/PORT: {e}");
            std::process::exit(2);
        });

    let store = match Store::open(&cfg.data_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("data dir {}: {e}", cfg.data_dir.display());
            std::process::exit(1);
        }
    };
    let meta = MetaClient::new(&cfg);
    let app = App { cfg, store, meta };

    if let Err(e) = http::serve(app, bind) {
        eprintln!("server error: {e}");
        std::process::exit(1);
    }
}

fn app_root() -> PathBuf {
    if let Ok(p) = env::var("WADHU_CONNECT_ROOT") {
        return PathBuf::from(p);
    }
    // When run via `cargo run -p wadhu-connect`, CARGO_MANIFEST_DIR is the app.
    if let Ok(m) = env::var("CARGO_MANIFEST_DIR") {
        return PathBuf::from(m);
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}
