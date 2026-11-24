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
use crate::{gateway, identity, lan, permissions};
#[cfg(not(target_os = "android"))]
use wadhu_sms_gateway::{gateway, identity, lan, permissions};

use gateway::Gateway;
use makepad_widgets::*;
use makepad_widgets::makepad_platform::event::finger::TouchState;
use std::net::SocketAddr;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

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
            color_hover: #x0d0b3a
            color_down: #xfcfbfb
            color_focus: #xfcfbfb
            text_style +: {font_size: 15.}
        }
    }

    // The Do Next card is the SecondaryBtn blue, so its button takes the
    // night fill: on the stock face the button had no visible edge.
    let NextBtn = SecondaryBtn{
        draw_bg +: {
            color: #x0d0b3a
            color_hover: #x00c8fc
            color_down: #x161252
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
                                    // Fill, not Fit + spacer: the title gives
                                    // way first, so the badge stays whole on a
                                    // 360 px phone (Fit + Fit clipped the badge).
                                    title_label := Label{
                                        width: Fill
                                        height: Fit
                                        text: "SMS Gateway"
                                        draw_text.color: #xfcfbfb
                                        draw_text.text_style.font_size: 24
                                    }
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
                                    body: "This phone sends SMS for your apps — grant permission, send a test, share the URL"
                                }
                                identity_label := BodyCopySm{
                                    body: ""
                                    font_size: 11.
                                }
                                exit_back_notice := BodyCopySm{
                                    body: ""
                                    font_size: 12.
                                    font_color: #xadfb15
                                }
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
                                padding: Inset{left: 28. right: 28. top: 24. bottom: 64.}

                                // —— Next step (action, not brochure) ——
                                next_card := RoundedView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 14
                                    padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                    draw_bg +: {
                                        color: #x0133ad
                                        border_radius: 20.
                                    }
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "DO NEXT"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    next_step_label := BodyCopy{
                                        body: "Starting…"
                                        font_size: 15.
                                        font_color: #xfcfbfb
                                        draw_text +: {color: #xfcfbfb}
                                    }
                                    perm_button := NextBtn{
                                        text: "Allow SMS permission"
                                    }
                                }

                                // —— Status chips (compact) ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 10
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
                                        SolidView{
                                            width: Fill
                                            height: Fit
                                            flow: Right
                                            spacing: 12
                                            padding: Inset{left: 22. right: 22. top: 18. bottom: 18.}
                                            align: Align{y: 0.5}
                                            Label{
                                                width: Fit
                                                height: Fit
                                                text: "SIM"
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
                                            padding: Inset{left: 22. right: 22. top: 18. bottom: 18.}
                                            align: Align{y: 0.5}
                                            Label{
                                                width: Fit
                                                height: Fit
                                                text: "LAN"
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
                                            padding: Inset{left: 22. right: 22. top: 18. bottom: 18.}
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
                                            padding: Inset{left: 22. right: 22. top: 18. bottom: 18.}
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
                                            padding: Inset{left: 22. right: 22. top: 18. bottom: 18.}
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

                                // —— Test send (prove the radio) ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "SEND A REAL SMS"
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
                                        BodyCopySm{
                                            body: "Goes through the same queue as LAN clients. Needs SMS permission + SIM."
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
                                            text: "Send via this SIM"
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
                                    spacing: 12
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
                                        padding: Inset{left: 22. right: 22. top: 20. bottom: 20.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        recent_label := BodyCopySm{
                                            body: "No messages yet — send one above."
                                        }
                                    }
                                }

                                // —— Allow apps (non-expert LAN pair) ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "ALLOW AN APP (SAME WI-FI)"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 12
                                        padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        BodyCopySm{
                                            body: "In the brand app tap Connect to SMS phone, then enter this code. No IP typing."
                                        }
                                        allow_app_button := PrimaryBtn{
                                            text: "Allow a new app"
                                        }
                                        pair_code_label := Label{
                                            width: Fill
                                            height: Fit
                                            text: "————"
                                            draw_text.color: #xadfb15
                                            draw_text.text_style.font_size: 32
                                        }
                                        pair_hint_label := BodyCopySm{
                                            body: "Code appears here for about 5 minutes."
                                            font_size: 12.
                                        }
                                        Label{
                                            width: Fill
                                            height: Fit
                                            margin: Inset{top: 8. bottom: 0.}
                                            text: "CONNECTED APPS"
                                            draw_text.color: #x9fb4d0
                                            draw_text.text_style.font_size: 11
                                        }
                                        clients_label := BodyCopySm{
                                            body: "No apps connected yet."
                                            font_size: 13.
                                        }
                                        // Short hints: a 360 px phone leaves the
                                        // field about 27 characters before "…".
                                        client_name_input := FieldInput{
                                            empty_text: "App name"
                                        }
                                        client_ops := View{
                                            width: Fill height: Fit
                                            flow: Down
                                            spacing: 10.
                                            block_client_button := GhostBtn{
                                                text: "Block app"
                                            }
                                            unblock_client_button := GhostBtn{
                                                text: "Unblock app"
                                            }
                                        }
                                        // The consequence is its own line: on one
                                        // button face it ran off both edges at 360 px.
                                        BodyCopySm{
                                            body: "Changing the access key disconnects every app."
                                            font_size: 12.
                                        }
                                        rotate_key_button := GhostBtn{
                                            text: "Change access key"
                                        }
                                    }
                                }

                                // —— Advanced (developers) ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "ADVANCED (OPTIONAL)"
                                        draw_text.color: #x9fb4d0
                                        draw_text.text_style.font_size: 11
                                    }
                                    RoundedView{
                                        width: Fill
                                        height: Fit
                                        flow: Down
                                        spacing: 12
                                        padding: Inset{left: 22. right: 22. top: 22. bottom: 22.}
                                        draw_bg +: {
                                            color: #x161252
                                            border_radius: 20.
                                        }
                                        BodyCopySm{
                                            body: "Only if a tool asks for a raw URL. Shop staff should use Allow a new app above."
                                        }
                                        listen_label := BodyCopy{
                                            body: ""
                                            font_size: 14.
                                            font_color: #xfcfbfb
                                            draw_text +: {color: #xfcfbfb}
                                        }
                                        token_label := BodyCopySm{
                                            body: ""
                                            font_color: #x00c8fc
                                            draw_text +: {color: #x00c8fc}
                                        }
                                        // Short Fit actions — apply_responsive
                                        // unlocks to Right when the column is wide
                                        // enough (wrap+Fill is rejected by Makepad).
                                        share_row := View{
                                            width: Fill height: Fit
                                            flow: Down
                                            spacing: 10.
                                            share_url_button := GhostBtn{
                                                text: "Share URL + operator key"
                                            }
                                            share_curl_button := GhostBtn{
                                                text: "Share curl example"
                                            }
                                            copy_key_button := GhostBtn{
                                                text: "Copy operator key"
                                            }
                                        }
                                    }
                                }

                                // —— Relay (optional, collapsed visually) ——
                                SolidView{
                                    width: Fill
                                    height: Fit
                                    flow: Down
                                    spacing: 12
                                    Label{
                                        width: Fill
                                        height: Fit
                                        text: "RELAY (OPTIONAL)"
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
                                        BodyCopySm{
                                            body: "Internet clients via sms-relay. LAN/BT keep working without it. The URL looks like http://host:9876."
                                        }
                                        relay_url_input := FieldInput{
                                            empty_text: "Relay URL"
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

/// Content column width where short Fit action pairs / triples sit on one row.
const ACTIONS_ONE_ROW_FROM: f64 = 720.0;

const PRESS_BACK_AGAIN_EXIT: &str = "Press back again to exit";
const EXIT_BACK_SECS: u64 = 10;

fn exit_back_armed_at() -> &'static std::sync::Mutex<Option<Instant>> {
    static SLOT: OnceLock<std::sync::Mutex<Option<Instant>>> = OnceLock::new();
    SLOT.get_or_init(|| std::sync::Mutex::new(None))
}

fn clear_exit_back_arm() {
    *exit_back_armed_at().lock().unwrap() = None;
}

fn note_exit_back_press(cx: &mut Cx) -> Result<bool, ()> {
    let now = Instant::now();
    let mut armed = exit_back_armed_at().lock().unwrap();
    if let Some(prev) = *armed
        && now.duration_since(prev) < Duration::from_secs(EXIT_BACK_SECS)
    {
        *armed = None;
        drop(armed);
        #[cfg(target_os = "android")]
        {
            let _ = cx;
            return Ok(false);
        }
        #[cfg(not(target_os = "android"))]
        {
            cx.quit();
            return Err(());
        }
    }
    *armed = Some(now);
    drop(armed);
    Ok(true)
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    gateway: Option<Gateway>,
    #[rust]
    refresh_timer: Timer,
    /// Last test-send id — refresh_all updates result_label as status moves.
    #[rust]
    last_test_id: Option<String>,
    #[rust]
    laid_out_w: Option<i64>,
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
            // Nothing to grant on a desktop: no button, the card says why.
            self.ui.button(cx, ids!(perm_button)).set_visible(cx, false);
            self.ui.markdown(cx, ids!(tagline_label)).set_text(
                cx,
                "Desktop preview — run on Android to send real SMS through a SIM",
            );
            self.ui.markdown(cx, ids!(next_step_label)).set_text(
                cx,
                "Desktop dry-run: LAN API works here; SIM send needs an Android device.",
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

        // Backup poll — primary updates come from outbox → SignalToUI.
        self.refresh_timer = cx.start_interval(0.5);
    }

    fn handle_signal(&mut self, cx: &mut Cx) {
        self.refresh_all(cx);
        self.ui.redraw(cx);
    }

    fn handle_timer(&mut self, cx: &mut Cx, e: &TimerEvent) {
        if self.refresh_timer.is_timer(e).is_some() {
            self.refresh_all(cx);
        }
    }

    fn handle_back_pressed(&mut self, cx: &mut Cx) -> bool {
        self.handle_visit_root_back(cx)
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
                "Permission dialog requested — allow SMS + Bluetooth, then wait for SIM → Ready / BT → On",
            );
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(allow_app_button)).clicked(actions) {
            let Some(gw) = &self.gateway else { return };
            let code = gw.begin_pairing();
            self.ui
                .label(cx, ids!(pair_code_label))
                .set_text(cx, &code);
            self.ui.markdown(cx, ids!(pair_hint_label)).set_text(
                cx,
                "Show this code to the other phone/app. Expires in about 5 minutes.",
            );
            self.ui.label(cx, ids!(result_label)).set_text(
                cx,
                "Allow code ready — enter it in the brand app on the same Wi-Fi",
            );
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(block_client_button)).clicked(actions) {
            let Some(gw) = &self.gateway else { return };
            let name = self.ui.text_input(cx, ids!(client_name_input)).text();
            match gw.set_client_blocked(name.trim(), true) {
                Ok(()) => {
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text(cx, &format!("Blocked “{}” — it can no longer send SMS", name.trim()));
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text_color(cx, color_lime());
                }
                Err(e) => {
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text(cx, &format!("Could not block: {e}"));
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text_color(cx, color_fail());
                }
            }
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(unblock_client_button)).clicked(actions) {
            let Some(gw) = &self.gateway else { return };
            let name = self.ui.text_input(cx, ids!(client_name_input)).text();
            match gw.set_client_blocked(name.trim(), false) {
                Ok(()) => {
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text(cx, &format!("Unblocked “{}”", name.trim()));
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text_color(cx, color_lime());
                }
                Err(e) => {
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text(cx, &format!("Could not unblock: {e}"));
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text_color(cx, color_fail());
                }
            }
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(rotate_key_button)).clicked(actions) {
            let Some(gw) = &self.gateway else { return };
            let _ = gw.rotate_operator_key();
            self.ui.label(cx, ids!(result_label)).set_text(
                cx,
                "Access key changed — all apps must connect again with a new Allow code",
            );
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
            self.refresh_all(cx);
        }
        if self.ui.button(cx, ids!(share_url_button)).clicked(actions)
            || self.ui.button(cx, ids!(share_curl_button)).clicked(actions)
        {
            let Some(gw) = &self.gateway else { return };
            let base = lan::listen_base_url(gw.handle.listen.port());
            let token = gw.operator_token();
            let curl = format!(
                "curl -s -H \"X-Api-Key: {token}\" -H 'Content-Type: application/json' \\\n  -d '{{\"to\":\"+15551212\",\"body\":\"hello\",\"idempotency_key\":\"demo-1\"}}' \\\n  {base}/v1/messages"
            );
            let share_curl = self.ui.button(cx, ids!(share_curl_button)).clicked(actions);
            let text = if share_curl {
                format!("Wadhu SMS Gateway\n\n{curl}")
            } else {
                format!(
                    "Wadhu SMS Gateway (advanced)\n\nURL: {base}\nOperator key: {token}\n\nPrefer: Allow a new app + code on the phone."
                )
            };
            cx.share_text(&text);
            cx.copy_to_clipboard(&text);
            self.ui.label(cx, ids!(result_label)).set_text(
                cx,
                if share_curl {
                    "Shared curl example (also copied)"
                } else {
                    "Shared advanced URL + key (also copied)"
                },
            );
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
        }
        if self.ui.button(cx, ids!(copy_key_button)).clicked(actions) {
            let Some(gw) = &self.gateway else { return };
            let token = gw.operator_token();
            cx.copy_to_clipboard(&token);
            cx.share_text(&token);
            self.ui
                .label(cx, ids!(result_label))
                .set_text(cx, "Operator key copied / shared");
            self.ui
                .label(cx, ids!(result_label))
                .set_text_color(cx, color_lime());
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
            #[cfg(target_os = "android")]
            if !permissions::sms_permission_granted() {
                self.ui.label(cx, ids!(result_label)).set_text(
                    cx,
                    "Allow SMS permission first (Do Next above)",
                );
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text_color(cx, color_fail());
                return;
            }
            if let Some(gw) = &self.gateway {
                let caps = gw.handle.radio.capabilities();
                if !caps.available {
                    self.ui.label(cx, ids!(result_label)).set_text(
                        cx,
                        &format!("SIM not ready: {}", humanize_radio_detail(&caps.detail)),
                    );
                    self.ui
                        .label(cx, ids!(result_label))
                        .set_text_color(cx, color_fail());
                    return;
                }
                match gw.handle.outbox.accept(String::new(), to, body) {
                    Ok(m) => {
                        self.last_test_id = Some(m.id.clone());
                        self.ui.label(cx, ids!(result_label)).set_text(
                            cx,
                            &format!("Queued for {} — sending via SIM…", m.to),
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
        let token = gw.operator_token();
        let base = lan::listen_base_url(listen.port());

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

        let lan_chip = if lan::local_ipv4().is_some() {
            "Listening"
        } else {
            "No Wi-Fi IP"
        };
        self.ui.label(cx, ids!(api_chip)).set_text(cx, lan_chip);
        self.ui.label(cx, ids!(api_chip)).set_text_color(
            cx,
            if lan::local_ipv4().is_some() {
                color_ok()
            } else {
                color_warn()
            },
        );

        let bt = gw.handle.bluetooth.snapshot();
        let (bt_text, bt_color) = if bt.listening {
            ("On", color_ok())
        } else if bt.detail.contains("permission") {
            ("Need BT", color_warn())
        } else if bt.detail.contains("adapter_off") {
            ("BT off", color_warn())
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
                "{} — {}\nSent {submitted} · failed {failed}",
                caps.label,
                humanize_radio_detail(&caps.detail)
            ),
        );

        self.ui.markdown(cx, ids!(listen_label)).set_text(cx, &base);
        self.ui
            .markdown(cx, ids!(token_label))
            .set_text(cx, &format!("Operator key: {token}"));

        if let Some((code, _)) = gw.handle.clients.pairing_code() {
            self.ui
                .label(cx, ids!(pair_code_label))
                .set_text(cx, &code);
        }

        let clients = gw.handle.clients.snapshot();
        let clients_text = if clients.is_empty() {
            "No apps connected yet.".into()
        } else {
            clients
                .iter()
                .map(|c| {
                    let state = if c.blocked { "Blocked" } else { "Allowed" };
                    format!("• {} — {} SMS — {state}", c.label, c.sent_count)
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        self.ui
            .markdown(cx, ids!(clients_label))
            .set_text(cx, &clients_text);

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
                format!("{}  →  {}{}", status_label(m.status), m.to, err)
            })
            .collect();
        let text = if lines.is_empty() {
            "No messages yet — send one above.".into()
        } else {
            lines.join("\n")
        };
        self.ui.markdown(cx, ids!(recent_label)).set_text(cx, &text);

        // Keep the test-send line in sync with the outbox — never leave it on
        // "Queued…" after the radio already accepted the message.
        if let Some(id) = &self.last_test_id {
            if let Some(m) = gw.handle.outbox.get(id) {
                let (msg, color) = match m.status {
                    gateway::message::Status::Queued
                    | gateway::message::Status::Submitting => (
                        format!("Sending to {} via SIM…", m.to),
                        color_lime(),
                    ),
                    gateway::message::Status::Submitted
                    | gateway::message::Status::Delivered => (
                        format!(
                            "Sent to {} — SIM handed it to the carrier",
                            m.to
                        ),
                        color_ok(),
                    ),
                    gateway::message::Status::Failed => (
                        format!(
                            "Failed to {}: {}",
                            m.to,
                            m.error.as_deref().unwrap_or("radio error")
                        ),
                        color_fail(),
                    ),
                    other => (
                        format!("{} — {}", status_label(other), m.to),
                        color_muted(),
                    ),
                };
                self.ui.label(cx, ids!(result_label)).set_text(cx, &msg);
                self.ui
                    .label(cx, ids!(result_label))
                    .set_text_color(cx, color);
            }
        }

        #[cfg(target_os = "android")]
        let sms_ok = permissions::sms_permission_granted();
        #[cfg(not(target_os = "android"))]
        let sms_ok = false;

        let next = if cfg!(not(target_os = "android")) {
            "Desktop dry-run: API listens on localhost. Install on Android to use a SIM."
                .to_string()
        } else if !sms_ok {
            "1) Tap Allow SMS permission — without it this device cannot send."
                .to_string()
        } else if !caps.available {
            format!(
                "SMS allowed, but SIM not ready: {}. Check airplane mode / signal.",
                humanize_radio_detail(&caps.detail)
            )
        } else if submitted + failed + pending == 0 {
            "SIM ready. Send a real SMS below, or share the Listen URL with a client."
                .to_string()
        } else {
            format!(
                "Gateway live at {base}. Sent {submitted} · failed {failed} · pending {pending}."
            )
        };
        self.ui
            .markdown(cx, ids!(next_step_label))
            .set_text(cx, &next);

        self.ui.button(cx, ids!(perm_button)).set_text(
            cx,
            if sms_ok {
                "SMS permission granted"
            } else {
                "Allow SMS permission"
            },
        );
    }
}

fn humanize_radio_detail(detail: &str) -> String {
    if detail == "ready" {
        return "ready".into();
    }
    if detail == "permission_needed" {
        return "SMS permission needed".into();
    }
    if detail == "no_telephony" {
        return "no cellular radio on this device".into();
    }
    if detail == "error" {
        return "could not read SIM status".into();
    }
    if let Some(rest) = detail.strip_prefix("sim_not_ready:") {
        return format!("SIM not ready (state {rest})");
    }
    if detail.is_empty() {
        return "unknown".into();
    }
    detail.replace('_', " ")
}

fn status_label(status: gateway::message::Status) -> &'static str {
    use gateway::message::Status::*;
    match status {
        Queued => "Queued",
        Submitting => "Sending",
        Submitted => "Sent",
        Delivered => "Delivered",
        Failed => "Failed",
        Expired => "Expired",
        Cancelled => "Cancelled",
        Indeterminate => "Unknown",
    }
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

impl App {
    fn text_field_consumes_back(&mut self, cx: &mut Cx) -> bool {
        let fields = [
            ids!(relay_url_input),
            ids!(relay_device_id_input),
            ids!(relay_secret_input),
            ids!(client_name_input),
            ids!(to_input),
            ids!(body_input),
        ];
        for id in fields {
            let field = self.ui.text_input(cx, id);
            if cx.has_key_focus(field.area()) {
                cx.set_key_focus(Area::Empty);
                return true;
            }
        }
        false
    }

    fn show_exit_back_hint(&mut self, cx: &mut Cx) {
        self.ui
            .markdown(cx, ids!(exit_back_notice))
            .set_text(cx, PRESS_BACK_AGAIN_EXIT);
    }

    fn clear_exit_back_hint(&mut self, cx: &mut Cx) {
        self.ui.markdown(cx, ids!(exit_back_notice)).set_text(cx, "");
    }

    /// Single-screen gateway: natural back at root uses double-press to exit.
    fn handle_visit_root_back(&mut self, cx: &mut Cx) -> bool {
        if self.text_field_consumes_back(cx) {
            clear_exit_back_arm();
            self.clear_exit_back_hint(cx);
            return true;
        }
        match note_exit_back_press(cx) {
            Ok(true) => {
                self.show_exit_back_hint(cx);
                true
            }
            Ok(false) => false, // Android: unhandled → moveTaskToBack
            Err(()) => true,
        }
    }

    /// Left-edge swipe → back / double-back-to-exit (64css edge, 56px travel).
    fn edge_swipe_pointer(&mut self, cx: &mut Cx, event: &Event) -> bool {
        static ORIGIN: OnceLock<std::sync::Mutex<Option<DVec2>>> = OnceLock::new();
        static TOUCH: OnceLock<std::sync::Mutex<Option<(u64, DVec2)>>> = OnceLock::new();
        let mouse = ORIGIN.get_or_init(|| std::sync::Mutex::new(None));
        let touch = TOUCH.get_or_init(|| std::sync::Mutex::new(None));
        let go = |start: DVec2, end: DVec2, app: &mut App, cx: &mut Cx| -> bool {
            let dx = end.x - start.x;
            let dy = (end.y - start.y).abs();
            if dx > 56.0 && dx > dy * 1.25 {
                app.handle_visit_root_back(cx)
            } else {
                false
            }
        };
        match event {
            Event::MouseDown(e) if e.button.is_primary() => {
                let mut o = mouse.lock().unwrap();
                *o = if e.abs.x < 64.0 {
                    Some(e.abs)
                } else {
                    None
                };
                false
            }
            Event::MouseUp(e) if e.button.is_primary() => {
                if let Some(start) = mouse.lock().unwrap().take() {
                    go(start, e.abs, self, cx)
                } else {
                    false
                }
            }
            Event::TouchUpdate(e) => {
                let mut o = touch.lock().unwrap();
                let mut pair = None;
                for t in &e.touches {
                    match t.state {
                        TouchState::Start => {
                            if t.abs.x < 64.0 {
                                *o = Some((t.uid, t.abs));
                            }
                        }
                        TouchState::Stop => {
                            if let Some((uid, start)) = *o
                                && uid == t.uid
                            {
                                pair = Some((start, t.abs));
                                *o = None;
                            }
                        }
                        _ => {}
                    }
                }
                drop(o);
                if let Some((start, end)) = pair {
                    go(start, end, self, cx)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

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
        let one_row = size.x >= ACTIONS_ONE_ROW_FROM;
        let flow = if one_row {
            Flow::right()
        } else {
            Flow::Down
        };
        for path in [ids!(share_row), ids!(client_ops)] {
            if let Some(mut view) = self.ui.view(cx, path).borrow_mut() {
                view.layout.flow = flow;
            }
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
            Event::MouseDown(mouse) if mouse.button.is_back() => {
                if self.handle_visit_root_back(cx) {
                    return;
                }
            }
            Event::KeyDown(key)
                if key.key_code == KeyCode::Escape
                    || (key.key_code == KeyCode::Back && !cfg!(target_os = "android"))
                    || (key.key_code == KeyCode::ArrowLeft && key.modifiers.alt) =>
            {
                // Android KEYCODE_BACK is Event::BackPressed only — KeyCode::Back
                // here would double-fire. Desktop XF86Back / Escape / Alt+← share exit arm.
                if self.handle_visit_root_back(cx) {
                    return;
                }
            }
            _ => {}
        }
        if self.edge_swipe_pointer(cx, event) {
            return;
        }
    }
}
