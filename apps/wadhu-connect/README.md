# Wadhu Connect

WhatsApp Cloud API **Tech Provider** bridge for Gotigin brands and integrators.
Businesses keep their own WhatsApp Business number; brand apps send/receive
through Cloud API after Embedded Signup.

| | |
| --- | --- |
| Production | https://connect.wadhu.com |
| Brand / privacy | https://wadhu.com · https://wadhu.com/privacy |
| Language | Rust (std HTTP + Graph client) |
| Meta app | Wadhu Connect (rename from Gotigin Connect if still showing) |

## What it does

1. Serves the Embedded Signup landing UI (`public/`).
2. Exchanges the OAuth code **server-side** (app secret never in the browser).
3. Subscribes the WABA, registers the phone (coexistence), optional history sync.
4. Receives signed webhooks at `/webhook`.
5. Sends test / brand-app messages via `/api/send-test` and `/v1/messages`.

See [`docs/INTEGRATOR.md`](docs/INTEGRATOR.md) for Kumkaj / brand-app integration.
Auth & monetization doctrine: [`../../docs/IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](../../docs/IDENTITY_AND_MONETIZATION_ALIGNMENT.md).

## Run locally

```bash
cd apps/wadhu-connect
cp -n .env.example .env   # fill META_APP_ID / SECRET / CONFIG_ID
# from workspace root:
cargo wadhu-connect-dev
# → http://127.0.0.1:3847
```

For Meta webhooks / ES redirect, put HTTPS in front (ngrok/cloudflared) and set
`PUBLIC_ORIGIN` to that URL. Production `PUBLIC_ORIGIN=https://connect.wadhu.com`.

## Deploy (connect.wadhu.com)

Run the binary behind TLS (systemd, Docker, Fly, Cloud Run, …) with the same
env vars. Point DNS `connect.wadhu.com` at that host. Meta App Dashboard:

- Website Site URL: `https://wadhu.com` or `https://connect.wadhu.com`
- Privacy: `https://wadhu.com/privacy`
- Webhook: `https://connect.wadhu.com/webhook`

## Legacy

`apps/gotigin-connect` was the Node prototype. Prefer this crate.
