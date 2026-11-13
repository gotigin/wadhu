# Wadhu workspace — agent guide

Wadhu (by Gotigin, *Tools · Apps · More*) is Gotigin's Rust + Makepad 2
workspace for **reusable code** that other Gotigin brands' apps (e.g. Kumkaj)
and outside users need. Read before changing anything:

- `docs/WADHU_VISION_AND_PRINCIPLES.md`: what belongs here; **§3 independence rules**
- `docs/DEV.md`: layout, commands, env/ids, Makepad 2 notes, consuming from other workspaces
- `docs/CRATES.md`: rules for reusable crates
- `.cursor/rules/*.mdc`: the same rules in checklist form

Non-negotiables:

1. Apps never depend on sibling apps; crates never depend on sibling crates
   or apps; apps/sites may depend on crates. Run `./scripts/check-independence.sh`.
2. Crates take Makepad only as `makepad-widgets = { workspace = true }`.
3. Makepad 2 only. Never edit `vendor/makepad`; change `patches/makepad/`
   and rerun `./scripts/setup-makepad.sh`.
4. No brand product/business logic in Wadhu; crates stay brand-neutral.
5. New parts come from `./scripts/new-app.sh` / `./scripts/new-crate.sh`.
6. Before finishing: `cargo build --workspace --all-targets`,
   `cargo test --workspace`, the independence check, and for UI changes a
   screenshot on a private Xvfb (never `:0`).
