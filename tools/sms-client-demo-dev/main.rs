//! Thin cargo entrypoint for `cargo sms-client-demo-dev`.
//! Delegates to `apps/sms-client-demo/scripts/dev.sh` (loads apps/sms-client-demo/env/development.env).

use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("tools/sms-client-demo-dev → workspace root");
    let script = root.join("apps/sms-client-demo/scripts/dev.sh");
    let mut cmd = Command::new("bash");
    cmd.arg(&script);
    cmd.args(env::args().skip(1));
    cmd.current_dir(root);
    match cmd.status() {
        Ok(status) => exit(status.code().unwrap_or(1)),
        Err(err) => {
            eprintln!("failed to run {}: {err}", script.display());
            exit(1);
        }
    }
}
