//! Async HTTP via browser `fetch` (Chrome / Firefox / Safari / Edge).

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::{Client, ClientError};

pub(crate) async fn request(
    client: &Client,
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<String, ClientError> {
    let url = format!("{}{path}", client.base);
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_mode(RequestMode::Cors);
    if let Some(body) = body {
        opts.set_body(&wasm_bindgen::JsValue::from_str(body));
    }

    let request = Request::new_with_str_and_init(&url, &opts).map_err(js_err)?;
    let headers = request.headers();
    headers
        .set("Authorization", &format!("Bearer {}", client.token))
        .map_err(js_err)?;
    headers
        .set("X-Api-Key", &client.token)
        .map_err(js_err)?;
    if body.is_some() {
        headers
            .set("Content-Type", "application/json")
            .map_err(js_err)?;
    }

    let window = web_sys::window().ok_or_else(|| ClientError::Io("no window".into()))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(js_err)?;
    let resp: Response = resp_value.dyn_into().map_err(|_| {
        ClientError::Io("fetch did not return Response".into())
    })?;
    let status = resp.status();
    let text = JsFuture::from(resp.text().map_err(js_err)?)
        .await
        .map_err(js_err)?;
    let body = text.as_string().unwrap_or_default();
    if !(200..300).contains(&status) {
        return Err(ClientError::Api { status, body });
    }
    Ok(body)
}

fn js_err(e: wasm_bindgen::JsValue) -> ClientError {
    ClientError::Io(e.as_string().unwrap_or_else(|| format!("{e:?}")))
}
