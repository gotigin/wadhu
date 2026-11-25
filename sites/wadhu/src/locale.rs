//! wadhu.com words. English is the source (`EN`, keyed by a stable id);
//! every other catalogue language is `translations/<code>.tsv` in this crate
//! — one `key<TAB>text` line per `EN` key, in this order, `{placeholders}`
//! kept. The runtime (catalogue, `#l=` bridge, packs, the widget walker) is
//! `sites/_shared/i18n/site_i18n.rs`.
//!
//! **Page copy** in `ui.rs` is matched by its English text: a Label /
//! Markdown / Button whose text is an `EN` value shows that key's
//! translation. Text the site sets from code goes through `App::tr`.
//! `tests::every_ui_string_is_keyed_or_skipped` fails when `ui.rs` gains
//! copy that is neither here nor in `UI_SKIP` (names, numbers, ids).
//!
//! Legal page bodies stay English. Account chrome is `data-i18n` / `T()`.
//!
//! **New or changed English:** edit `EN`, run
//! `./scripts/scaffold-site-translations.sh wadhu` (adds the key to every
//! pack as the English source), then translate for real. Hindi and Urdu
//! must not stay English; other packs may until someone translates them.
//! `@@EN@@` stand-ins fail the pack test.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Every site key, in pack order. One pair per line — the scaffold parser
/// reads it that way. Do not rustfmt this array onto several lines.
#[rustfmt::skip]
pub const EN: &[(&str, &str)] = &[
    ("menu", "Menu"),
    ("close", "Close"),
    ("home", "Home"),
    ("about", "About"),
    ("privacy", "Privacy"),
    ("terms", "Terms"),
    ("cancellation", "Cancellation"),
    ("data_deletion", "Data deletion"),
    ("support", "Support"),
    ("account", "Account"),
    ("email", "Email"),
    ("window_title", "Wadhu — Tools · Apps · More"),
    ("tagline", "Tools · Apps · More"),
    ("hero_kicker", "WADHU · BY GOTIGIN"),
    ("hero_title", "Wadhu means more."),
    ("hero_lede", "Reusable tools, utils, and apps — Tools · Apps · More."),
    ("the_brand", "THE BRAND"),
    ("what_title", "Tools, utils, and apps."),
    ("what_lede", "Written once, so other Gotigin products and anyone else can take exactly the piece they need."),
    ("what_body", "Libraries, small programs, and services live here. Each part stands on its own."),
    ("reuse", "REUSE"),
    ("reuse_title", "Take one piece."),
    ("reuse_body", "A crate is a library. An app is a program you run beside your own. A site is a page you visit. One part changing never forces a sibling to change."),
    ("products", "PRODUCTS"),
    ("products_title", "Under the Wadhu brand"),
    ("products_lede", "Products that ship under this brand."),
    ("sms_gateway_blurb", "Android-first SIM gateway. LAN and Bluetooth stay free forever."),
    ("sms_relay_blurb", "Optional internet path to a paired gateway. Local send does not need it."),
    ("connect_blurb", "WhatsApp Cloud API Tech Provider bridge. Businesses keep their number; apps send and receive after Embedded Signup."),
    ("open_connect", "Open Wadhu Connect"),
    ("names", "NAMES"),
    ("about_title", "Who we are"),
    ("trademark", "TRADEMARK"),
    ("find_it", "Memon for “find it” · pronounced go-tee-geen"),
    ("slogan", "Where Technology Meets Discovery"),
    ("company", "COMPANY"),
    ("director_line", "Director · mearaj@gmail.com"),
    ("brand", "BRAND"),
    ("more_line", "Memon for “more” · Tools · Apps · More"),
    ("products_line", "SIM gateway, optional relay, and a WhatsApp Cloud API bridge"),
    ("next", "NEXT"),
    ("closing_title", "Build with Wadhu."),
    ("closing_body", "A wadhu.com account is for hosted features. Local tools over LAN and Bluetooth stay free."),
    ("acct_page_title", "Account — Wadhu"),
    ("acct_lede", "Sign in for hosted Wadhu features. Local tools over LAN and Bluetooth stay usable without an account."),
    ("acct_continue_with", "Continue with email, a provider, or phone."),
    ("acct_name_optional", "Name (optional)"),
    ("acct_password", "Password"),
    ("acct_continue", "Continue"),
    ("acct_or_continue", "or continue with"),
    ("acct_federated", "Federated sign-in"),
    ("acct_phone", "Phone"),
    ("acct_mobile", "Mobile number"),
    ("acct_phone_placeholder", "+91…"),
    ("acct_send_sms", "Send SMS code"),
    ("acct_sms_code", "SMS code"),
    ("acct_confirm_code", "Confirm code"),
    ("acct_signed_in", "Signed in"),
    ("acct_signed_in_as", "Signed in as"),
    ("acct_status_label", "Status"),
    ("acct_resend_verify", "Resend verification email"),
    ("acct_sign_out", "Sign out"),
    ("acct_home_aria", "Wadhu home"),
    ("acct_nav_aria", "Primary"),
    ("company_privacy", "Company privacy"),
    ("err_invalid_email", "Enter a valid email address."),
    ("err_weak_password", "Use a password with at least 6 characters."),
    ("err_wrong_password", "Wrong password."),
    ("err_invalid_credential", "Email or password is wrong."),
    ("err_popup_closed", "Sign-in window was closed."),
    ("err_cancelled", "Sign-in was cancelled."),
    ("err_unauthorized_domain", "This domain is not authorized for Firebase Auth."),
    ("err_not_allowed", "That sign-in method is not enabled yet."),
    ("err_too_many", "Too many attempts. Try again later."),
    ("err_invalid_phone", "Use a full phone number with country code (e.g. +91…)."),
    ("err_bad_code", "That SMS code is wrong or expired."),
    ("err_generic", "Something went wrong."),
    ("acct_no_email", "(no email)"),
    ("acct_email_verified", "Email verified"),
    ("acct_email_unverified", "Email not verified yet — check your inbox"),
    ("acct_phone_signed_in", "Signed in with phone"),
    ("acct_you_are_in", "You’re signed in."),
    ("acct_verify_sent", "Verification email sent — check your inbox"),
];

