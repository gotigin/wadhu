//! wadhu.com — one Makepad scrolling brand SPA (Khaynjo pattern).
//!
//! One Hosting tree: this wasm home + `/privacy` `/terms` `/cancellation`
//! `/about` HTML (presented from `docs/legal/`) + Analytics. Legal is not an
//! in-app route; nav opens those pages with `open_link` so this SPA stays loaded.

pub use makepad_widgets;

use makepad_widgets::*;

app_main!(App);

include!("ui.rs");

const PUBLIC_HOST: &str = "https://wadhu.com";
const PUBLIC_CONNECT: &str = "https://connect.wadhu.com/";
const LOCAL_CONNECT_PORT: u16 = 3847;

const HOME_PATH: &str = "/";
const PRIVACY_PATH: &str = "/privacy";
const TERMS_PATH: &str = "/terms";
const CANCELLATION_PATH: &str = "/cancellation";
const ABOUT_PATH: &str = "/about";
const ACCOUNT_PATH: &str = "/account";

// Three product cards; stack until there is room for a row.
const CARDS_SIDE_BY_SIDE_FROM: f64 = 960.0;
const MIN_WIDTH: f64 = 320.0;
const MAX_CONTENT: f64 = 1100.0;
const MAX_CONTENT_WIDE: f64 = 1280.0;

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
            // Extra top pad so section heads clear the sticky brand+chip bar
            // when scrolled (phone evidence 2026-11-22; PRODUCTS still kissed
            // the chips at 48 — raise once more).
            Self::Phone => 56.0,
            Self::Narrow => 52.0,
            _ => 64.0,
        }
    }
}

const NAV_INLINE_FROM: f64 = 960.0;

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    laid_out: Option<(Tier, i64, i64)>,
    #[rust]
    nav_open: bool,
}

fn open_site(cx: &mut Cx, url: &str) {
    // Same-tab for wadhu.com paths (home SPA + legal HTML) — matches gotigin.com.
    #[cfg(target_arch = "wasm32")]
    {
        cx.open_url(url, OpenUrlInPlace::Yes);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        open_external(cx, url);
    }
}

fn open_external(cx: &mut Cx, url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        cx.open_url(url, OpenUrlInPlace::No);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = cx;
        #[cfg(target_os = "windows")]
        let spawned = std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn();
        #[cfg(target_os = "macos")]
        let spawned = std::process::Command::new("open").arg(url).spawn();
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let spawned = std::process::Command::new("xdg-open").arg(url).spawn();
        if let Err(err) = spawned {
            log!("could not open {url}: {err}");
        }
    }
}

impl App {
    fn site_url(cx: &Cx, path: &str) -> String {
        if matches!(cx.os_type(), OsType::Web(_)) {
            path.to_string()
        } else {
            format!("{PUBLIC_HOST}{path}")
        }
    }

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

        for path in [
            ids!(hero),
            ids!(what),
            ids!(products),
            ids!(reuse),
            ids!(about),
            ids!(closing),
        ] {
            if let Some(mut view) = self.ui.view(cx, path).borrow_mut() {
                view.layout.padding = band_pad;
            }
        }
        let nav_inline = layout_w >= NAV_INLINE_FROM;
        if nav_inline {
            self.nav_open = false;
        }
        if let Some(mut bar) = self.ui.view(cx, ids!(top)).borrow_mut() {
            let pad = if tier == Tier::Phone { 12.0 } else { 24.0 };
            bar.layout.padding = Inset {
                left: pad,
                right: pad,
                top: 12.0,
                bottom: 12.0,
            };
            bar.layout.flow = if nav_inline { Flow::right() } else { Flow::Down };
            bar.layout.spacing = if nav_inline { 10.0 } else { 8.0 };
        }
        if let Some(mut chrome) = self.ui.view(cx, ids!(top_chrome)).borrow_mut() {
            chrome.walk.width = Size::fill();
        }
        self.ui
            .button(cx, ids!(nav_toggle))
            .set_visible(cx, !nav_inline);
        self.ui.button(cx, ids!(nav_toggle)).set_text(
            cx,
            if self.nav_open { "Close" } else { "Menu" },
        );
        let show_nav = nav_inline || self.nav_open;
        self.ui.view(cx, ids!(topnav)).set_visible(cx, show_nav);
        if let Some(mut nav) = self.ui.view(cx, ids!(topnav)).borrow_mut() {
            nav.layout.flow = if nav_inline {
                Flow::right()
            } else {
                Flow::Down
            };
            nav.walk.width = if nav_inline {
                Size::fit()
            } else {
                Size::fill()
            };
            nav.layout.align.x = if nav_inline { 1.0 } else { 0.0 };
        }

