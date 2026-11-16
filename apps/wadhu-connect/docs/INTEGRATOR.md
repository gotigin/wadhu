# Integrating Wadhu Connect in a brand app (e.g. Kumkaj)

Wadhu Connect lets **your customer’s** WhatsApp Business Account send and
receive Cloud API messages from **your** product (Khaynjo, Moklanjo, …).
You do not take over their number; they grant your Meta app access via
Embedded Signup / Login for Business.

Related doctrine:

- [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](../../../docs/IDENTITY_AND_MONETIZATION_ALIGNMENT.md) — wadhu.com accounts are for **cloud** features; LAN/BT SMS stays free.
- [`LOCAL_SMS.md`](../../../docs/LOCAL_SMS.md) — SMS is a separate channel; Connect is WhatsApp Cloud API.
- Brand site: https://wadhu.com · API host: https://connect.wadhu.com

## Product split

| Layer | Who | Pays |
| --- | --- | --- |
| Brand app UI (orders, KOT, labels) | Kumkaj / your brand | Brand pricing |
| Wadhu Connect (Cloud API pipe) | Wadhu | Cloud / platform (when billed) |
| WABA + phone number | The shop / agency | Meta / carrier as usual |

## Onboarding a shop (Embedded Signup)

1. In your app, open a WebView / browser to  
   `https://connect.wadhu.com/` (or embed the same FB SDK flow against Connect’s `/api/config`).
2. User clicks **Launch Embedded Signup** → Meta Login for Business.
3. They pick their existing WhatsApp Business app number (coexistence) when offered.
4. Connect exchanges the code, stores the business token, subscribes webhooks, registers the phone.
5. Your backend (or Connect, while early) can now send as that WABA.

Until wadhu.com org OAuth ships, treat Connect as **operator / Tech Provider**
hosted by Gotigin; brand apps call Connect’s HTTP API with a future org token.
Do **not** put `META_APP_SECRET` in any brand APK.

## Send a message (HTTP)

After a WABA is connected on the Connect instance:

```bash
curl -s -X POST https://connect.wadhu.com/v1/messages \
  -H 'Content-Type: application/json' \
  -d '{"to":"9198XXXXXXXX","text":"Your order is ready"}'
```

Today this uses the **single connection** stored on that Connect server (dev /
own-business). Multi-tenant routing (per org WABA) lands with wadhu.com
identity — see identity doc sequencing: LAN/BT first, then IdP → Connect UX.

For production templates (outside the 24h session window), send template
payloads via Graph the same way Connect’s `send_text` does — extend
`/v1/messages` or call Graph with the stored business token from your own
secure backend that obtained access through Connect.

## Receive messages

Subscribe Meta webhooks to `https://connect.wadhu.com/webhook` with your
verify token. Connect verifies `X-Hub-Signature-256` and appends events under
its data dir. Brand apps should poll or (later) receive a push/webhook relay
from Connect — do not point Meta webhooks at every brand server.

## Capabilities probe

```bash
curl -s https://connect.wadhu.com/v1/capabilities
curl -s https://connect.wadhu.com/v1/health
```

## Local testing

```bash
cargo wadhu-connect-dev
# PUBLIC_ORIGIN=https://<tunnel> cargo wadhu-connect-dev
open http://127.0.0.1:3847/
```

Use Meta test numbers / allowlisted recipients while the app is in Development
mode. Advanced access (App Review) is required before onboarding arbitrary
customer WABAs.

## Privacy / store listings

Use these URLs for Play / Microsoft Store / Meta for Wadhu products:

| | |
| --- | --- |
| Privacy | https://wadhu.com/privacy |
| Terms | https://wadhu.com/terms |
| Data deletion | https://wadhu.com/data-deletion |
| Support | https://wadhu.com/support |

Canonical Markdown: [`docs/legal/`](../../../docs/legal/README.md) (Connect
serves the same files). Brand apps (Khaynjo, …) keep **their** product privacy
URLs for their own listings; mention WhatsApp only where the brand actually
uses Connect.

## Independence

Brand apps **must not** depend on the `wadhu-connect` crate as a path
dependency inside Kumkaj. Talk over HTTP to `connect.wadhu.com` (or a
self-hosted Connect). Wadhu vision §3: apps never depend on sibling apps.
