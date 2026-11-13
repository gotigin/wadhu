//! Runnable demo for this widget crate — the crate's own showcase, so it can
//! be seen and checked without any app depending on it.
//!
//! `cargo run -p wadhu-template-widgets --example demo`

pub use makepad_widgets;

use makepad_widgets::*;
use wadhu_template_widgets::TemplatePillWidgetRefExt;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "TemplatePill demo"
                window.inner_size: vec2(420, 240)
                body +: {
                    SolidView{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 12
                        align: Center
                        draw_bg +: {color: #x0d0b3a}

                        pill := TemplatePill{}
                        bump_button := Button{ text: "Bump" }
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
    bumps: u64,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.ui
            .template_pill(cx, ids!(pill))
            .set_text(cx, "Tools · Apps · More");
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(bump_button)).clicked(actions) {
            self.bumps += 1;
            let text = format!("Bumped {}×", self.bumps);
            self.ui.template_pill(cx, ids!(pill)).set_text(cx, &text);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        wadhu_template_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
