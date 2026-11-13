// Production Windows: GUI subsystem (no console). Dev keeps a console for logs.
#![cfg_attr(
    all(target_os = "windows", wadhu_hide_console),
    windows_subsystem = "windows"
)]

//! SMS Gateway — Android-first operator shell (phones, tablets, and other
//! Android OS devices with a SIM).
//!
//! Layout rules (do not regress):
//! - Body / multi-word copy uses `Label { width: Fill, height: Fit }` so text
//!   wraps inside the card. Never put long Fit-width labels in a Right row
//!   beside a badge — they crop.
//! - Page padding lives on an **inner** content view (ScrollYView padding is
//!   easy to lose). Cards keep their own padding ≥ 20.
//! - Text fields: Fixed height (≥ 56), `padding: Inset{…}`, `flow: Right`,
//!   `max_lines: 1`. Bare `{left:…}` / `{y:…}` objects are type-rejected
//!   (`expected Inset/Align, got object`) — padding becomes zero.
//! - Status value chips stay short Fit text; keys are Fill.

pub use makepad_widgets;

#[cfg(target_os = "android")]
use crate::{gateway, identity, permissions};
#[cfg(not(target_os = "android"))]
use wadhu_sms_gateway::{gateway, identity, permissions};

use gateway::Gateway;
use makepad_widgets::*;
use std::net::SocketAddr;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    // Shared wells — Fixed height + Inset padding. Fit+wrap empty_text clips.
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
        draw_text +: {
            color: #x0d0b3a
            text_style +: {font_size: 15.}
        }
    }

    let SecondaryBtn = Button{
        width: Fill
        height: 56.
        margin: Inset{left: 0. right: 0. top: 12. bottom: 8.}
        padding: Inset{left: 18. right: 18. top: 14. bottom: 14.}
        draw_bg +: {
            color: #x0133ad
            color_hover: #x00c8fc
            color_down: #x0b1f63
            border_radius: 14.
            border_size: 0.
        }
        draw_text +: {
            color: #xfcfbfb
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
            text_style +: {font_size: 14.}
        }
    }

    // Body prose — stock Label{Fill} crops; Markdown+Flow.Right{wrap} wraps.
    let BodyCopy = Markdown{
        width: Fill
        height: Fit
        flow: Flow.Right{wrap: true}
        padding: Inset{left: 0. right: 0. top: 0. bottom: 0.}
        selectable: false
        paragraph_spacing: 0.
        pre_code_spacing: 0.
        font_size: 14.
        font_color: #x9fb4d0
        draw_text +: {color: #x9fb4d0}
    }

    let BodyCopySm = BodyCopy{
        font_size: 13.
    }

    let BodyCopyLime = BodyCopy{
        font_size: 13.
        font_color: #xadfb15
        draw_text +: {color: #xadfb15}
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "SMS Gateway"
                window.inner_size: vec2(460, 920)
                body +: {
                    SolidView{
                        width: Fill
                        height: Fill
                        flow: Down
                        draw_bg +: {color: #x0d0b3a}

                        // —— Header (full width, wraps under the badge) ——
                        SolidView{
                            width: Fill
                            height: Fit
                            flow: Down
                            draw_bg +: {color: #x0b1f63}
                            SolidView{
                                width: Fill
                                height: 3
                                draw_bg +: {color: #x00c8fc}
                            }
                            SolidView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12
                                padding: Inset{left: 28. right: 28. top: 24. bottom: 24.}

                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 12
                                    align: Align{y: 0.5}
                                    title_label := Label{
                                        width: Fit
                                        height: Fit
                                        text: "SMS Gateway"
                                        draw_text.color: #xfcfbfb
                                        draw_text.text_style.font_size: 26
                                    }
                                    SolidView{ width: Fill height: 1 }
                                    RoundedView{
                                        width: Fit
                                        height: Fit
                                        padding: Inset{left: 14. right: 14. top: 10. bottom: 10.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 999.
                                            border_size: 1.
                                            border_color: #x00c8fc
                                        }
                                        live_badge := Label{
                                            width: Fit
                                            height: Fit
                                            text: "…"
                                            draw_text.color: #x00c8fc
                                            draw_text.text_style.font_size: 11
                                        }
                                    }
                                }

                                tagline_label := BodyCopyLime{
                                    body: "Turn this Android device into an SMS radio for your apps"
                                }
                                identity_label := BodyCopySm{
                                    body: ""
                                    font_size: 11.
                                }
                            }
                        }

                        ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down

                            // Inner pad — do not rely on ScrollYView padding alone.
                            SolidView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 36
                                padding: Inset{left: 28. right: 28. top: 28. bottom: 64.}

                                // —— Status ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "STATUS"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 0
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }

                                        // Right rows: Fit key + Fill spacer + Fit chip.
                                        // Fill-before-Fit starves the chip (clips "Listening").
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            spacing: 12
                                            padding: Inset{left: 22. right: 22. top: 20. bottom: 20.}
                                            align: Align{y: 0.5}
                                            Label{
                                                width: Fit
                                                height: Fit
                                                text: "SIM radio"
                                                draw_text.color: #x9fb4d0
                                                draw_text.text_style.font_size: 14
                                            }
                                            SolidView{ width: Fill height: 1 }
                                            radio_chip := Label{
                                                width: Fit
                                                height: Fit
                                                text: "…"
                                                draw_text.color: #x00c8fc
                                                draw_text.text_style.font_size: 14
                                            }
                                        }
                                        SolidView{ width: Fill height: 1 draw_bg +: {color: #x0b1f63} }
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            spacing: 12
                                            padding: Inset{left: 22. right: 22. top: 20. bottom: 20.}
                                            align: Align{y: 0.5}
                                            Label{
                                                width: Fit
                                                height: Fit
                                                text: "LAN API"
                                                draw_text.color: #x9fb4d0
                                                draw_text.text_style.font_size: 14
                                            }
                                            SolidView{ width: Fill height: 1 }
                                            api_chip := Label{
                                                width: Fit
                                                height: Fit
                                                text: "…"
                                                draw_text.color: #x00c8fc
                                                draw_text.text_style.font_size: 14
                                            }
                                        }
                                        SolidView{ width: Fill height: 1 draw_bg +: {color: #x0b1f63} }
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            spacing: 12
                                            padding: Inset{left: 22. right: 22. top: 20. bottom: 20.}
                                            align: Align{y: 0.5}
                                            Label{
                                                width: Fit
                                                height: Fit
                                                text: "Bluetooth"
                                                draw_text.color: #x9fb4d0
                                                draw_text.text_style.font_size: 14
                                            }
                                            SolidView{ width: Fill height: 1 }
                                            bt_chip := Label{
                                                width: Fit
                                                height: Fit
                                                text: "…"
                                                draw_text.color: #x00c8fc
                                                draw_text.text_style.font_size: 14
                                            }
                                        }
                                        SolidView{ width: Fill height: 1 draw_bg +: {color: #x0b1f63} }
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            spacing: 12
                                            padding: Inset{left: 22. right: 22. top: 20. bottom: 20.}
                                            align: Align{y: 0.5}
                                            Label{
                                                width: Fit
                                                height: Fit
                                                text: "Relay"
                                                draw_text.color: #x9fb4d0
                                                draw_text.text_style.font_size: 14
                                            }
                                            SolidView{ width: Fill height: 1 }
                                            relay_chip := Label{
                                                width: Fit
                                                height: Fit
                                                text: "…"
                                                draw_text.color: #x00c8fc
                                                draw_text.text_style.font_size: 14
                                            }
                                        }
                                        SolidView{ width: Fill height: 1 draw_bg +: {color: #x0b1f63} }
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            spacing: 12
                                            padding: Inset{left: 22. right: 22. top: 20. bottom: 20.}
                                            align: Align{y: 0.5}
                                            Label{
                                                width: Fit
                                                height: Fit
                                                text: "Queue"
                                                draw_text.color: #x9fb4d0
                                                draw_text.text_style.font_size: 14
                                            }
                                            SolidView{ width: Fill height: 1 }
                                            queue_chip := Label{
                                                width: Fit
                                                height: Fit
                                                text: "…"
                                                draw_text.color: #x00c8fc
                                                draw_text.text_style.font_size: 14
                                            }
                                        }
                                    }
                                    detail_label := BodyCopySm{
                                        body: ""
                                        font_size: 12.
                                    }
                                }

                                // —— Share ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "SHARE WITH CLIENT APPS"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    BodyCopy{
                                        body: "Any app on the same Wi‑Fi can send SMS through this device."
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 20
                                        padding: Inset{left: 22. right: 22. top: 24. bottom: 24.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Down
                                            spacing: 8
                                            Label{
                                                width: Fill
                                                height: Fit
                                                text: "Listen URL"
                                                draw_text.color: #x9fb4d0
                                                draw_text.text_style.font_size: 11
                                            }
                                            listen_label := BodyCopy{
                                                body: ""
                                                font_size: 15.
                                                font_color: #xfcfbfb
                                                draw_text +: {color: #xfcfbfb}
                                            }
                                        }
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Down
                                            spacing: 8
                                            Label{
                                                width: Fill
                                                height: Fit
                                                text: "API key  ·  X-Api-Key or Bearer"
                                                draw_text.color: #x9fb4d0
                                                draw_text.text_style.font_size: 11
                                            }
                                            token_label := BodyCopySm{
                                                body: ""
                                                font_color: #x00c8fc
                                                draw_text +: {color: #x00c8fc}
                                            }
                                        }
                                        hint_label := BodyCopySm{
                                            body: "POST /v1/messages   ·   GET /v1/health"
                                            font_size: 12.
                                        }
                                        bt_hint_label := BodyCopySm{
                                            body: "Bluetooth framing also available on TCP port 18766 (same JSON as HTTP)."
                                            font_size: 12.
                                        }
                                    }
                                }

                                // —— This device ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "THIS ANDROID DEVICE"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 16
                                        padding: Inset{left: 22. right: 22. top: 24. bottom: 24.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        perm_label := Label{
                                            width: Fill
                                            height: Fit
                                            text: ""
                                            draw_text.color: #x00c8fc
                                            draw_text.text_style.font_size: 14
                                        }
                                        perm_button := SecondaryBtn{
                                            text: "Allow SMS permission"
                                        }
                                        device_hint := BodyCopySm{
                                            body: "Works on phones, tablets, and other Android devices with a usable SIM. Keep the app open while clients send."
                                        }
                                    }
                                }

                                // —— Test send ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 14
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "TEST SEND"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8
                                        padding: Inset{left: 22. right: 22. top: 24. bottom: 24.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        BodyCopySm{
                                            body: "Uses the same queue client apps use."
                                        }
                                        to_input := FieldInput{
                                            empty_text: "To (+country number)"
                                        }
                                        SolidView{ width: Fill height: 8 }
                                        body_input := FieldInput{
                                            empty_text: "Message"
                                        }
                                        SolidView{ width: Fill height: 12 }
                                        send_button := PrimaryBtn{
                                            text: "Queue & send"
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

                                // —— Recent ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 14
                                    SolidView{
                                        width: Fill
                                        height: Fit
                                        flow: Right
                                        spacing: 12
                                        align: Align{y: 0.5}
                                        Label{
                                            width: Fill
                                            height: Fit
                                            text: "RECENT"
                                            draw_text.color: #x9fb4d0
                                            draw_text.text_style.font_size: 11
                                        }
                                        refresh_button := Button{
                                            width: Fit
                                            height: 40.
                                            text: "Refresh"
                                            padding: Inset{left: 16. right: 16. top: 10. bottom: 10.}
                                            draw_bg +: {
                                                color: #x0b1f63
                                                color_hover: #x0133ad
                                                border_radius: 12.
                                                border_size: 0.
                                            }
                                            draw_text +: {
                                                color: #xfcfbfb
                                                text_style +: {font_size: 13.}
                                            }
                                        }
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
                                        recent_label := BodyCopySm{
                                            body: "No messages yet. Send a test, or POST from a client app."
                                        }
                                    }
                                }

                                // —— Relay ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 14
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "RELAY (OPTIONAL)"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    BodyCopy{
                                        body: "For internet clients. LAN and Bluetooth keep working if the relay is offline."
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 8
                                        padding: Inset{left: 22. right: 22. top: 24. bottom: 24.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        relay_url_input := FieldInput{
                                            empty_text: "Relay URL (http://host:9876)"
                                        }
                                        SolidView{ width: Fill height: 8 }
                                        relay_device_id_input := FieldInput{
                                            empty_text: "Device id"
                                        }
                                        SolidView{ width: Fill height: 8 }
                                        relay_secret_input := FieldInput{
                                            empty_text: "Device secret"
                                        }
                                        SolidView{ width: Fill height: 12 }
                                        relay_save_button := GhostBtn{
                                            text: "Save relay pairing"
                                        }
                                        relay_status_label := BodyCopySm{
                                            body: ""
                                            font_size: 12.
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

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    gateway: Option<Gateway>,
    #[rust]
    refresh_timer: Timer,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        log!("{}", identity::summary());
        self.ui
            .window(cx, ids!(main_window))
            .set_title(cx, identity::APP_LABEL);
        self.ui.markdown(cx, ids!(identity_label)).set_text(
            cx,
            &format!("{} · {}", identity::APP_ID, identity::ENV),
        );

        #[cfg(not(target_os = "android"))]
        {
            self.ui.label(cx, ids!(perm_label)).set_text(
                cx,
                "Desktop dry-run — SIM radio needs an Android device with a usable SIM.",
            );
            self.ui
                .label(cx, ids!(perm_label))
                .set_text_color(cx, color_muted());
            self.ui
                .button(cx, ids!(perm_button))
                .set_text(cx, "SMS permission (Android only)");
            self.ui.markdown(cx, ids!(tagline_label)).set_text(
                cx,
                "Desktop preview — ship and run on Android phones, tablets, and other devices",
            );
        }

        let addr: SocketAddr = if cfg!(target_os = "android") {
            "0.0.0.0:18765".parse().unwrap()
        } else {
            "127.0.0.1:18765".parse().unwrap()
        };

        match Gateway::start(addr) {
            Ok(gw) => {
                self.gateway = Some(gw);
                self.refresh_all(cx);
            }
            Err(e) => {
                self.ui.label(cx, ids!(live_badge)).set_text(cx, "DOWN");
                self.ui
                    .label(cx, ids!(live_badge))
                    .set_text_color(cx, color_fail());
                self.ui.label(cx, ids!(api_chip)).set_text(cx, "Down");
                self.ui
                    .label(cx, ids!(api_chip))
                    .set_text_color(cx, color_fail());
                self.ui
                    .markdown(cx, ids!(detail_label))
                    .set_text(cx, &format!("Gateway failed to start: {e}"));
            }
        }

        self.refresh_timer = cx.start_interval(1.5);
    }

    fn handle_timer(&mut self, cx: &mut Cx, e: &TimerEvent) {
        if self.refresh_timer.is_timer(e).is_some() {
            self.refresh_all(cx);
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(refresh_button)).clicked(actions) {
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(relay_save_button)).clicked(actions) {
            let url = self.ui.text_input(cx, ids!(relay_url_input)).text();
            let device_id = self.ui.text_input(cx, ids!(relay_device_id_input)).text();
            let secret = self.ui.text_input(cx, ids!(relay_secret_input)).text();
            if let Some(gw) = &self.gateway {
                gw.configure_relay(
                    url.trim().into(),
                    device_id.trim().into(),
                    secret.trim().into(),
                );
                self.ui.markdown(cx, ids!(relay_status_label)).set_text(
                    cx,
                    "Relay pairing saved — reconnecting in the background",
                );
                self.refresh_all(cx);
            }
        }
        if self.ui.button(cx, ids!(perm_button)).clicked(actions) {
            permissions::request_sms_permission();
            self.ui.label(cx, ids!(result_label)).set_text(
                cx,
                "Permission dialog requested — allow SMS, then wait for status to update",
            );
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(send_button)).clicked(actions) {
            let to = self.ui.text_input(cx, ids!(to_input)).text();
            let body = self.ui.text_input(cx, ids!(body_input)).text();
            if to.trim().is_empty() || body.trim().is_empty() {
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text(cx, "Enter a destination and a message");
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text_color(cx, color_fail());
                return;
            }
            if let Some(gw) = &self.gateway {
                match gw.handle.outbox.accept(String::new(), to, body) {
                    Ok(m) => {
                        self.ui.label(cx, ids!(result_label)).set_text(
                            cx,
                            &format!("Queued {} — watching drain…", short_id(&m.id)),
                        );
                        self.ui
                            .label(cx, ids!(result_label))
                            .set_text_color(cx, color_lime());
                    }
                    Err(e) => {
                        self.ui
                            .label(cx, ids!(result_label))
                            .set_text(cx, &format!("Not accepted: {e}"));
                        self.ui
                            .label(cx, ids!(result_label))
                            .set_text_color(cx, color_fail());
                    }
                }
                self.refresh_all(cx);
            }
        }
    }
}

impl App {
    fn refresh_all(&self, cx: &mut Cx) {
        let Some(gw) = &self.gateway else { return };

        let caps = gw.handle.radio.capabilities();
        let (pending, submitted, failed) = gw.handle.outbox.counts();
        let listen = gw.handle.listen;
        let token = &gw.handle.api_token;

        self.ui.label(cx, ids!(live_badge)).set_text(cx, "LIVE");
        self.ui
            .label(cx, ids!(live_badge))
            .set_text_color(cx, color_ok());

        let (radio_text, radio_color) = if caps.available {
            ("Ready", color_ok())
        } else if caps.kind == "none" {
            ("Desktop only", color_warn())
        } else {
            ("Not ready", color_fail())
        };
        self.ui
            .label(cx, ids!(radio_chip))
            .set_text(cx, radio_text);
        self.ui
            .label(cx, ids!(radio_chip))
            .set_text_color(cx, radio_color);

        self.ui.label(cx, ids!(api_chip)).set_text(cx, "Listening");
        self.ui
            .label(cx, ids!(api_chip))
            .set_text_color(cx, color_ok());

        let bt = gw.handle.bluetooth.snapshot();
        let (bt_text, bt_color) = if bt.listening {
            ("On", color_ok())
        } else if bt.enabled {
            ("Starting", color_warn())
        } else {
            ("Off", color_muted())
        };
        self.ui.label(cx, ids!(bt_chip)).set_text(cx, bt_text);
        self.ui.label(cx, ids!(bt_chip)).set_text_color(cx, bt_color);

        let relay = gw.handle.relay.snapshot();
        let (relay_text, relay_color) = if relay.connected {
            ("Online", color_ok())
        } else if relay.configured {
            ("Connecting", color_warn())
        } else {
            ("Off", color_muted())
        };
        self.ui.label(cx, ids!(relay_chip)).set_text(cx, relay_text);
        self.ui
            .label(cx, ids!(relay_chip))
            .set_text_color(cx, relay_color);
        self.ui
            .markdown(cx, ids!(relay_status_label))
            .set_text(cx, &relay.detail);

        let queue_text = if pending == 0 {
            "Idle".into()
        } else {
            format!("{pending} pending")
        };
        self.ui
            .label(cx, ids!(queue_chip))
            .set_text(cx, &queue_text);
        self.ui.label(cx, ids!(queue_chip)).set_text_color(
            cx,
            if pending > 0 {
                color_warn()
            } else {
                color_ok()
            },
        );

        self.ui.markdown(cx, ids!(detail_label)).set_text(
            cx,
            &format!(
                "{}\n\n{}\n\nSent {submitted} · failed {failed}",
                caps.label, caps.detail
            ),
        );

        let host = if cfg!(target_os = "android") {
            format!("http://<this-device-ip>:{}", listen.port())
        } else {
            format!("http://{}", listen)
        };
        self.ui.markdown(cx, ids!(listen_label)).set_text(cx, &host);
        self.ui.markdown(cx, ids!(token_label)).set_text(cx, token);

        let lines: Vec<String> = gw
            .handle
            .outbox
            .list(10)
            .into_iter()
            .map(|m| {
                let err = m
                    .error
                    .as_deref()
                    .filter(|e| !e.is_empty())
                    .map(|e| format!(" — {e}"))
                    .unwrap_or_default();
                format!(
                    "{}  ·  {}  →  {}{}",
                    m.status.as_str(),
                    short_id(&m.id),
                    m.to,
                    err
                )
            })
            .collect();
        let text = if lines.is_empty() {
            "No messages yet. Send a test, or POST from a client app.".into()
        } else {
            lines.join("\n")
        };
        self.ui.markdown(cx, ids!(recent_label)).set_text(cx, &text);

        #[cfg(target_os = "android")]
        {
            let granted = permissions::sms_permission_granted();
            if granted {
                self.ui
                    .label(cx, ids!(perm_label))
                    .set_text(cx, "SMS permission granted — this device can send");
                self.ui
                    .label(cx, ids!(perm_label))
                    .set_text_color(cx, color_ok());
            } else {
                self.ui.label(cx, ids!(perm_label)).set_text(
                    cx,
                    "Allow SMS so this Android device can send through its SIM",
                );
                self.ui
                    .label(cx, ids!(perm_label))
                    .set_text_color(cx, color_warn());
            }
        }
    }
}

fn short_id(id: &str) -> &str {
    id.rsplit('-').next().unwrap_or(id)
}

fn color_ok() -> Vec4f {
    hex_rgb(0x0f_ca_65)
}
fn color_warn() -> Vec4f {
    hex_rgb(0xad_fb_15)
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

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