/// `ui.rs` text that is not site copy: brand and product names, the legal
/// name, contact lines, and the unused LinkBtn template label.
#[cfg_attr(not(test), allow(dead_code))]
pub const UI_SKIP: &[&str] = &[
    "Open",
    "EN",
    "Wadhu",
    "Gotigin",
    "SMS Gateway",
    "SMS Relay",
    "Wadhu Connect",
    "SMS Gateway ·",
    "SMS Relay ·",
    "GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED",
    "support@gotigin.com",
    "© 2026 GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED",
    "Gotigin · Wadhu · wadhu.com",
];

/// English for `key` ("" for an unknown key — tests forbid that in code).
pub fn en(key: &str) -> &'static str {
    EN.iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .unwrap_or("")
}

/// The key whose English is exactly `english` (page copy lookup).
pub fn key_for(english: &str) -> Option<&'static str> {
    static REVERSE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    REVERSE
        .get_or_init(|| EN.iter().map(|(k, v)| (*v, *k)).collect())
        .get(english)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site_i18n::checks;

    const DIR: &str = env!("CARGO_MANIFEST_DIR");

    fn keys() -> Vec<&'static str> {
        EN.iter().map(|(k, _)| *k).collect()
    }

    #[test]
    fn keys_are_unique_and_english_is_unique() {
        let mut seen_k = std::collections::HashSet::new();
        let mut seen_v = std::collections::HashSet::new();
        for (k, v) in EN {
            assert!(seen_k.insert(*k), "duplicate key {k}");
            assert!(seen_v.insert(*v), "duplicate English {v:?} (two keys would fight over one widget)");
            assert!(!v.trim().is_empty(), "{k}: empty English");
        }
    }

    /// Every table language has a pack with every key, in `EN` order, with
    /// the same placeholders and no `@@EN@@` stand-in. Untranslated packs
    /// may still hold the English source; Hindi and Urdu may not.
    #[test]
    fn every_catalogue_language_translates_every_key() {
        checks::every_language_translates_every_key(DIR, &keys(), &en);
    }

    #[test]
    fn hi_and_ur_are_real_translations() {
        let mut problems = Vec::new();
        for code in ["hi", "ur"] {
            let pack = checks::read_pack(DIR, code).unwrap_or_else(|| {
                panic!("{code}: translations/{code}.tsv missing");
            });
            for (key, english) in EN {
                let Some(value) = pack.get(*key) else {
                    problems.push(format!("{code}/{key}: missing"));
                    continue;
                };
                if value.starts_with(crate::site_i18n::STAND_IN) {
                    problems.push(format!("{code}/{key}: still @@EN@@"));
                }
                if value == english {
                    problems.push(format!("{code}/{key}: still English"));
                }
            }
        }
        assert!(problems.is_empty(), "hi/ur packs:\n{}", problems.join("\n"));
    }

    #[test]
    fn every_translation_draws_with_the_site_fonts() {
        checks::every_pack_draws_with_its_fonts(DIR, &crate::script_font_files);
    }

    #[test]
    fn words_are_marked_where_the_script_has_no_spaces() {
        checks::words_are_marked_in_scripts_without_spaces(DIR);
    }

    #[test]
    fn every_ui_string_is_keyed_or_skipped() {
        checks::every_ui_string_is_keyed_or_skipped(include_str!("ui.rs"), &key_for, UI_SKIP);
    }

    /// Account page (`data-i18n`) and the account script (`T("key", "English")`).
    /// Legal bodies stay English and are not in this list.
    #[test]
    fn html_and_script_copy_match_en() {
        checks::html_and_script_copy_match_en(
            DIR,
            &["static/account/index.html"],
            &["client/account-app.js"],
            &en,
        );
    }
}
