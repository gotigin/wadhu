//! wadhu.com — scrolling Makepad 2 brand site.
//! Static legal pages also ship under `static/` for Firebase / Meta.
//! In-app Privacy/Terms/About keep the wasm shell loaded (no reload on back).
//! Layout is responsive (Khaynjo-style tiers on window size).

pub use makepad_widgets;

use makepad_widgets::*;

app_main!(App);

include!("ui.rs");

const PUBLIC_CONNECT: &str = "https://connect.wadhu.com/";
const LOCAL_CONNECT_PORT: u16 = 3847;

const PRIVACY_MD: &str = include_str!("../../../docs/legal/PRIVACY.md");
const TERMS_MD: &str = include_str!("../../../docs/legal/TERMS.md");
const ABOUT_MD: &str = include_str!("../../../docs/legal/ABOUT.md");

const TAG_BESIDE_LOGO_FROM: f64 = 720.0;
const CARDS_SIDE_BY_SIDE_FROM: f64 = 680.0;
const MIN_WIDTH: f64 = 320.0;
const MAX_CONTENT: f64 = 1100.0;
const MAX_CONTENT_WIDE: f64 = 1280.0;

const COLOR_INK: Vec4 = vec4(0.988, 0.984, 0.984, 1.0); // #fcfbfb

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Route {
    #[default]
    Home,
    Privacy,
    Terms,
    About,
}

