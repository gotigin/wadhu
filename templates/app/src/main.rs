// Production Windows: GUI subsystem (no console). Dev keeps a console for logs.
// `wadhu_hide_console` is set by build.rs when WADHU_ENV=production.
#![cfg_attr(
    all(target_os = "windows", wadhu_hide_console),
    windows_subsystem = "windows"
)]

//! Wadhu template app — the Makepad 2 shell.
//!
//! Copy this app with `scripts/new-app.sh <name>`; do not depend on it.
//! Makepad 2 only: `script_mod!` + `AppMain`, never Makepad 1 `live_design!`.

pub use makepad_widgets;

// Android compiles this file into the library (`crate`). The desktop binary
// links the same package's library as `wadhu_template_app`.
#[cfg(target_os = "android")]
use crate::identity;
#[cfg(not(target_os = "android"))]
use wadhu_template_app::identity;

use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "Wadhu Template"
                window.inner_size: vec2(520, 360)
                body +: {
                    SolidView{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 14
                        padding: 28
                        align: Center
                        draw_bg +: {color: #x0d0b3a}

                        title_label := Label{
                            text: "Wadhu"
                            draw_text.color: #xfcfbfb
                            draw_text.text_style.font_size: 30
                        }
                        tagline_label := Label{
                            text: "Tools · Apps · More — by Gotigin"
                            draw_text.color: #xadfb15
                            draw_text.text_style.font_size: 12
                        }
                        identity_label := Label{
                            text: ""
                            draw_text.color: #x9fb4d0
                            draw_text.text_style.font_size: 10
                        }
                        ping_button := Button{
                            text: "Ping"
                        }
                        ping_label := Label{
                            text: "Not pinged yet"
                            draw_text.color: #x00c8fc
                            draw_text.text_style.font_size: 12
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
    pings: u64,
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
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(ping_button)).clicked(actions) {
            self.pings += 1;
            let text = match self.pings {
                1 => "Pinged once".to_string(),
                n => format!("Pinged {n} times"),
            };
            self.ui.label(cx, ids!(ping_label)).set_text(cx, &text);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        // Register each Wadhu widget crate this app uses, after
        // makepad_widgets and before the app's own module, e.g.:
        // wadhu_<crate>::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