        if let Some(mut cards) = self.ui.view(cx, ids!(cards)).borrow_mut() {
            cards.layout.flow = if layout_w >= CARDS_SIDE_BY_SIDE_FROM {
                Flow::right()
            } else {
                Flow::Down
            };
        }

        // Type steps down with the window: a 48 px headline takes three
        // lines of a phone.
        let (title_px, lede_px) = match tier {
            Tier::Phone => (30.0, 17.0),
            Tier::Narrow => (36.0, 18.0),
            Tier::Tablet => (42.0, 19.0),
            Tier::Desk | Tier::Wide => (48.0, 19.0),
        };
        for id in [
            ids!(hero_title),
            ids!(what_title),
            ids!(reuse_title),
            ids!(products_title),
            ids!(about_title),
            ids!(closing_title),
        ] {
            Self::set_md_size(cx, &self.ui, id, title_px);
        }
        for id in [ids!(hero_lede), ids!(what_lede), ids!(products_lede)] {
            Self::set_md_size(cx, &self.ui, id, lede_px);
        }
    }

    fn set_md_size(cx: &mut Cx, ui: &WidgetRef, id: &[LiveId], size: f64) {
        if let Some(mut md) = ui.markdown(cx, id).borrow_mut() {
            let px = size as f32;
            md.font_size = px;
            md.text_style_normal.font_size = px;
            md.text_style_bold.font_size = px;
            md.draw_text.text_style.font_size = px;
            md.redraw(cx);
        }
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.ui
            .window(cx, ids!(main_window))
            .set_title(cx, "Wadhu — Tools · Apps · More");
        self.apply_responsive(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_toggle)).clicked(actions) {
            self.nav_open = !self.nav_open;
            self.laid_out = None;
            self.apply_responsive(cx);
            return;
        }

        if self
            .ui
            .view(cx, ids!(brand_home))
            .finger_up(actions)
            .is_some_and(|e| e.was_tap())
        {
            open_site(cx, &Self::site_url(cx, HOME_PATH));
        }

        let mut close_nav = false;
        let privacy = self.ui.button(cx, ids!(nav_privacy)).clicked(actions)
            || self.ui.button(cx, ids!(cta_privacy_foot)).clicked(actions);
        if privacy {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, PRIVACY_PATH));
        }

        if self.ui.button(cx, ids!(nav_terms)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, TERMS_PATH));
        }

        if self.ui.button(cx, ids!(nav_cancellation)).clicked(actions)
            || self.ui.button(cx, ids!(cta_cancellation)).clicked(actions)
        {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, CANCELLATION_PATH));
        }

        if self.ui.button(cx, ids!(nav_about)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, ABOUT_PATH));
        }

        if self.ui.button(cx, ids!(nav_account)).clicked(actions) {
            close_nav = true;
            open_site(cx, &Self::site_url(cx, ACCOUNT_PATH));
        }

        if self.ui.button(cx, ids!(open_connect)).clicked(actions) {
            open_external(cx, &Self::connect_url(cx));
        }

        if close_nav && self.nav_open {
            self.nav_open = false;
            self.laid_out = None;
            self.apply_responsive(cx);
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
