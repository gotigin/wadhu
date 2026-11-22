// Production Windows: GUI subsystem (no console). Dev keeps a console for logs.
#![cfg_attr(
    all(target_os = "windows", wadhu_hide_console),
    windows_subsystem = "windows"
)]

//! SMS Client Demo — desktop integrator shell for `wadhu-sms-client`.
//!
//! Talks only to the crate (LAN gateway or relay). No radio / BT code here.

pub use makepad_widgets;

#[cfg(target_os = "android")]
use crate::identity;
#[cfg(not(target_os = "android"))]
use wadhu_sms_client_demo::identity;

use makepad_widgets::*;
use wadhu_sms_client::framed::FramedClient;
use wadhu_sms_client::{AcceptRequest, Client};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    let FieldInput = TextInput{
        width: Fill
        height: 56.
        margin: Inset{left: 0. right: 0. top: 10. bottom: 10.}
        padding: Inset{left: 18. right: 18. top: 16. bottom: 14.}
        flow: Right
        is_multiline: false
        blink_speed: 0.5
        draw_bg +: {
            color: #x0b1f63
            color_hover: #x0133ad
            color_focus: #x0133ad
            color_empty: #x0b1f63
            border_radius: 14.
            border_size: 1.
            border_color: #x161252
            border_color_focus: #x00c8fc
            border_color_empty: #x161252
        }
        draw_text +: {
            color: #xfcfbfb
            color_empty: #x9fb4d0
            max_lines: 1
            text_overflow: Ellipsis
            text_style +: {font_size: 15.}
        }
    }

    let PrimaryBtn = Button{
        width: Fill
        height: 56.
        margin: Inset{left: 0. right: 0. top: 12. bottom: 8.}
        padding: Inset{left: 18. right: 18. top: 14. bottom: 14.}
        draw_bg +: {
            color: #xadfb15
            color_hover: #x00c8fc
            color_down: #x0fca65
            border_radius: 14.
            border_size: 0.
        }
        // Dark ink on every face: the theme's hover ink is pale and
        // vanished on the cyan hover fill.
        draw_text +: {
            color: #x0d0b3a
            color_hover: #x0d0b3a
            color_down: #x0d0b3a
            color_focus: #x0d0b3a
            text_style +: {font_size: 15.}
        }
    }

    let GhostBtn = Button{
        width: Fill
        height: 56.
        margin: Inset{left: 0. right: 0. top: 12. bottom: 8.}
        padding: Inset{left: 18. right: 18. top: 14. bottom: 14.}
        draw_bg +: {
            color: #x0b1f63
            color_hover: #x0133ad
            border_radius: 14.
            border_size: 0.
        }
        draw_text +: {
            color: #xfcfbfb
            color_hover: #xfcfbfb
            color_down: #xfcfbfb
            color_focus: #xfcfbfb
            text_style +: {font_size: 14.}
        }
    }

    let BodyCopy = Markdown{
        width: Fill
        height: Fit
        body: ""
        font_size: 14.
        font_color: #x9fb4d0
        paragraph_spacing: 0.
        flow: Right{wrap: true}
        selectable: false
        draw_text +: {color: #x9fb4d0}
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "SMS Client Demo"
                window.inner_size: vec2(520, 780)
                body +: {
                    SolidView{
                        width: Fill
                        height: Fill
                        flow: Down
                        draw_bg +: {color: #x0d0b3a}

                        SolidView{
                            width: Fill
                            height: Fit
                            flow: Down
                            spacing: 8
                            padding: Inset{left: 28. right: 28. top: 28. bottom: 16.}
                            Label{
                                width: Fill
                                height: Fit
                                text: "SMS Client Demo"
                                draw_text.color: #xfcfbfb
                                draw_text.text_style.font_size: 28
                            }
                            Label{
                                width: Fill
                                height: Fit
                                text: "Desktop integrator for wadhu-sms-client"
                                draw_text.color: #xadfb15
                                draw_text.text_style.font_size: 13
                            }
                            identity_label := Label{
                                width: Fill
                                height: Fit
                                text: ""
                                draw_text.color: #x9fb4d0
                                draw_text.text_style.font_size: 11
                            }
                        }

                        ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down
                            SolidView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 28
                                padding: Inset{left: 28. right: 28. top: 12. bottom: 64.}

                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "CONNECT TO PHONE (LAN)"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8
                                        padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        BodyCopy{
                                            body: "On the phone: SMS Gateway → Allow a new app → read the code. Same Wi-Fi. No IP typing."
                                        }
                                        pair_code_input := FieldInput{
                                            empty_text: "6-digit code"
                                        }
                                        connect_lan_button := PrimaryBtn{
                                            text: "Find phone & connect"
                                        }
                                        connect_label := BodyCopy{
                                            body: "Not connected yet."
                                        }
                                    }
                                }

                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "ADVANCED / RELAY"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8
                                        padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        BodyCopy{
                                            body: "Filled automatically after Find phone & connect. Or paste relay URL + key for internet."
                                        }
                                        url_input := FieldInput{
                                            empty_text: "Gateway or relay URL"
                                        }
                                        key_input := FieldInput{
                                            empty_text: "App token or relay key"
                                        }
                                        health_button := GhostBtn{
                                            text: "Check health"
                                        }
                                        health_label := BodyCopy{
                                            body: "Not checked yet."
                                        }
                                    }
                                }

                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "SEND"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8
                                        padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        to_input := FieldInput{
                                            empty_text: "To (+country number)"
                                        }
                                        body_input := FieldInput{
                                            empty_text: "Message"
                                        }
                                        send_button := PrimaryBtn{
                                            text: "Send via HTTP"
                                        }
                                        result_label := Label{
                                            width: Fill
                                            height: Fit
                                            margin: Inset{top: 4. bottom: 0.}
                                            text: ""
                                            draw_text.color: #xadfb15
                                            draw_text.text_style.font_size: 13
                                        }
                                    }
                                }

                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "FRAMED TCP (BT LAB)"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8
                                        padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        BodyCopy{
                                            body: "No cloud. Run SMS Gateway on this PC — it listens on 127.0.0.1:18766 with the same length-prefixed JSON as Classic RFCOMM. Paste the gateway API key above, then ping/send here. Phone RFCOMM uses UUID 00001101-… (see BLUETOOTH.md)."
                                        }
                                        framed_host_input := FieldInput{
                                            empty_text: "Host (127.0.0.1)"
                                        }
                                        framed_ops := View{
                                            width: Fill height: Fit
                                            flow: Down
                                            spacing: 10.
                                            framed_ping_button := GhostBtn{
                                                text: "Framed ping :18766"
                                            }
                                            framed_send_button := PrimaryBtn{
                                                text: "Send via framed TCP"
                                            }
                                        }
                                        framed_label := Label{
                                            width: Fill
                                            height: Fit
                                            margin: Inset{top: 4. bottom: 0.}
                                            text: ""
                                            draw_text.color: #xadfb15
                                            draw_text.text_style.font_size: 13
                                        }
                                    }
                                }

                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "HOW TO INTEGRATE"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8
                                        padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        BodyCopy{
                                            body: "Depend on crates/wadhu-sms-client. HTTP Client for phone LAN and sms-relay. FramedClient for BT lab TCP. Production nearby-device RFCOMM is implemented in brand apps with the same framing. Native = sync; wasm = *_async + fetch (HTTP only)."
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Content width where framed ping/send Fit actions sit on one row.
const FRAMED_ONE_ROW_FROM: f64 = 720.0;

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    poll_timer: Timer,
    #[rust]
    last_id: Option<String>,
    #[rust]
    last_base: String,
    #[rust]
    last_token: String,
    /// When true, poll every ~300ms until status leaves queued/submitting.
    #[rust]
    watch_active: bool,
    #[rust]
    laid_out_w: Option<i64>,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        log!("{}", identity::summary());
        self.ui
            .window(cx, ids!(main_window))
            .set_title(cx, identity::APP_LABEL);
        self.ui.label(cx, ids!(identity_label)).set_text(
            cx,
            &format!("{} · {} · desktop demo", identity::APP_ID, identity::ENV),
        );
        // Fast interval while watching a send; cheap no-op when idle.
        self.poll_timer = cx.start_interval(0.3);
    }

    fn handle_timer(&mut self, cx: &mut Cx, e: &TimerEvent) {
        if self.poll_timer.is_timer(e).is_none() {
            return;
        }
        if !self.watch_active {
            return;
        }
        let Some(id) = self.last_id.clone() else {
            self.watch_active = false;
            return;
        };
        if self.last_base.is_empty() || self.last_token.is_empty() {
            self.watch_active = false;
            return;
        }
        let client = Client::new(&self.last_base, &self.last_token);
        match client.get_message(&id) {
            Ok(m) => {
                let (text, color) = status_line(&m.status, &m.to, m.error.as_deref());
                self.ui.label(cx, ids!(result_label)).set_text(cx, &text);
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text_color(cx, color);
                if is_terminal_status(&m.status) {
                    self.watch_active = false;
                }
            }
            Err(e) => {
                self.ui.label(cx, ids!(result_label)).set_text(
                    cx,
                    &format!("Could not refresh status — {}", e.user_message()),
                );
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text_color(cx, color_fail());
                // Keep watching briefly; one blip should not freeze the UI forever.
            }
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(connect_lan_button)).clicked(actions) {
            let code = self.ui.text_input(cx, ids!(pair_code_input)).text();
            if code.trim().len() < 4 {
                self.ui.markdown(cx, ids!(connect_label)).set_text(
                    cx,
                    "Enter the code shown on the phone (Allow a new app).",
                );
                return;
            }
            self.ui
                .markdown(cx, ids!(connect_label))
                .set_text(cx, "Looking for the phone on this Wi-Fi…");
            match Client::connect_on_lan(code.trim(), "SMS Client Demo") {
                Ok(client) => {
                    self.ui
                        .text_input(cx, ids!(url_input))
                        .set_text(cx, client.base_url());
                    self.ui
                        .text_input(cx, ids!(key_input))
                        .set_text(cx, client.token());
                    self.ui.markdown(cx, ids!(connect_label)).set_text(
                        cx,
                        &format!("Connected — {}", client.base_url()),
                    );
                    self.ui.markdown(cx, ids!(health_label)).set_text(
                        cx,
                        "Connected via Allow code. You can Check health or Send.",
                    );
                }
                Err(e) => {
                    self.ui.markdown(cx, ids!(connect_label)).set_text(
                        cx,
                        &format!("Could not connect — {}", e.user_message()),
                    );
                }
            }
        }

        if self.ui.button(cx, ids!(health_button)).clicked(actions) {
            let (base, token) = self.read_endpoint(cx);
            if base.is_empty() || token.is_empty() {
                self.ui.markdown(cx, ids!(health_label)).set_text(
                    cx,
                    "Paste the phone Listen URL and API key from SMS Gateway first.",
                );
                return;
            }
            let client = Client::new(&base, &token);
            match client.health() {
                Ok(raw) => {
                    self.ui
                        .markdown(cx, ids!(health_label))
                        .set_text(cx, &format!("Connected — {}", summarize_health(&raw)));
                }
                Err(e) => {
                    self.ui.markdown(cx, ids!(health_label)).set_text(
                        cx,
                        &format!("Not connected — {}", e.user_message()),
                    );
                }
            }
        }

        if self.ui.button(cx, ids!(send_button)).clicked(actions) {
            let (base, token) = self.read_endpoint(cx);
            let to = self.ui.text_input(cx, ids!(to_input)).text();
            let body = self.ui.text_input(cx, ids!(body_input)).text();
            if base.is_empty() || token.is_empty() {
                self.ui.label(cx, ids!(result_label)).set_text(
                    cx,
                    "Paste phone Listen URL + API key first (not localhost unless gateway is on this PC)",
                );
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text_color(cx, color_fail());
                return;
            }
            if to.trim().is_empty() || body.trim().is_empty() {
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text(cx, "Enter destination and message");
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text_color(cx, color_fail());
                return;
            }
            self.ui
                .label(cx, ids!(result_label))
                .set_text(cx, "Contacting gateway…");
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
            let client = Client::new(&base, &token);
            match client.accept(&AcceptRequest {
                to: to.trim().into(),
                body: body.trim().into(),
                idempotency_key: None,
            }) {
                Ok(m) => {
                    self.last_id = Some(m.id.clone());
                    self.last_base = base;
                    self.last_token = token;
                    self.watch_active = !is_terminal_status(&m.status);
                    let (text, color) = status_line(&m.status, &m.to, m.error.as_deref());
                    self.ui.label(cx, ids!(result_label)).set_text(cx, &text);
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text_color(cx, color);
                }
                Err(e) => {
                    self.watch_active = false;
                    self.ui.label(cx, ids!(result_label)).set_text(
                        cx,
                        &format!("Send failed — {}", e.user_message()),
                    );
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text_color(cx, color_fail());
                }
            }
        }

        if self.ui.button(cx, ids!(framed_ping_button)).clicked(actions) {
            let token = self.ui.text_input(cx, ids!(key_input)).text().trim().to_string();
            let host = self.framed_host(cx);
            if token.is_empty() {
                self.ui.label(cx, ids!(framed_label)).set_text(
                    cx,
                    "Paste the gateway API key above first",
                );
                self.ui
                    .label(cx, ids!(framed_label))
                    .set_text_color(cx, color_fail());
                return;
            }
            match FramedClient::connect_tcp(&host, 18766, &token).and_then(|mut c| c.ping()) {
                Ok(()) => {
                    self.ui.label(cx, ids!(framed_label)).set_text(
                        cx,
                        &format!("Framed ping ok — {host}:18766 (no cloud)"),
                    );
                    self.ui
                        .label(cx, ids!(framed_label))
                        .set_text_color(cx, color_ok());
                }
                Err(e) => {
                    self.ui.label(cx, ids!(framed_label)).set_text(
                        cx,
                        &format!("Framed ping failed — {}", e.user_message()),
                    );
                    self.ui
                        .label(cx, ids!(framed_label))
                        .set_text_color(cx, color_fail());
                }
            }
        }

        if self.ui.button(cx, ids!(framed_send_button)).clicked(actions) {
            let token = self.ui.text_input(cx, ids!(key_input)).text().trim().to_string();
            let host = self.framed_host(cx);
            let to = self.ui.text_input(cx, ids!(to_input)).text();
            let body = self.ui.text_input(cx, ids!(body_input)).text();
            if token.is_empty() {
                self.ui.label(cx, ids!(framed_label)).set_text(
                    cx,
                    "Paste the gateway API key above first",
                );
                self.ui
                    .label(cx, ids!(framed_label))
                    .set_text_color(cx, color_fail());
                return;
            }
            if to.trim().is_empty() || body.trim().is_empty() {
                self.ui
                    .label(cx, ids!(framed_label))
                    .set_text(cx, "Enter destination and message above");
                self.ui
                    .label(cx, ids!(framed_label))
                    .set_text_color(cx, color_fail());
                return;
            }
            match FramedClient::connect_tcp(&host, 18766, &token).and_then(|mut c| {
                c.send(to.trim(), body.trim(), None)
            }) {
                Ok(reply) => {
                    self.ui.label(cx, ids!(framed_label)).set_text(
                        cx,
                        &format!("Framed send ok — {reply}"),
                    );
                    self.ui
                        .label(cx, ids!(framed_label))
                        .set_text_color(cx, color_ok());
                }
                Err(e) => {
                    self.ui.label(cx, ids!(framed_label)).set_text(
                        cx,
                        &format!("Framed send failed — {}", e.user_message()),
                    );
                    self.ui
                        .label(cx, ids!(framed_label))
                        .set_text_color(cx, color_fail());
                }
            }
        }
    }
}

