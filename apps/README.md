# apps/

Independent Wadhu apps and tools: standalone programs that other apps or
people need (helpers, bridges, services, desktop/mobile apps). Package
`wadhu-<app>`.

Naming: **Wadhu** = brand; **Gotigin** = trademark; company =
GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED; products below are under Wadhu.
See [`../docs/BRAND.md`](../docs/BRAND.md).

- An app **never depends on a sibling app**. It may depend on crates in
  [`../crates/`](../crates/README.md).
- Each app owns its `env/`, `scripts/`, `docs/` and its
  `tools/<app>-dev` shim.
- Create one with `./scripts/new-app.sh <name>` (copies
  [`../templates/app/`](../templates/app/README.md)).

Rules: [`../docs/WADHU_VISION_AND_PRINCIPLES.md`](../docs/WADHU_VISION_AND_PRINCIPLES.md) §3 ·
how-to: [`../docs/DEV.md`](../docs/DEV.md).

| App | Product | What |
|---|---|---|
| [`wadhu-sms-gateway/`](wadhu-sms-gateway/README.md) | **SMS Gateway** | Android-first SIM gateway (LAN + Bluetooth + optional relay) |
| [`wadhu-sms-relay/`](wadhu-sms-relay/README.md) | **SMS Relay** | Self-hostable remote plane: internet clients → paired gateways |
| [`wadhu-connect/`](wadhu-connect/README.md) | **Wadhu Connect** | WhatsApp Cloud API Tech Provider bridge (Rust) · https://connect.wadhu.com |
| [`wadhu-sms-client-demo/`](wadhu-sms-client-demo/README.md) | (demo) | Desktop demo for `wadhu-sms-client` |

There is **no** `gotigin-connect` app. The Meta product name is **Wadhu Connect**.
