# Warnings — fix, never suppress

**Strict rule.** Every platform Wadhu builds for must compile **without
warnings**. Do not silence them.

## Platforms in scope

| Target | How we build |
| --- | --- |
| Host Linux (`x86_64-unknown-linux-gnu`) | `cargo build --workspace --all-targets` |
| Windows (`x86_64-pc-windows-gnu`) | `cargo check -p <crate> --target x86_64-pc-windows-gnu` / windows-dev |
| Android (`aarch64-linux-android`, `x86_64-linux-android`) | `cargo-makepad` / `cargo <app>-android-dev` (the **lib**/cdylib — not a bare `cargo check` of the desktop `bin`) |
| Wasm (`wasm32-unknown-unknown`) | `cargo-makepad wasm` / `sites/*/scripts/run-web.sh` |

Java / JNI bridge code under Android packaging must also stay warning-clean
(no `@SuppressWarnings` to hide real issues). Prefer APIs that do not warn on
the compile SDK (see `docs/plans/BT_THEN_IDENTITY.md`).

## Do

- Fix the code: use the value, delete dead code, wire `Drop`/`stop`, add a
  real test that exercises a helper, remove redundant Cargo.toml fields.
- Re-check **each** target you touch (host is not enough).
- Treat new warnings in CI / local scripts as failures.

## Do not

- `#[allow(…)]` / `#![allow(…)]` to hide unused, dead_code, or similar
- `@SuppressWarnings` / compiler flag hacks to bury warnings
- `RUSTFLAGS` / `.cargo/config` that only demotes or ignores warnings
- Claiming “clean” after checking only one OS or only the desktop bin

Rare exception: an **upstream** false positive you cannot fix without forking
— document the exact lint, why, and a tracking note in the same PR. Prefer a
patch in `patches/makepad/` over an allow in product code.

## Check before finishing

```bash
./scripts/check-warnings.sh
```

That script fails on any rustc/cargo warning for the workspace members and
the cross targets listed above (Android as `--lib` where applicable).
