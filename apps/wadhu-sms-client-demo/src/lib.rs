//! SMS Client Demo — the non-UI half.
//!
//! Logic that tests and the Makepad shell both call lives in this library;
//! `main.rs` is only the shell. Keep it that way as the app grows: anything
//! worth testing belongs here (or in a Wadhu crate under `crates/` when it is
//! reusable by other apps).

#[cfg(target_os = "android")]
pub use makepad_widgets;

pub mod identity;

// Android packaging compiles this library as the cdylib. The desktop
// binary's `app_main!` is not in that library, so without this the
// Activity finds no native entry point and the process dies at launch.
#[cfg(target_os = "android")]
#[path = "main.rs"]
mod shell;
