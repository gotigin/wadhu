//! wadhu.com — Wadhu brand site (Makepad 2).
//! Static legal pages also ship under `static/` for Firebase.

pub use makepad_widgets;

use makepad_widgets::*;

app_main!(App);

include!("ui.rs");

const CHAPTERS: [(&str, &str); 5] = [
    ("hero", "Home"),
    ("products", "Products"),
    ("connect", "Connect"),
    ("about", "About"),
    ("close", "More"),
];

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    chapter: usize,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.ui
            .window(cx, ids!(main_window))
            .set_title(cx, "Wadhu — Tools · Apps · More");
        self.go_chapter(cx, 0);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_privacy)).clicked(actions) {
            cx.open_url("https://wadhu.com/privacy", OpenUrlInPlace::No);
        }
        if self.ui.button(cx, ids!(nav_terms)).clicked(actions) {
            cx.open_url("https://wadhu.com/terms", OpenUrlInPlace::No);
        }
        if self.ui.button(cx, ids!(nav_connect)).clicked(actions)
            || self.ui.button(cx, ids!(cta_connect)).clicked(actions)
        {
            cx.open_url("https://connect.wadhu.com/", OpenUrlInPlace::No);
        }
        if self.ui.button(cx, ids!(dots.d0)).clicked(actions) {
            self.go_chapter(cx, 0);
        }
        if self.ui.button(cx, ids!(dots.d1)).clicked(actions) {
            self.go_chapter(cx, 1);
        }
        if self.ui.button(cx, ids!(dots.d2)).clicked(actions) {
            self.go_chapter(cx, 2);
        }
        if self.ui.button(cx, ids!(dots.d3)).clicked(actions) {
            self.go_chapter(cx, 3);
        }
        if self.ui.button(cx, ids!(dots.d4)).clicked(actions) {
            self.go_chapter(cx, 4);
        }
        let slides = self.ui.slides_view(cx, ids!(slides));
        let goal = slides.goal_slide() as usize;
        if goal != self.chapter && goal < CHAPTERS.len() {
            self.chapter = goal;
        }
    }
}

impl App {
    fn go_chapter(&mut self, cx: &mut Cx, i: usize) {
        self.chapter = i.min(CHAPTERS.len() - 1);
        self.ui
            .slides_view(cx, ids!(slides))
            .set_goal_slide(cx, self.chapter);
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
