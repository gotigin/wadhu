# Identity & monetization — alignment note (post-Bluetooth)

Short written reconciliation of the ChatGPT Wadhu identity/SMS platform brief
with what already exists. **Not** a build checklist for a full IdP.

**User / org / grouping definitions:** [`IDENTITY_MODEL.md`](IDENTITY_MODEL.md)
(read that first — do not treat every cloud feature as org-scoped).

Source brief (external): `CHATGPT-WADHU-IDENTITY-SMS-GATEWAY-PLAN.md`  
Doctrine: [`LOCAL_SMS.md`](LOCAL_SMS.md) · Vision: [`WADHU_VISION_AND_PRINCIPLES.md`](WADHU_VISION_AND_PRINCIPLES.md) §3

------------------------------------------------------------------------

## Locked product split

| Layer | Who pays | Needs wadhu.com account? |
|-------|----------|---------------------------|
| Phone SIM SMS (carrier) | SIM owner | No |
| LAN / Bluetooth to own phone | **Free forever** | No |
| Hosted relay + multi-device cloud | Wadhu subscription | Yes (**user**; **org** when multi-purchase or multi-member — see identity model) |
| Brand app (Moklanjo, Khaynjo, …) | That brand’s pricing | Brand account; Wadhu **Connect** only if they use cloud SMS and/or [Wadhu Connect](../apps/wadhu-connect/README.md) (WhatsApp Cloud API) |
| Wadhu Connect (WhatsApp Cloud API pipe) | Wadhu / platform (when billed) | Yes for cloud; login is **user**-based; isolate per **org** when the same person has more than one WABA/plan or shares with staff. Own-business / Tech Provider testing can run on connect.wadhu.com (dev/test mode — do not block on Meta Advanced Access) |

Registration on **wadhu.com** is for **cloud** features (creates a **user**).
Brand apps either (a) talk to the local gateway (LAN/BT + phone API key), or
(b) connect to Wadhu as that **user** for short-lived cloud credentials. No
permanent cloud secrets in APKs.

------------------------------------------------------------------------

## What already matches

| ChatGPT idea | Reality today |
|--------------|---------------|
| Independent SMS gateway | [`apps/wadhu-sms-gateway`](../apps/wadhu-sms-gateway) + [`crates/wadhu-sms-client`](../crates/wadhu-sms-client) |
| One job pipeline, multi-transport | Outbox + LAN HTTP + Classic RFCOMM + lab TCP framing |
| LAN/BT free; cloud optional | This doctrine + [`BLUETOOTH.md`](../apps/wadhu-sms-gateway/docs/BLUETOOTH.md) |
| No shared core across brand apps | Vision §3 |
| Relay for internet | [`apps/wadhu-sms-relay`](../apps/wadhu-sms-relay) (admin keys / pairing — **not** full SaaS billing) |
| Person registration (IdP UI) | Firebase + `/account` on wadhu.com — see [`FIREBASE_AUTH_SETUP.md`](FIREBASE_AUTH_SETUP.md); Wadhu `user_id` DB still to build |

------------------------------------------------------------------------

## Agree (direction)

- Do **not** put permanent cloud secrets in brand APKs.
- Do **not** make one Kumkaj app call another for SMS.
- Do **not** disable LAN/BT when a relay subscription expires.
- One Wadhu platform; registered client apps never depend on sibling apps.
- Identity provider ≠ business DB (Firebase UID must not be the billing FK).
- Cross-app sameness = **user**; **org** only for multi-purchase / multi-member /
  scoped billing — [`IDENTITY_MODEL.md`](IDENTITY_MODEL.md).
- Email verification mandatory for email signup; SMS OTP ≠ substitute for email.
- Monetize **hosted relay / platform only**.
- Do **not** serialize cloud work behind Meta App Review; use Development /
  test numbers meanwhile.

------------------------------------------------------------------------

## Timing / sequencing

```text
Done:     Classic RFCOMM + LAN HTTP + client framed lab; Firebase /account UI
Parallel: Keep SMS /v1 + OpenAPI + client docs stable for brand LAN/BT
Cloud:    Users (Postgres) → wire /account → Connect test-mode (user; org when needed)
          → site WhatsApp UX → payments → hosted relay entitlements
```

Do **not** block brand-app **local** SMS on Firebase/Stripe. Block local SMS
only on **stable `/v1` + client crate + working LAN/BT**.

Do **not** block **cloud** progress on Meta Advanced Access or Apple Sign-In.

Other caution points:

1. **Wadhu signup OTP via SMS gateway** — keep an internal meter/path so platform
   OTP never depends on a customer subscription.
2. **Firebase as first IdP** — adapter only; Wadhu owns `user_id` in Postgres.
3. **sms-relay SQLite + admin token** — lab/self-host slice, not the final SaaS.
4. **Offline-first day-1** — product copy must not force cloud; paste Listen URL
   + key / BT must remain first-class.
5. **Cloud provider** — defer bake-off; keep the SMS message API stable.

------------------------------------------------------------------------

## Explicit non-goals of this note

- Replacing self-hosted relay with a chosen cloud vendor in this pass
- Changing the free LAN/BT policy
- Treating in-product outlet **grouping** as Wadhu **orgs**
