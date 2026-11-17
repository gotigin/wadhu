//! wadhu.com — one Makepad scrolling brand SPA (Khaynjo pattern).
//!
//! One Hosting tree: this wasm home + `/privacy` `/terms` `/about` HTML
//! (presented from `docs/legal/`) + Analytics. Legal is not an in-app route;
//! nav opens those pages with `open_link` (new tab) so this SPA stays loaded.

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
const ABOUT_PATH: &str = "/about";

const CARDS_SIDE_BY_SIDE_FROM: f64 = 680.0;
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
            Self::Phone => 36.0,
            Self::Narrow => 48.0,
            _ => 64.0,
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    laid_out: Option<(Tier, i64, i64)>,
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

        for path in [ids!(hero), ids!(products), ids!(connect), ids!(about), ids!(close)] {
            if let Some(mut view) = self.ui.view(cx, path).borrow_mut() {
                view.layout.padding = band_pad;
            }
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

        if let Some(mut cards) = self.ui.view(cx, ids!(cards)).borrow_mut() {
            cards.layout.flow = if layout_w >= CARDS_SIDE_BY_SIDE_FROM {
                Flow::right()
            } else {
                Flow::Down
            };
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
        if self
            .ui
            .view(cx, ids!(brand_home))
            .finger_up(actions)
            .is_some_and(|e| e.was_tap())
        {
            open_site(cx, &Self::site_url(cx, HOME_PATH));
        }

        let privacy = self.ui.button(cx, ids!(nav_privacy)).clicked(actions)
            || self.ui.button(cx, ids!(cta_privacy)).clicked(actions);
        if privacy {
            open_site(cx, &Self::site_url(cx, PRIVACY_PATH));
        }

        if self.ui.button(cx, ids!(nav_terms)).clicked(actions) {
            open_site(cx, &Self::site_url(cx, TERMS_PATH));
        }

        if self.ui.button(cx, ids!(nav_about)).clicked(actions) {
            open_site(cx, &Self::site_url(cx, ABOUT_PATH));
        }

        let connect = self.ui.button(cx, ids!(nav_connect)).clicked(actions)
            || self.ui.button(cx, ids!(cta_connect)).clicked(actions)
            || self.ui.button(cx, ids!(open_connect)).clicked(actions);
        if connect {
            open_external(cx, &Self::connect_url(cx));
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
