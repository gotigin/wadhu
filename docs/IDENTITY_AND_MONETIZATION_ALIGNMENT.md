# Identity & monetization — alignment note (post-Bluetooth)

Short written reconciliation of the ChatGPT Wadhu identity/SMS platform brief
with what already exists. **Not** a build checklist for a full IdP.

Source brief (external): `CHATGPT-WADHU-IDENTITY-SMS-GATEWAY-PLAN.md`  
Doctrine: [`LOCAL_SMS.md`](LOCAL_SMS.md) · Vision: [`WADHU_VISION_AND_PRINCIPLES.md`](WADHU_VISION_AND_PRINCIPLES.md) §3

------------------------------------------------------------------------

## Locked product split

| Layer | Who pays | Needs wadhu.com account? |
|-------|----------|---------------------------|
| Phone SIM SMS (carrier) | SIM owner | No |
| LAN / Bluetooth to own phone | **Free forever** | No |
| Hosted relay + org entitlements + multi-device cloud | Wadhu subscription | Yes |
| Brand app (Moklanjo, Khaynjo, …) | That brand’s pricing | Brand account; Wadhu **Connect** only if they use cloud SMS and/or [Wadhu Connect](../apps/wadhu-connect/README.md) (WhatsApp Cloud API) |
| Wadhu Connect (WhatsApp Cloud API pipe) | Wadhu / platform (when billed) | Yes for multi-tenant cloud; own-business / Tech Provider testing can run on connect.wadhu.com |

Registration on **wadhu.com** is for **cloud** features. Brand apps either
(a) talk to the local gateway (LAN/BT + phone API key), or (b) OAuth-connect to
Wadhu for short-lived relay credentials. No permanent org secrets in APKs.

------------------------------------------------------------------------

## What already matches

| ChatGPT idea | Reality today |
|--------------|---------------|
| Independent SMS gateway | [`apps/wadhu-sms-gateway`](../apps/wadhu-sms-gateway) + [`crates/wadhu-sms-client`](../crates/wadhu-sms-client) |
| One job pipeline, multi-transport | Outbox + LAN HTTP + Classic RFCOMM + lab TCP framing |
| LAN/BT free; cloud optional | This doctrine + [`BLUETOOTH.md`](../apps/wadhu-sms-gateway/docs/BLUETOOTH.md) |
| No shared core across brand apps | Vision §3 |
| Relay for internet | [`apps/wadhu-sms-relay`](../apps/wadhu-sms-relay) (admin keys / pairing — **not** full org billing) |

------------------------------------------------------------------------

## Agree (direction)

- Do **not** put permanent org secrets in brand APKs.
- Do **not** make one Kumkaj app call another for SMS.
- Do **not** disable LAN/BT when a relay subscription expires.
- One Wadhu platform, four registered client apps — apps never depend on sibling apps.
- Identity provider ≠ business DB (Firebase UID must not be the billing FK).
- Email verification mandatory for email signup; SMS OTP ≠ substitute for email.
- Monetize **hosted relay / platform only**.

------------------------------------------------------------------------

## Timing / sequencing (disagree with “build IdP now”)

```text
Done / now:   Classic RFCOMM + LAN HTTP offline story; stable /v1 + client crate
Next:         Freeze OpenAPI + client docs for brand apps (LAN + BT)
Later:        Identity → orgs → hosted relay entitlements → Connect UX → Stripe
```

Do **not** block brand-app SMS on Firebase/Stripe. Block only on **stable `/v1` +
client crate + working LAN/BT**.

ChatGPT stages 2–7 are a multi-week platform. Brands can ship LAN (+ BT) today
with local API keys from the phone; cloud Connect can arrive later without
rewriting the message API.

Other caution points:

1. **Wadhu signup OTP via SMS gateway** — keep an internal meter/path so platform
   OTP never depends on a customer subscription.
2. **Firebase as first IdP** — adapter only; Wadhu owns business IDs in Postgres.
3. **sms-relay SQLite + admin token** — lab/self-host slice, not the final SaaS.
4. **Offline-first day-1** — product copy must not force cloud; paste Listen URL
   + key / BT must remain first-class (ChatGPT §8 already allows this).
5. **Cloud provider** — defer bake-off; keep the API stable first.

------------------------------------------------------------------------

## Explicit non-goals of this note

- Implementing Firebase / OAuth Connect / Stripe / org admin UI
- Replacing self-hosted relay with GCP in this pass
- Changing the free LAN/BT policy
