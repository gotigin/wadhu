# crates/

Reusable, self-contained Wadhu libraries. Package `wadhu-<crate>`, lib
`wadhu_<crate>`.

- A crate **never depends on a sibling crate or on any app**, so a consumer
  can take exactly one crate.
- Makepad only via `makepad-widgets = { workspace = true }`; UI behind an
  optional `widgets` feature when the crate also has a non-UI core.
- Brand-neutral, SemVer per crate, `CHANGELOG.md`, tests through the
  public API.
- Create one with `./scripts/new-crate.sh <name> [--widgets]` (copies
  [`../templates/crate/`](../templates/crate/README.md) or
  [`../templates/widgets/`](../templates/widgets/README.md)).

Rules: [`../docs/CRATES.md`](../docs/CRATES.md) · using a crate from another
workspace: [`../docs/DEV.md`](../docs/DEV.md) § Consuming Wadhu from another workspace.

| Crate | What |
|---|---|
| [`sms-client/`](sms-client/README.md) | HTTP helpers for SMS Gateway / SMS Relay `/v1` (native sync + wasm async). Demo: [`apps/sms-client-demo`](../apps/sms-client-demo) |
