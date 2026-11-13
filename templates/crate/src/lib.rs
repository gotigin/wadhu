//! Wadhu template crate — a plain-Rust, brand-neutral library.
//!
//! Replace this example API with the crate's real one. What a Wadhu crate
//! looks like (docs/CRATES.md):
//!
//! - one clear job, documented here with a runnable example;
//! - no dependency on a sibling Wadhu crate or app, no brand/product logic;
//! - errors as a typed enum, never `panic!` on bad input;
//! - unit tests next to the code, integration tests in `tests/`.
//!
//! # Example
//!
//! ```
//! use wadhu_template_crate::slugify;
//!
//! assert_eq!(slugify("Tools, Apps & More!").unwrap(), "tools-apps-more");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::fmt;

/// Why [`slugify`] could not produce a slug.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlugError {
    /// The input had no ASCII letters or digits to keep.
    Empty,
}

impl fmt::Display for SlugError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SlugError::Empty => f.write_str("input has no letters or digits to make a slug from"),
        }
    }
}

impl std::error::Error for SlugError {}

/// Lower-case ASCII slug: letters and digits kept, every other run of
/// characters collapsed to a single `-`, no leading or trailing `-`.
pub fn slugify(input: &str) -> Result<String, SlugError> {
    let mut out = String::with_capacity(input.len());
    let mut pending_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    if out.is_empty() {
        Err(SlugError::Empty)
    } else {
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_separators() {
        assert_eq!(slugify("  a -- b__c  ").unwrap(), "a-b-c");
    }

    #[test]
    fn rejects_input_without_letters_or_digits() {
        assert_eq!(slugify(" -- !! "), Err(SlugError::Empty));
    }
}
