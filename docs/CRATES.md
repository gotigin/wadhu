# Wadhu crates — rules for reusable libraries

A Wadhu crate is a library that apps at Gotigin, and outside it, can take
on its own. Why these rules exist:
[`WADHU_VISION_AND_PRINCIPLES.md`](WADHU_VISION_AND_PRINCIPLES.md) §3–5.
How to create one: `./scripts/new-crate.sh <name> [--widgets]`
([`DEV.md`](DEV.md) § Adding an app or a crate).

## Shape

| | Convention |
|---|---|
| Directory | `crates/<name>/` |
| Package / lib | `wadhu-<name>` / `wadhu_<name>` |
| Files | `Cargo.toml`, `README.md`, `CHANGELOG.md`, `src/lib.rs`, `tests/`, `examples/` (widget crates: `examples/demo.rs`) |
| Manifest | `edition`, `license`, `authors` from the workspace; own `version`, `description`, `rust-version` |
| Dependencies | external crates only; shared versions via `{ workspace = true }` |

## Independence

- **No sibling dependencies.** A crate never depends on another Wadhu
  crate or on an app (enforced by `scripts/check-independence.sh`). If two
  crates need the same helper, each keeps its own copy.
- **Makepad only through the workspace dependency**
  (`makepad-widgets = { workspace = true }`), never a `vendor/` path.
- **Brand-neutral.** No Gotigin product logic, product names, ids or
  styling defaults that only suit one brand. Brand colours belong in the
  consuming app.

## Features

- Keep default features minimal. A consumer should pay only for what it
  uses.
- **Logic and UI in one crate:** when a crate has both a core (protocol,
  format, engine) and Makepad widgets for it, put the widgets behind an
  optional feature of the same crate. Do not create a second crate that
  depends on the first, because §3.2 forbids that:

  ```toml
  [features]
  widgets = ["dep:makepad-widgets"]

  [dependencies]
  makepad-widgets = { workspace = true, optional = true }
  ```

  ```rust
  #[cfg(feature = "widgets")]
  pub mod widgets;  // pub fn script_mod(vm) lives here
  ```

- Plain-logic crates do not depend on Makepad at all.

## API

- Crate docs (`//!`) state the one job and show a runnable example
  (doctest). `#![warn(missing_docs)]`; `#![forbid(unsafe_code)]` unless
  unsafe is the point of the crate (then document every `unsafe` block).
- Errors are a typed `enum` implementing `std::error::Error`. Never
  `panic!`/`unwrap` on caller input.
- No hidden global state, threads or logging spam. Long or blocking work
  (I/O, devices, network) must be callable off the UI thread, and say so
  in the docs.
- Widget crates: prefix script names with the crate name (`EscposPreview`),
  expose `pub fn script_mod(vm: &mut ScriptVm)`, and give each widget a
  typed `<Name>Ref` API (setters on the Ref, not just on the struct).

## Versioning

- [SemVer](https://semver.org/) per crate. Crates release independently;
  there is no "Wadhu version".
- While `0.x`, a breaking change bumps the minor version; from `1.0`, the
  major.
- Every change that a consumer could notice gets a `CHANGELOG.md` entry
  ([Keep a Changelog](https://keepachangelog.com/en/1.1.0/)) in the same
  commit, together with the version bump if it is a release.
- Remove API in two steps: `#[deprecated(note = "use …")]` in one release,
  removal in a later breaking release.
- Tag releases `wadhu-<name>-v<version>` once the repository has a remote.
  Git consumers pin those tags.

## MSRV

Declare `rust-version` in every crate (the oldest Rust it supports;
edition 2024 needs ≥ 1.85). The workspace resolver is MSRV-aware, so the
lowest declared value constrains the shared lockfile ([`DEV.md`](DEV.md)
§ Lockfile and MSRV). Makepad-based crates need whatever toolchain the
vendored Makepad needs.

## Tests

- Unit tests next to the code; **integration tests in `tests/` that use
  only the public API**, the way a consumer would.
- Doctests for the examples in docs.
- Widget crates: a runnable `examples/demo.rs`, checked on screen under a
  private Xvfb ([`DEV.md`](DEV.md) § Look at the screen).
- `cargo test -p wadhu-<name>` and `./scripts/check-independence.sh` pass
  before every commit.

## Licensing and publishing

- `license = "MIT OR Apache-2.0"` (from the workspace). License texts:
  [`LICENSE-MIT`](../LICENSE-MIT) and [`LICENSE-APACHE`](../LICENSE-APACHE).
- `publish = false` until a crate is ready for outside users. Then remove
  it, fill `description`, `readme`, `keywords`, `categories` and
  `repository`, and run `cargo publish --dry-run -p wadhu-<name>`.
- crates.io accepts a crate only if every dependency resolves from the
  registry. **Makepad-based crates cannot be published while
  `makepad-widgets` 2.x is not on crates.io** (only up to 1.0.0 on
  2026-11-08). Until then, share them by path or git. Plain crates can be
  published any time.