impl App {
    fn read_endpoint(&self, cx: &mut Cx) -> (String, String) {
        let base = self.ui.text_input(cx, ids!(url_input)).text().trim().to_string();
        let token = self.ui.text_input(cx, ids!(key_input)).text().trim().to_string();
        (base, token)
    }

    fn framed_host(&self, cx: &mut Cx) -> String {
        let host = self
            .ui
            .text_input(cx, ids!(framed_host_input))
            .text()
            .trim()
            .to_string();
        if host.is_empty() {
            "127.0.0.1".into()
        } else {
            host
        }
    }
}

fn is_terminal_status(status: &str) -> bool {
    matches!(
        status,
        "submitted" | "delivered" | "failed" | "expired" | "cancelled" | "indeterminate"
    )
}

fn status_line(status: &str, to: &str, error: Option<&str>) -> (String, Vec4f) {
    match status {
        "queued" => (
            format!("Queued for {to} — waiting for the phone radio…"),
            color_lime(),
        ),
        "submitting" => (
            format!("Handing to SIM for {to}…"),
            color_lime(),
        ),
        "submitted" => (
            format!("Sent to {to} — SIM handed it to the carrier"),
            color_ok(),
        ),
        "delivered" => (format!("Delivered to {to}"), color_ok()),
        "failed" => (
            format!(
                "Failed for {to}: {}",
                error.unwrap_or("unknown radio/permission error")
            ),
            color_fail(),
        ),
        "indeterminate" => (
            format!("Uncertain for {to} — may have been interrupted mid-send"),
            color_muted(),
        ),
        other => (format!("{other} → {to}"), color_muted()),
    }
}

