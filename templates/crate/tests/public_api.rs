//! Integration tests: exercise the crate only through its public API, the
//! way a consumer in another workspace would.

use wadhu_template_crate::{SlugError, slugify};

#[test]
fn slug_is_stable_for_brand_names() {
    assert_eq!(slugify("Wadhu by Gotigin").unwrap(), "wadhu-by-gotigin");
}

#[test]
fn error_is_displayable() {
    let err = slugify("").unwrap_err();
    assert_eq!(err, SlugError::Empty);
    assert!(!err.to_string().is_empty());
}
