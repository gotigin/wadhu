//! Env config for Wadhu Connect. Secrets never go to the browser.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Config {
    pub app_id: String,
    pub app_secret: String,
    pub config_id: String,
    pub graph_version: String,
    pub verify_token: String,
    pub public_origin: String,
    pub host: String,
    pub port: u16,
    pub data_dir: PathBuf,
    pub public_dir: PathBuf,
}

impl Config {
    pub fn from_env(app_root: &Path) -> Self {
        load_dotenv(&app_root.join(".env"));

        let data_dir = env::var("DATA_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| app_root.join("data"));
        let public_dir = env::var("PUBLIC_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| app_root.join("public"));
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3847);
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let public_origin = env::var("PUBLIC_ORIGIN")
            .unwrap_or_else(|_| format!("http://{host}:{port}"));

        Self {
            app_id: env::var("META_APP_ID").unwrap_or_default(),
            app_secret: env::var("META_APP_SECRET").unwrap_or_default(),
            config_id: env::var("META_CONFIG_ID").unwrap_or_default(),
            graph_version: env::var("META_GRAPH_VERSION").unwrap_or_else(|_| "v22.0".into()),
            verify_token: env::var("WEBHOOK_VERIFY_TOKEN")
                .unwrap_or_else(|_| "wadhu_connect_verify_change_me".into()),
            public_origin,
            host,
            port,
            data_dir,
            public_dir,
        }
    }

    pub fn ready(&self) -> bool {
        !self.app_id.is_empty() && !self.app_secret.is_empty() && !self.config_id.is_empty()
    }

    pub fn missing(&self) -> Vec<&'static str> {
        let mut m = Vec::new();
        if self.app_id.is_empty() {
            m.push("META_APP_ID");
        }
        if self.app_secret.is_empty() {
            m.push("META_APP_SECRET");
        }
        if self.config_id.is_empty() {
            m.push("META_CONFIG_ID");
        }
        m
    }
}

fn load_dotenv(path: &Path) {
    let Ok(raw) = fs::read_to_string(path) else {
        return;
    };
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(eq) = trimmed.find('=') else {
            continue;
        };
        let key = trimmed[..eq].trim();
        let mut val = trimmed[eq + 1..].trim().to_string();
        if (val.starts_with('"') && val.ends_with('"'))
            || (val.starts_with('\'') && val.ends_with('\''))
        {
            val = val[1..val.len() - 1].to_string();
        }
        if env::var_os(key).is_none() {
            // SAFETY: single-threaded startup before serving.
            unsafe { env::set_var(key, val) };
        }
    }
}
