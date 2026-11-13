# wadhu-template-crate

Starter for a plain-Rust Wadhu library crate. Copy it:

```bash
./scripts/new-crate.sh <name>
```

Replace this README with: what the crate does (one line), a minimal usage
example, its feature flags, its MSRV, and its stability (`0.x` = may break).
Rules every Wadhu crate follows: [`docs/CRATES.md`](../../docs/CRATES.md).

```toml
[dependencies]
wadhu-template-crate = { path = "../../crates/template-crate" }
```

```rust
use wadhu_template_crate::slugify;
assert_eq!(slugify("Tools, Apps & More!").unwrap(), "tools-apps-more");
```
