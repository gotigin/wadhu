# apps/

Independent Wadhu apps and tools: standalone programs that other apps or
people need (helpers, bridges, services, desktop/mobile apps). Package
`wadhu-<app>`.

- An app **never depends on a sibling app**. It may depend on crates in
  [`../crates/`](../crates/README.md).
- Each app owns its `env/`, `scripts/`, `docs/` and its
  `tools/<app>-dev` shim.
- Create one with `./scripts/new-app.sh <name>` (copies
  [`../templates/app/`](../templates/app/README.md)).

Rules: [`../docs/WADHU_VISION_AND_PRINCIPLES.md`](../docs/WADHU_VISION_AND_PRINCIPLES.md) §3 ·
how-to: [`../docs/DEV.md`](../docs/DEV.md).

| App | What |
|---|---|
| [`sms-gateway/`](sms-gateway/README.md) | **Android-first SIM gateway** (phones/tablets/…): LAN + Bluetooth + optional relay — [`docs/LOCAL_SMS.md`](../docs/LOCAL_SMS.md) |
| [`sms-relay/`](sms-relay/README.md) | **Self-hostable remote plane:** internet clients → paired Android gateways |
| [`wadhu-connect/`](wadhu-connect/README.md) | **Wadhu Connect** — WhatsApp Cloud API Tech Provider bridge (Rust): ES, webhooks, send · https://connect.wadhu.com |
| [`gotigin-connect/`](gotigin-connect/README.md) | Legacy Node prototype — use `wadhu-connect` |
