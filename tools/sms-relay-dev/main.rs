//! Thin cargo entrypoint for `cargo sms-relay-dev`.
//! Runs `wadhu-sms-relay` with a local data dir under the workspace.

use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("tools/sms-relay-dev → workspace root");
    let data = root.join("target").join("sms-relay-data");
    let _ = std::fs::create_dir_all(&data);

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("-p")
        .arg("wadhu-sms-relay")
        .arg("--");
    cmd.args(env::args().skip(1));
    cmd.current_dir(root);
    if env::var_os("SMS_RELAY_DATA").is_none() {
        cmd.env("SMS_RELAY_DATA", &data);
    }
    if env::var_os("SMS_RELAY_BIND").is_none() {
        cmd.env("SMS_RELAY_BIND", "127.0.0.1:9876");
    }
    if env::var_os("SMS_RELAY_ADMIN_TOKEN").is_none() {
        cmd.env("SMS_RELAY_ADMIN_TOKEN", "dev-admin-token");
    }
    match cmd.status() {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(err) => {
            eprintln!("failed to run wadhu-sms-relay: {err}");
            exit(1);
        }
    }
}
