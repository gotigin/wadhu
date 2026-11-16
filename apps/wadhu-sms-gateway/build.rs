//! Bake `env/development.env` or `env/production.env` (under this crate) into the binary.
//!
//! Selection:
//! - `WADHU_ENV=development|production` if set (scripts export this)
//! - else Cargo `PROFILE=release` → production, otherwise development

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Used when the env file is missing or omits a key. Keep in step with env/*.env.
fn defaults(profile: &str) -> HashMap<&'static str, &'static str> {
    match profile {
        "production" => HashMap::from([
            ("WADHU_ENV", "production"),
            ("WADHU_APP_ID", "com.wadhu.smsgateway"),
            ("WADHU_APP_LABEL", "SMS Gateway"),
        ]),
        _ => HashMap::from([
            ("WADHU_ENV", "development"),
            ("WADHU_APP_ID", "com.wadhu.smsgateway.debug"),
            ("WADHU_APP_LABEL", "SMS Gateway Dev"),
        ]),
    }
}

fn main() {
    // Tell rustc about custom cfgs set below (avoids unexpected_cfgs warnings).
    println!("cargo:rustc-check-cfg=cfg(wadhu_hide_console)");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let wadhu_env =
        env::var("WADHU_ENV").unwrap_or_else(|_| match env::var("PROFILE").as_deref() {
            Ok("release") => "production".into(),
            _ => "development".into(),
        });

    let env_path = manifest_dir.join("env").join(format!("{wadhu_env}.env"));
    println!("cargo:rerun-if-changed={}", env_path.display());
    println!("cargo:rerun-if-env-changed=WADHU_ENV");
    println!("cargo:rerun-if-env-changed=PROFILE");

    let mut vars: HashMap<String, String> = defaults(&wadhu_env)
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    if let Ok(text) = fs::read_to_string(&env_path) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            if !key.is_empty() {
                vars.insert(key.to_string(), value.to_string());
            }
        }
    } else {
        println!(
            "cargo:warning=env file missing at {} — using built-in {wadhu_env} defaults",
            env_path.display()
        );
    }

    // The ENV key always matches the selected profile, even if the file omits it.
    vars.insert("WADHU_ENV".into(), wadhu_env.clone());

    // Production Windows PE: no console when the user double-clicks the exe.
    // Dev builds keep the console for logs. Driven by WADHU_ENV, not the
    // Cargo profile (Windows dev packages also build --release).
    if wadhu_env == "production" {
        println!("cargo:rustc-cfg=wadhu_hide_console");
    }

    for (key, value) in vars {
        println!("cargo:rustc-env={key}={value}");
    }
}
