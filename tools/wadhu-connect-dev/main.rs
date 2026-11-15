//! Thin cargo entrypoint for `cargo wadhu-connect-dev`.

use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("tools/wadhu-connect-dev → workspace root");
    let app = root.join("apps").join("wadhu-connect");
    let data = root.join("target").join("wadhu-connect-data");
    let _ = std::fs::create_dir_all(&data);

    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("-p").arg("wadhu-connect").arg("--");
    cmd.args(env::args().skip(1));
    cmd.current_dir(root);
    cmd.env("WADHU_CONNECT_ROOT", &app);
    cmd.env("CARGO_MANIFEST_DIR", &app);
    if env::var_os("DATA_DIR").is_none() {
        cmd.env("DATA_DIR", &data);
    }
    if env::var_os("HOST").is_none() {
        cmd.env("HOST", "127.0.0.1");
    }
    if env::var_os("PORT").is_none() {
        cmd.env("PORT", "3847");
    }
    // Load app .env by running with cwd semantics — Config also reads apps/wadhu-connect/.env
    match cmd.status() {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(err) => {
            eprintln!("failed to run wadhu-connect: {err}");
            exit(1);
        }
    }
}
