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

    /// App Review / Allowed usage Graph probes (User token).
    pub fn me_businesses(&self, token: &str) -> GraphResult {
        let url = format!("{}/me/businesses?fields=id,name", self.base());
        get_json(&url, Some(token))
    }

    pub fn me_accounts(&self, token: &str) -> GraphResult {
        let url = format!("{}/me/accounts?fields=id,name,access_token", self.base());
        get_json(&url, Some(token))
    }

    pub fn page_posts(&self, page_id: &str, page_token: &str) -> GraphResult {
        let url = format!(
            "{}/{}/posts?fields=id,created_time&limit=3",
            self.base(),
            page_id
        );
        get_json(&url, Some(page_token))
    }

    /// Run BM + Pages list + Page posts. Returns status codes only (no tokens).
    pub fn app_review_precheck(&self, user_token: &str, preferred_page_id: Option<&str>) -> Value {
        let bm = self.me_businesses(user_token);
        let accounts = self.me_accounts(user_token);

        let mut page_id = preferred_page_id.map(|s| s.to_string());
        let mut page_token: Option<String> = None;
        if let Some(data) = accounts.json.get("data").and_then(|d| d.as_array()) {
            if page_id.is_none() {
                if let Some(first) = data.first() {
                    page_id = first.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());
                }
            }
            if let Some(want) = &page_id {
                for p in data {
                    if p.get("id").and_then(|v| v.as_str()) == Some(want.as_str()) {
                        page_token = p
                            .get("access_token")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        break;
                    }
                }
            }
            if page_token.is_none() {
                if let Some(first) = data.first() {
                    page_id = first.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());
                    page_token = first
                        .get("access_token")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                }
            }
        }

        let posts = match (&page_id, &page_token) {
            (Some(pid), Some(pt)) => self.page_posts(pid, pt),
            _ => GraphResult {
                ok: false,
                status: 0,
                json: json!({ "error": "no_page_token_from_me_accounts" }),
            },
        };

        json!({
            "business_management": {
                "endpoint": "GET /me/businesses",
                "http": bm.status,
                "ok": bm.ok,
                "error": bm.json.get("error").cloned().unwrap_or(Value::Null),
            },
            "pages_show_list": {
                "endpoint": "GET /me/accounts",
                "http": accounts.status,
                "ok": accounts.ok,
                "pageCount": accounts.json.get("data").and_then(|d| d.as_array()).map(|a| a.len()).unwrap_or(0),
                "error": accounts.json.get("error").cloned().unwrap_or(Value::Null),
            },
            "pages_read_engagement": {
                "endpoint": "GET /{page-id}/posts",
                "pageId": page_id,
                "http": posts.status,
                "ok": posts.ok,
                "error": posts.json.get("error").cloned().unwrap_or(Value::Null),
            },
        })
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
