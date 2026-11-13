//! Who this build is: app id, label and environment, baked in by `build.rs`
//! from `env/development.env` or `env/production.env`.

/// `development` or `production`.
pub const ENV: &str = env!("WADHU_ENV");
/// Reverse-DNS id: Android package, desktop data-dir key, Apple bundle id.
pub const APP_ID: &str = env!("WADHU_APP_ID");
/// Human-facing name: window title, launcher label.
pub const APP_LABEL: &str = env!("WADHU_APP_LABEL");

/// True for development / debug builds.
pub fn is_development() -> bool {
    ENV == "development"
}

/// One line for logs and the dev caption: `Label · id · env`.
pub fn summary() -> String {
    format!("{APP_LABEL} · {APP_ID} · {ENV}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_is_a_known_profile() {
        assert!(
            ENV == "development" || ENV == "production",
            "unexpected WADHU_ENV {ENV:?}"
        );
    }

    #[test]
    fn development_ids_are_marked_debug() {
        // Dev and prod installs must never share an id (data dirs, packages).
        assert_eq!(
            is_development(),
            APP_ID.ends_with(".debug"),
            "APP_ID {APP_ID:?} for ENV {ENV:?}"
        );
    }

    #[test]
    fn summary_names_every_part() {
        let s = summary();
        assert!(s.contains(APP_LABEL) && s.contains(APP_ID) && s.contains(ENV));
    }
}
