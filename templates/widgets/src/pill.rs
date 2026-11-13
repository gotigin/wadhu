//! `TemplatePill` — a rounded label chip.
//!
//! Script names are global across every crate an app registers, so a
//! reusable crate prefixes its widget names with its own name
//! (`TemplatePill`, not `Pill`) to never collide with a sibling crate.

use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.TemplatePillBase = #(TemplatePill::register_widget(vm))
    mod.widgets.TemplatePill = set_type_default() do mod.widgets.TemplatePillBase {
        width: Fit
        height: Fit
        chip := RoundedView {
            width: Fit
            height: Fit
            padding: Inset{left: 12 top: 6 right: 12 bottom: 6}
            show_bg: true
            draw_bg +: {
                color: #x161252
                radius: 10.0
            }
            label := Label {
                text: ""
                draw_text.color: #xadfb15
                draw_text.text_style.font_size: 11
            }
        }
    }
}

/// A rounded chip showing one short line of text.
#[derive(Script, ScriptHook, Widget)]
pub struct TemplatePill {
    #[deref]
    view: View,
}

impl Widget for TemplatePill {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.view.label(cx, ids!(chip.label)).set_text(cx, text);
    }
}

impl TemplatePillRef {
    /// Set the chip's text. No-op on an empty ref.
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            Widget::set_text(&mut *inner, cx, text);
        }
    }
}
