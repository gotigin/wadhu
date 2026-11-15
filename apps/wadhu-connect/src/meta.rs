//! Meta Graph API helpers. App secret stays server-side.

use serde_json::{Value, json};

use crate::config::Config;

pub struct MetaClient {
    app_id: String,
    app_secret: String,
    graph_version: String,
}

impl MetaClient {
    pub fn new(cfg: &Config) -> Self {
        Self {
            app_id: cfg.app_id.clone(),
            app_secret: cfg.app_secret.clone(),
            graph_version: cfg.graph_version.clone(),
        }
    }

    fn base(&self) -> String {
        format!("https://graph.facebook.com/{}", self.graph_version)
    }

    pub fn exchange_code(&self, code: &str, redirect_uri: Option<&str>) -> GraphResult {
        let mut url = format!(
            "{}/oauth/access_token?client_id={}&client_secret={}&code={}",
            self.base(),
            urlencoding(&self.app_id),
            urlencoding(&self.app_secret),
            urlencoding(code)
        );
        if let Some(ru) = redirect_uri {
            url.push_str("&redirect_uri=");
            url.push_str(&urlencoding(ru));
        }
        get_json(&url, None)
    }

    pub fn subscribe_app(&self, waba_id: &str, token: &str) -> GraphResult {
        let url = format!("{}/{}/subscribed_apps", self.base(), waba_id);
        post_json(&url, Some(token), &json!({}))
    }

    pub fn register_phone(&self, phone_number_id: &str, token: &str, pin: &str) -> GraphResult {
        let url = format!("{}/{}/register", self.base(), phone_number_id);
        post_json(
            &url,
            Some(token),
            &json!({
                "messaging_product": "whatsapp",
                "pin": pin,
            }),
        )
    }

    pub fn phone_status(&self, phone_number_id: &str, token: &str) -> GraphResult {
        let url = format!(
            "{}/{}?fields=display_phone_number,verified_name,quality_rating,platform_type,is_on_biz_app,code_verification_status,status",
            self.base(),
            phone_number_id
        );
        get_json(&url, Some(token))
    }

    pub fn send_text(
        &self,
        phone_number_id: &str,
        token: &str,
        to: &str,
        body: &str,
    ) -> GraphResult {
        let url = format!("{}/{}/messages", self.base(), phone_number_id);
        post_json(
            &url,
            Some(token),
            &json!({
                "messaging_product": "whatsapp",
                "recipient_type": "individual",
                "to": to,
                "type": "text",
                "text": { "preview_url": false, "body": body },
            }),
        )
    }

    pub fn request_history_sync(&self, phone_number_id: &str, token: &str) -> GraphResult {
        let url = format!("{}/{}/smb_app_data", self.base(), phone_number_id);
        post_json(
            &url,
            Some(token),
            &json!({
                "messaging_product": "whatsapp",
                "sync_type": "smb_app_state_sync",
            }),
        )
    }
}

#[derive(Debug)]
pub struct GraphResult {
    pub ok: bool,
    pub status: u16,
    pub json: Value,
}

fn get_json(url: &str, token: Option<&str>) -> GraphResult {
    let mut req = ureq::get(url);
    if let Some(t) = token {
        req = req.set("Authorization", &format!("Bearer {t}"));
    }
    finish(req.call())
}

fn post_json(url: &str, token: Option<&str>, body: &Value) -> GraphResult {
    let mut req = ureq::post(url).set("Content-Type", "application/json");
    if let Some(t) = token {
        req = req.set("Authorization", &format!("Bearer {t}"));
    }
    finish(req.send_json(body.clone()))
}

fn finish(res: Result<ureq::Response, ureq::Error>) -> GraphResult {
    match res {
        Ok(r) => {
            let status = r.status();
            let json: Value = r.into_json().unwrap_or_else(|_| json!({}));
            GraphResult {
                ok: (200..300).contains(&status),
                status,
                json,
            }
        }
        Err(ureq::Error::Status(status, r)) => {
            let json: Value = r.into_json().unwrap_or_else(|_| json!({}));
            GraphResult {
                ok: false,
                status,
                json,
            }
        }
        Err(e) => GraphResult {
            ok: false,
            status: 0,
            json: json!({ "error": e.to_string() }),
        },
    }
}

fn urlencoding(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
