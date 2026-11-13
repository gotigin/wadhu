// Production Windows: GUI subsystem (no console). Dev keeps a console for logs.
#![cfg_attr(
    all(target_os = "windows", wadhu_hide_console),
    windows_subsystem = "windows"
)]

//! SMS Gateway — Makepad 2 operator shell for the local SMS gateway service.

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

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "SMS Gateway"
                window.inner_size: vec2(760, 860)
                body +: {
                    SolidView{
                        width: Fill
                        height: Fill
                        flow: Down
                        draw_bg +: {color: #x0d0b3a}

                        // Brand accent strip
                        SolidView{
                            width: Fill
                            height: 4
                            draw_bg +: {color: #x00c8fc}
                        }

                        ScrollYView{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 16
                            padding: {left: 24. right: 24. top: 22. bottom: 32.}

                            // —— Header ——
                            SolidView{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 12
                                align: {y: 0.5}
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 4
                                    title_label := Label{
                                        text: "SMS Gateway"
                                        draw_text.color: #xfcfbfb
                                        draw_text.text_style.font_size: 30
                                    }
                                    tagline_label := Label{
                                        text: "Other apps connect here to send SMS"
                                        draw_text.color: #xadfb15
                                        draw_text.text_style.font_size: 13
                                    }
                                    identity_label := Label{
                                        text: ""
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 10
                                    }
                                }
                                RoundedView{
                                    width: Fit
                                    height: Fit
                                    padding: {left: 12. right: 12. top: 8. bottom: 8.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        border_radius: 999.
                                        border_size: 1.
                                        border_color: #x00c8fc
                                    }
                                    live_badge := Label{
                                        text: "STARTING"
                                        draw_text.color: #x00c8fc
                                        draw_text.text_style.font_size: 10
                                    }
                                }
                            }

                            // —— Health ——
                            RoundedView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 12
                                padding: {left: 18. right: 18. top: 16. bottom: 16.}
                                draw_bg +: {
                                    color: #x161252
                                    border_radius: 16.
                                }
                                Label{
                                    text: "GATEWAY HEALTH"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 10
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 4
                                        padding: {left: 14. right: 14. top: 12. bottom: 12.}
                                        draw_bg +: {
                                            color: #x0b1f63
                                            border_radius: 12.
                                        }
                                        Label{
                                            text: "RADIO"
                                            draw_text.color: #x9fb4d0
                                            draw_text.text_style.font_size: 9
                                        }
                                        radio_chip := Label{
                                            text: "…"
                                            draw_text.color: #x00c8fc
                                            draw_text.text_style.font_size: 15
                                        }
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 4
                                        padding: {left: 14. right: 14. top: 12. bottom: 12.}
                                        draw_bg +: {
                                            color: #x0b1f63
                                            border_radius: 12.
                                        }
                                        Label{
                                            text: "API"
                                            draw_text.color: #x9fb4d0
                                            draw_text.text_style.font_size: 9
                                        }
                                        api_chip := Label{
                                            text: "…"
                                            draw_text.color: #x00c8fc
                                            draw_text.text_style.font_size: 15
                                        }
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 4
                                        padding: {left: 14. right: 14. top: 12. bottom: 12.}
                                        draw_bg +: {
                                            color: #x0b1f63
                                            border_radius: 12.
                                        }
                                        Label{
                                            text: "QUEUE"
                                            draw_text.color: #x9fb4d0
                                            draw_text.text_style.font_size: 9
                                        }
                                        queue_chip := Label{
                                            text: "…"
                                            draw_text.color: #x00c8fc
                                            draw_text.text_style.font_size: 15
                                        }
                                    }
                                }
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 10
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 4
                                        padding: {left: 14. right: 14. top: 12. bottom: 12.}
                                        draw_bg +: {
                                            color: #x0b1f63
                                            border_radius: 12.
                                        }
                                        Label{
                                            text: "BLUETOOTH"
                                            draw_text.color: #x9fb4d0
                                            draw_text.text_style.font_size: 9
                                        }
                                        bt_chip := Label{
                                            text: "…"
                                            draw_text.color: #x00c8fc
                                            draw_text.text_style.font_size: 15
                                        }
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 4
                                        padding: {left: 14. right: 14. top: 12. bottom: 12.}
                                        draw_bg +: {
                                            color: #x0b1f63
                                            border_radius: 12.
                                        }
                                        Label{
                                            text: "RELAY"
                                            draw_text.color: #x9fb4d0
                                            draw_text.text_style.font_size: 9
                                        }
                                        relay_chip := Label{
                                            text: "…"
                                            draw_text.color: #x00c8fc
                                            draw_text.text_style.font_size: 15
                                        }
                                    }
                                }
                                detail_label := Label{
                                    text: ""
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
                                }
                            }

                            // —— Connection ——
                            RoundedView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 10
                                padding: {left: 18. right: 18. top: 16. bottom: 16.}
                                draw_bg +: {
                                    color: #x161252
                                    border_radius: 16.
                                }
                                Label{
                                    text: "FOR CLIENT APPS"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                Label{
                                    text: "Give these to any app that should send SMS through this gateway."
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
                                }
                                RoundedView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 4
                                    padding: {left: 14. right: 14. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        border_radius: 10.
                                    }
                                    Label{
                                        text: "LISTEN URL"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 9
                                    }
                                    listen_label := Label{
                                        text: ""
                                        draw_text.color: #xfcfbfb
                                        draw_text.text_style.font_size: 14
                                    }
                                }
                                RoundedView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 4
                                    padding: {left: 14. right: 14. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        border_radius: 10.
                                    }
                                    Label{
                                        text: "API KEY  ·  X-Api-Key or Authorization: Bearer"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 9
                                    }
                                    token_label := Label{
                                        text: ""
                                        draw_text.color: #x00c8fc
                                        draw_text.text_style.font_size: 12
                                    }
                                }
                                hint_label := Label{
                                    text: "POST /v1/messages   ·   GET /v1/health   ·   GET /v1/capabilities"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                bt_hint_label := Label{
                                    text: "Bluetooth framing: TCP :18766 (same JSON) — see docs/BLUETOOTH.md"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                            }

                            // —— Relay pairing ——
                            RoundedView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 10
                                padding: {left: 18. right: 18. top: 16. bottom: 16.}
                                draw_bg +: {
                                    color: #x161252
                                    border_radius: 16.
                                }
                                Label{
                                    text: "SMS RELAY (OPTIONAL)"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                Label{
                                    text: "Self-hosted sms-relay for internet clients. Local LAN/BT never need this."
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
                                }
                                relay_url_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "Relay URL — http://host:9876"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                relay_device_id_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "Device id from relay pairing"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                relay_secret_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "Device secret from relay pairing"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                relay_save_button := Button{
                                    width: Fill
                                    height: Fit
                                    text: "Save relay pairing"
                                    padding: {left: 16. right: 16. top: 12. bottom: 12.}
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
                                relay_status_label := Label{
                                    text: ""
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
                                }
                            }

                            // —— Compose ——
                            RoundedView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 10
                                padding: {left: 18. right: 18. top: 16. bottom: 16.}
                                draw_bg +: {
                                    color: #x161252
                                    border_radius: 16.
                                }
                                Label{
                                    text: "SEND A TEST SMS"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                Label{
                                    text: "Queues through the same path client apps use."
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
                                }
                                to_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "To — +country code and number"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                body_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "Message body"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                send_button := Button{
                                    width: Fill
                                    height: Fit
                                    text: "Queue & send"
                                    padding: {left: 16. right: 16. top: 14. bottom: 14.}
                                    draw_bg +: {
                                        color: #x0133ad
                                        color_hover: #x00c8fc
                                        color_down: #x0b1f63
                                        border_radius: 12.
                                        border_size: 0.
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        text_style +: {font_size: 14.}
                                    }
                                }
                                result_label := Label{
                                    text: ""
                                    draw_text.color: #xadfb15
                                    draw_text.text_style.font_size: 12
                                }
                            }

                            // —— Recent ——
                            SolidView{
                                width: Fill
                                height: Fit
                                flow: Right
                                spacing: 10
                                align: {y: 0.5}
                                Label{
                                    width: Fill
                                    text: "RECENT MESSAGES"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                refresh_button := Button{
                                    width: Fit
                                    height: Fit
                                    text: "Refresh"
                                    padding: {left: 14. right: 14. top: 8. bottom: 8.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        border_radius: 10.
                                        border_size: 0.
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        text_style +: {font_size: 11.}
                                    }
                                }
                            }
                            RoundedView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 6
                                padding: {left: 18. right: 18. top: 14. bottom: 14.}
                                draw_bg +: {
                                    color: #x161252
                                    border_radius: 16.
                                }
                                recent_label := Label{
                                    text: "No messages yet"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 12
                                }
                            }

                            // —— Relay ——
                            RoundedView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 10
                                padding: {left: 18. right: 18. top: 16. bottom: 16.}
                                draw_bg +: {
                                    color: #x161252
                                    border_radius: 16.
                                }
                                Label{
                                    text: "SMS RELAY (OPTIONAL)"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                Label{
                                    text: "Pair with a self-hosted sms-relay for internet clients. LAN and Bluetooth still work offline."
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
                                }
                                relay_url_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "Relay URL — http://host:9876"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                relay_device_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "Device id from relay pairing"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                relay_secret_input := TextInput{
                                    width: Fill
                                    height: Fit
                                    empty_text: "Device secret from relay pairing"
                                    padding: {left: 12. right: 12. top: 10. bottom: 10.}
                                    draw_bg +: {
                                        color: #x0b1f63
                                        color_hover: #x0133ad
                                        color_focus: #x0133ad
                                        color_empty: #x0b1f63
                                        border_radius: 10.
                                        border_size: 1.
                                        border_color: #x161252
                                        border_color_focus: #x00c8fc
                                        border_color_empty: #x161252
                                    }
                                    draw_text +: {
                                        color: #xfcfbfb
                                        color_empty: #x9fb4d0
                                        text_style +: {font_size: 13.}
                                    }
                                }
                                relay_save_button := Button{
                                    width: Fill
                                    height: Fit
                                    text: "Save relay pairing"
                                    padding: {left: 16. right: 16. top: 12. bottom: 12.}
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
                                relay_detail_label := Label{
                                    text: ""
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
                                }
                            }

                            // —— Device ——
                            RoundedView{
                                width: Fill
                                height: Fit
                                flow: Down
                                spacing: 10
                                padding: {left: 18. right: 18. top: 16. bottom: 16.}
                                draw_bg +: {
                                    color: #x161252
                                    border_radius: 16.
                                }
                                Label{
                                    text: "THIS DEVICE"
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 10
                                }
                                perm_label := Label{
                                    text: ""
                                    draw_text.color: #x00c8fc
                                    draw_text.text_style.font_size: 12
                                }
                                perm_button := Button{
                                    width: Fill
                                    height: Fit
                                    text: "Request SMS permission"
                                    padding: {left: 16. right: 16. top: 12. bottom: 12.}
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
                                device_hint := Label{
                                    text: "Android uses this phone's SIM. Clients: LAN :18765 · BT framing :18766 · optional sms-relay."
                                    draw_text.color: #x9fb4d0
                                    draw_text.text_style.font_size: 11
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
        self.ui
            .label(cx, ids!(identity_label))
            .set_text(cx, &format!("{} · {}", identity::APP_ID, identity::ENV));

        #[cfg(not(target_os = "android"))]
        {
            self.ui
                .label(cx, ids!(perm_label))
                .set_text(cx, "SMS permission applies on Android only");
            self.ui
                .label(cx, ids!(perm_label))
                .set_text_color(cx, color_muted());
            self.ui
                .button(cx, ids!(perm_button))
                .set_text(cx, "SMS permission (Android only)");
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
                    .label(cx, ids!(detail_label))
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
                gw.configure_relay(url.trim().into(), device_id.trim().into(), secret.trim().into());
                self.ui.label(cx, ids!(relay_status_label)).set_text(
                    cx,
                    "Relay pairing saved — outbound session will reconnect",
                );
                self.ui
                    .label(cx, ids!(relay_status_label))
                    .set_text_color(cx, color_lime());
                self.refresh_all(cx);
            }
        }
        if self.ui.button(cx, ids!(perm_button)).clicked(actions) {
            permissions::request_sms_permission();
            self.ui.label(cx, ids!(result_label)).set_text(
                cx,
                "Permission dialog requested — allow SMS, then wait for health to update",
            );
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(relay_save_button)).clicked(actions) {
            let url = self.ui.text_input(cx, ids!(relay_url_input)).text();
            let device_id = self.ui.text_input(cx, ids!(relay_device_input)).text();
            let secret = self.ui.text_input(cx, ids!(relay_secret_input)).text();
            if let Some(gw) = &self.gateway {
                gw.configure_relay(url, device_id, secret);
                self.ui.label(cx, ids!(relay_detail_label)).set_text(
                    cx,
                    "Relay pairing saved — reconnecting in the background",
                );
                self.ui
                    .label(cx, ids!(relay_detail_label))
                    .set_text_color(cx, color_lime());
                self.refresh_all(cx);
            }
        }
        if self.ui.button(cx, ids!(send_button)).clicked(actions) {
            let to = self.ui.text_input(cx, ids!(to_input)).text();
            let body = self.ui.text_input(cx, ids!(body_input)).text();
            if to.trim().is_empty() || body.trim().is_empty() {
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text(cx, "Enter a destination and a message body");
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
            ("No modem yet", color_warn())
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
            ("Listening", color_ok())
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
            ("Optional", color_muted())
        };
        self.ui.label(cx, ids!(relay_chip)).set_text(cx, relay_text);
        self.ui
            .label(cx, ids!(relay_chip))
            .set_text_color(cx, relay_color);
        self.ui
            .label(cx, ids!(relay_status_label))
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

        self.ui.label(cx, ids!(detail_label)).set_text(
            cx,
            &format!(
                "{} · {}   ·   submitted {submitted}   ·   failed {failed}",
                caps.label, caps.detail
            ),
        );

        let host = if cfg!(target_os = "android") {
            format!("http://<this-phone-ip>:{}", listen.port())
        } else {
            format!("http://{}", listen)
        };
        self.ui.label(cx, ids!(listen_label)).set_text(cx, &host);
        self.ui.label(cx, ids!(token_label)).set_text(cx, token);

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
            "No messages yet. Send a test above, or POST from a client app.".into()
        } else {
            lines.join("\n")
        };
        self.ui.label(cx, ids!(recent_label)).set_text(cx, &text);
        self.ui.label(cx, ids!(recent_label)).set_text_color(
            cx,
            if lines.is_empty() {
                color_muted()
            } else {
                color_white()
            },
        );

        #[cfg(target_os = "android")]
        {
            let granted = permissions::sms_permission_granted();
            if granted {
                self.ui
                    .label(cx, ids!(perm_label))
                    .set_text(cx, "SMS permission granted — radio can send");
                self.ui
                    .label(cx, ids!(perm_label))
                    .set_text_color(cx, color_ok());
            } else {
                self.ui
                    .label(cx, ids!(perm_label))
                    .set_text(cx, "SMS permission needed before the SIM radio can send");
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
fn color_white() -> Vec4f {
    hex_rgb(0xfc_fb_fb)
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
