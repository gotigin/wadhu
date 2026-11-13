//! Wadhu template widget crate — a reusable Makepad 2 widget library.
//!
//! # Using it from an app
//!
//! ```toml
//! [dependencies]
//! wadhu-template-widgets = { path = "../../crates/template-widgets" }
//! ```
//!
//! Register the widgets once, after `makepad_widgets` and before the app's
//! own script module, then use them in the app's `script_mod!` with
//! `use mod.widgets.*`:
//!
//! ```ignore
//! fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
//!     crate::makepad_widgets::script_mod(vm);
//!     wadhu_template_widgets::script_mod(vm);
//!     self::script_mod(vm)
//! }
//! ```
//!
//! Runnable demo: `cargo run -p wadhu-template-widgets --example demo`.

pub mod pill;

pub use pill::{TemplatePill, TemplatePillRef, TemplatePillWidgetRefExt};

use makepad_widgets::ScriptVm;

/// Register this crate's widgets (`mod.widgets.TemplatePill`, …) with the
/// script VM. A host app calls this once, after `makepad_widgets::script_mod`.
pub fn script_mod(vm: &mut ScriptVm) {
    crate::pill::script_mod(vm);
}