fn summarize_health(raw: &str) -> String {
    let radio = extract_json_boolish(raw, "available").unwrap_or("?");
    let pending = extract_json_number(raw, "pending").unwrap_or(0);
    let submitted = extract_json_number(raw, "submitted").unwrap_or(0);
    let failed = extract_json_number(raw, "failed").unwrap_or(0);
    format!("radio available={radio} · queue pending={pending} sent={submitted} failed={failed}")
}

fn extract_json_number(raw: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\":");
    let i = raw.find(&needle)?;
    let rest = raw[i + needle.len()..].trim_start();
    let end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

fn extract_json_boolish(raw: &str, key: &str) -> Option<&'static str> {
    let needle = format!("\"{key}\":");
    let i = raw.find(&needle)?;
    let rest = raw[i + needle.len()..].trim_start();
    if rest.starts_with("true") {
        Some("yes")
    } else if rest.starts_with("false") {
        Some("no")
    } else {
        None
    }
}

fn color_ok() -> Vec4f {
    hex_rgb(0x0f_ca_65)
}
fn color_fail() -> Vec4f {
    hex_rgb(0xd9_67_fb)
}
fn color_lime() -> Vec4f {
    hex_rgb(0xad_fb_15)
}
fn color_muted() -> Vec4f {
    hex_rgb(0x9f_b4_d0)
}

fn hex_rgb(rgb: u32) -> Vec4f {
    Vec4f {
        x: ((rgb >> 16) & 0xff) as f32 / 255.0,
        y: ((rgb >> 8) & 0xff) as f32 / 255.0,
        z: (rgb & 0xff) as f32 / 255.0,
        w: 1.0,
    }
}

impl App {
    fn apply_responsive(&mut self, cx: &mut Cx) {
        let size = self.ui.window(cx, ids!(main_window)).get_inner_size(cx);
        if size.x < 1.0 {
            return;
        }
        let key = size.x.round() as i64;
        if self.laid_out_w == Some(key) {
            return;
        }
        self.laid_out_w = Some(key);
        let flow = if size.x >= FRAMED_ONE_ROW_FROM {
            Flow::right()
        } else {
            Flow::Down
        };
        if let Some(mut view) = self.ui.view(cx, ids!(framed_ops)).borrow_mut() {
            view.layout.flow = flow;
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
        match event {
            Event::Startup | Event::WindowGeomChange(_) => self.apply_responsive(cx),
            _ => {}
        }
    }
}