impl Route {
    fn hash(self) -> &'static str {
        match self {
            Self::Home => "",
            Self::Privacy => "privacy",
            Self::Terms => "terms",
            Self::About => "about",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Home => "Wadhu — Tools · Apps · More",
            Self::Privacy => "Privacy — Wadhu",
            Self::Terms => "Terms — Wadhu",
            Self::About => "About — Wadhu",
        }
    }

    fn markdown(self) -> Option<&'static str> {
        match self {
            Self::Home => None,
            Self::Privacy => Some(PRIVACY_MD),
            Self::Terms => Some(TERMS_MD),
            Self::About => Some(ABOUT_MD),
        }
    }

    fn from_web_hash(hash: &str) -> Self {
        match hash.trim_start_matches('#').trim() {
            "privacy" => Self::Privacy,
            "terms" => Self::Terms,
            "about" => Self::About,
            _ => Self::Home,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tier {
    Phone,
    Narrow,
    Tablet,
    Desk,
    Wide,
}

impl Tier {
    fn from_size(w: f64, h: f64) -> Self {
        if w < 480.0 {
            Self::Phone
        } else if w < 720.0 {
            Self::Narrow
        } else if w < 1024.0 {
            Self::Tablet
        } else if w >= 1600.0 && h >= 900.0 {
            Self::Wide
        } else {
            Self::Desk
        }
    }

    fn max_content(self) -> f64 {
        if self == Self::Wide {
            MAX_CONTENT_WIDE
        } else {
            MAX_CONTENT
        }
    }

    fn side_pad(self) -> f64 {
        match self {
            Self::Phone => 16.0,
            Self::Narrow => 24.0,
            Self::Tablet => 40.0,
            Self::Desk | Self::Wide => 64.0,
        }
    }

    fn band_pad_v(self) -> f64 {
        match self {
            Self::Phone => 36.0,
            Self::Narrow => 48.0,
            _ => 64.0,
        }
    }
}

/// Makepad MarkdownLink defaults to the literal word "Button" if link text
/// does not bind cleanly after `set_text`. For in-app legal, show labels only.
fn legal_md_for_canvas(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut chars = src.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '[' {
            out.push(c);
            continue;
        }
        let mut label = String::new();
        let mut closed = false;
        for ch in chars.by_ref() {
            if ch == ']' {
                closed = true;
                break;
            }
            label.push(ch);
        }
        if !closed {
            out.push('[');
            out.push_str(&label);
            continue;
        }
        if chars.peek() == Some(&'(') {
            chars.next();
            for ch in chars.by_ref() {
                if ch == ')' {
                    break;
                }
            }
            let clean = label.trim_matches('`');
            out.push_str(clean);
        } else {
            out.push('[');
            out.push_str(&label);
            out.push(']');
        }
    }
    out
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    route: Route,
    #[rust]
    laid_out: Option<(Tier, i64, i64)>,
}

impl App {
    fn connect_url(cx: &Cx) -> String {
        if let OsType::Web(page) = cx.os_type() {
            if page.hostname == "127.0.0.1" || page.hostname == "localhost" {
                return format!(
                    "{}//{}:{LOCAL_CONNECT_PORT}/onboard",
                    page.protocol, page.hostname
                );
            }
        }
        format!("{PUBLIC_CONNECT}onboard")
    }

    fn route_from_location(cx: &Cx) -> Route {
        match cx.os_type() {
            OsType::Web(page) => Route::from_web_hash(&page.hash),
            _ => Route::Home,
        }
    }

    fn paint_nav_active(&mut self, cx: &mut Cx) {
        // Idle GhostBtn + active LinkBtn pairs (draw_bg is private on Button).
        let pairs = [
            (ids!(nav_privacy), ids!(nav_privacy_on), Route::Privacy),
            (ids!(nav_terms), ids!(nav_terms_on), Route::Terms),
            (ids!(nav_about), ids!(nav_about_on), Route::About),
        ];
        for (idle, on, route) in pairs {
            let active = self.route == route;
            self.ui.button(cx, idle).set_visible(cx, !active);
            self.ui.button(cx, on).set_visible(cx, active);
        }
        // Back: keep GhostBtn ink (never dull default label color).
        if let Some(mut btn) = self.ui.button(cx, ids!(nav_back)).borrow_mut() {
            btn.draw_text.color = COLOR_INK;
        }
        self.ui.view(cx, ids!(top)).redraw(cx);
    }

    fn apply_route(&mut self, cx: &mut Cx, route: Route, push_history: bool) {
        self.route = route;
        let on_home = route == Route::Home;
        self.ui.view(cx, ids!(page)).set_visible(cx, on_home);
        self.ui.view(cx, ids!(legal)).set_visible(cx, !on_home);
        self.ui.button(cx, ids!(nav_back)).set_visible(cx, !on_home);

        if let Some(md) = route.markdown() {
            let body = legal_md_for_canvas(md);
            self.ui.widget(cx, ids!(legal_md)).set_text(cx, &body);
        }

        self.ui
            .window(cx, ids!(main_window))
            .set_title(cx, route.title());

        self.paint_nav_active(cx);

        if matches!(cx.os_type(), OsType::Web(_)) {
            let url = if route.hash().is_empty() {
                "/".to_string()
            } else {
                format!("/#{}", route.hash())
            };
            cx.browser_update_url(&url, !push_history);
        }
    }

    fn sync_route_from_browser(&mut self, cx: &mut Cx) {
        let route = Self::route_from_location(cx);
        if route != self.route {
            self.apply_route(cx, route, false);
        }
    }

    fn apply_responsive(&mut self, cx: &mut Cx) {
        let size = self.ui.window(cx, ids!(main_window)).get_inner_size(cx);
        if size.x < 1.0 || size.y < 1.0 {
            return;
        }
        let layout_w = size.x.max(MIN_WIDTH);
        let tier = Tier::from_size(layout_w, size.y);
        let key = (tier, layout_w.round() as i64, size.y.round() as i64);
        if self.laid_out == Some(key) {
            return;
        }
        self.laid_out = Some(key);

        let min_side = tier.side_pad();
        let side = min_side.max((layout_w - tier.max_content()) / 2.0);
        let pad_v = tier.band_pad_v();
        let band_pad = Inset {
            left: side,
            right: side,
            top: pad_v,
            bottom: pad_v,
        };

        for path in [ids!(hero), ids!(products), ids!(connect), ids!(about), ids!(close)] {
            if let Some(mut view) = self.ui.view(cx, path).borrow_mut() {
                view.layout.padding = band_pad;
            }
        }
        if let Some(mut view) = self.ui.view(cx, ids!(legal_pad)).borrow_mut() {
            view.layout.padding = Inset {
                left: side,
                right: side,
                top: 24.0,
                bottom: 80.0,
            };
        }
        if let Some(mut bar) = self.ui.view(cx, ids!(top)).borrow_mut() {
            let pad = if tier == Tier::Phone { 12.0 } else { 24.0 };
            bar.layout.padding = Inset {
                left: pad,
                right: pad,
                top: 12.0,
                bottom: 12.0,
            };
        }

        let tag_beside = layout_w >= TAG_BESIDE_LOGO_FROM;
        self.ui
            .widget(cx, ids!(brand_tag))
            .set_visible(cx, tag_beside);
        self.ui
            .widget(cx, ids!(hero_tag))
            .set_visible(cx, !tag_beside);

        if let Some(mut cards) = self.ui.view(cx, ids!(cards)).borrow_mut() {
            cards.layout.flow = if layout_w >= CARDS_SIDE_BY_SIDE_FROM {
                Flow::right()
            } else {
                Flow::Down
            };
        }

        self.paint_nav_active(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let route = Self::route_from_location(cx);
        self.apply_route(cx, route, false);
        self.apply_responsive(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_back)).clicked(actions) {
            self.apply_route(cx, Route::Home, true);
            return;
        }

        let privacy = self.ui.button(cx, ids!(nav_privacy)).clicked(actions)
            || self.ui.button(cx, ids!(nav_privacy_on)).clicked(actions)
            || self.ui.button(cx, ids!(cta_privacy)).clicked(actions);
        if privacy {
            self.apply_route(cx, Route::Privacy, true);
            return;
        }

        if self.ui.button(cx, ids!(nav_terms)).clicked(actions)
            || self.ui.button(cx, ids!(nav_terms_on)).clicked(actions)
        {
            self.apply_route(cx, Route::Terms, true);
            return;
        }

        if self.ui.button(cx, ids!(nav_about)).clicked(actions)
            || self.ui.button(cx, ids!(nav_about_on)).clicked(actions)
        {
            self.apply_route(cx, Route::About, true);
            return;
        }

        let connect = self.ui.button(cx, ids!(nav_connect)).clicked(actions)
            || self.ui.button(cx, ids!(cta_connect)).clicked(actions)
            || self.ui.button(cx, ids!(open_connect)).clicked(actions);
        if connect {
            let url = Self::connect_url(cx);
            cx.open_url(&url, OpenUrlInPlace::Yes);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Signal = event {
            self.sync_route_from_browser(cx);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
        match event {
            Event::Startup | Event::WindowGeomChange(_) => self.apply_responsive(cx),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::legal_md_for_canvas;

    #[test]
    fn strips_markdown_links_to_labels() {
        let s = legal_md_for_canvas(
            "Naming: [`docs/BRAND.md`](../BRAND.md). Privacy: [wadhu.com/privacy](https://wadhu.com/privacy)",
        );
        assert!(!s.contains("Button"));
        assert!(s.contains("docs/BRAND.md"));
        assert!(s.contains("wadhu.com/privacy"));
        assert!(!s.contains("]("));
    }
}
