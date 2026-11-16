# Gotigin WhatsApp Cloud API coexistence — diagnosis (2026-11-15)

Evidence from Meta Business Suite screenshots, Chrome history, and Meta docs.
No IDs or approval statuses invented.

## Existing assets (confirmed)

| Asset | ID / value | Evidence |
| --- | --- | --- |
| Business portfolio | `1076913081550690` | Business Suite URL + settings |
| Portfolio name | Gotigin Software & Hardware Private Limited (earlier UI still showed “Mearaj Bhagad”) | Screenshots |
| Facebook Page Gotigin | `1233972929807232` | Business assets |
| Messaging account “Mearaj” (type: WhatsApp Business app) | `147378475125876` | Messaging accounts + WhatsApp Manager |
| Phone on Messaging account | `+91 99677 17702`, display name Mearaj, status Offline | WhatsApp Manager phone numbers |
| Phone Number ID (Business App / Manager) | `152212324639075` | WhatsApp Manager profile |
| Incomplete WhatsApp account asset | `1008869008221270` | `whatsapp_business_presence` selected_asset_id |
| Incomplete asset display name | Gotigin Software & Hardware Private Limited, **Unverified** | Summary tab |
| Meta Developer app seen in Chrome history | **BB.VoiceArtist** `1112191535534260` | Only app ID in history today |

## Root cause of registration conflict

1. `+91 99677 17702` is already registered on the **WhatsApp Business mobile app**, surfaced in Business Suite as Messaging account **Mearaj** (`147378475125876`).
2. The Business Suite **WhatsApp accounts → Verify now / standard phone verification** path is the **dedicated Cloud API registration** flow. Meta correctly rejects it with **Phone Number In Use** (and related code-request failures / rate limit). Message text: migrate or disconnect first.
3. That is **not** coexistence. Coexistence is only offered through **Embedded Signup** with `featureType: "whatsapp_business_app_onboarding"` (requires Solution Partner or Tech Provider per Meta docs).
4. Asset `1008869008221270` is an **incomplete / unverified WhatsApp account shell** created while attempting standard registration. Empty “Linked WhatsApp accounts” does **not** mean the number is free — the live registration is the Messaging account + mobile app.
5. Phone Number ID `152212324639075` must **not** be assumed to be a Cloud API sender. Manager shows Business-app messaging account; Cloud API capability is unproven until Graph returns `platform_type=CLOUD_API` and (for coexistence) `is_on_biz_app=true`.

## What not to do (already violated / still forbidden)

- Do not keep retrying SMS/voice verification on the standard Verify flow (already hit **Verification code limit exceeded**).
- Do not delete Messaging account Mearaj or disconnect the Business app to “free” the number for API-only registration.
- Do not treat BB.VoiceArtist as the Gotigin WhatsApp product app until WhatsApp use-case + Embedded Signup are confirmed on it (history shows FB Login / App Review pages, not WhatsApp setup).

## Required Meta path

1. Meta app owned by portfolio `1076913081550690` with **WhatsApp** use case.
2. **Tech Provider onboarding** (or Solution Partner) — Meta documents this as a hard requirement for Business-app coexistence Embedded Signup.
3. Facebook Login for Business + Embedded Signup configuration + session logging.
4. Launch ES with coexistence extras; complete in-app Connect / code paste; then server-side code exchange, `subscribed_apps`, `/register`, webhook digest, optional history sync.
5. App Review advanced access for `whatsapp_business_messaging` + `whatsapp_business_management` before production client onboarding (own-business testing may proceed further in Development mode with role users — verify in dashboard, do not assume).

## Implementation in this repo

[`apps/wadhu-connect`](../../wadhu-connect/README.md) — Rust Connect (Embedded Signup
UI, server-side code exchange, webhook verify + signature check, persistence under
`./data`). Production host: `https://connect.wadhu.com`.

## 2026-11-15 update — coexistence hard gate

Embedded Signup with `whatsapp_business_app_onboarding` launches, but Meta rejects
`+919967717702` with: partner app isn't set up yet to connect WhatsApp Business app
numbers. Business verification is Approved; App Review still needs Allowed usage /
videos. Webhook must be `https://connect.wadhu.com/webhook` once Connect is deployed.
