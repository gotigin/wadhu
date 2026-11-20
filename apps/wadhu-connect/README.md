# Wadhu Connect

**Product** under the Wadhu brand (*more* — Tools · Apps · More). WhatsApp
Cloud API **Tech Provider** bridge for Gotigin brands and integrators.
Businesses keep their own WhatsApp Business number; brand apps send/receive
through Cloud API after Embedded Signup.

| | |
| --- | --- |
| Production | https://connect.wadhu.com |
| Brand | https://wadhu.com |
| Privacy | https://wadhu.com/privacy ← [`docs/legal/PRIVACY.md`](../../docs/legal/PRIVACY.md) |
| Terms | https://wadhu.com/terms |
| Data deletion | https://wadhu.com/data-deletion |
| Support | https://wadhu.com/support |
| Language | Rust (std HTTP + Graph client) |
| Meta app display name | **Wadhu Connect** |

## Legal — one Markdown file each (Khaynjo pattern)

Edit only [`docs/legal/`](../../docs/legal/README.md). Connect embeds those
files and serves formatted `/privacy`, `/terms`, `/data-deletion`,
`/support`, `/about`. wadhu.com presents the same files via
`sites/wadhu/scripts/present-legal.sh`.

## What it does

1. Serves the Embedded Signup landing UI (`public/`).
2. Exchanges the OAuth code **server-side** (app secret never in the browser).
3. Subscribes the WABA, registers the phone (coexistence), optional history sync.
4. Receives signed webhooks at `/webhook`.
5. Sends test / brand-app messages via `/api/send-test` and `/v1/messages`.

See [`docs/INTEGRATOR.md`](docs/INTEGRATOR.md). Meta paste:
[`docs/DATA_HANDLING_AND_REVIEWER.md`](docs/DATA_HANDLING_AND_REVIEWER.md).

## Run locally

```bash
cd apps/wadhu-connect
cp -n .env.example .env
cargo wadhu-connect-dev
# → http://127.0.0.1:3847/privacy
```

## Deploy (connect.wadhu.com)

Run the binary behind TLS. Meta App Dashboard:

- Website Site URL: `https://wadhu.com` or `https://connect.wadhu.com`
- Privacy: `https://wadhu.com/privacy`
- Data deletion: `https://wadhu.com/data-deletion`
- Webhook: `https://connect.wadhu.com/webhook`
