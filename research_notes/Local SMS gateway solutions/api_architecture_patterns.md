# Local SMS gateway API architecture patterns

Dense research notes for a reusable local-first SMS gateway (no vendor cloud required): transport/auth shapes, outbox semantics, multi-backend, LAN discovery/pairing, multi-client drain, status/DLR model, product-grade DX, and security.

## What API shapes do existing local gateways expose (REST send/status, webhooks on inbound, WebSocket events, MQTT)?

### Takeaway
Local gateways converge on REST for send + status poll, webhooks for inbound/outbound events, and optionally WebSocket or MQTT for push; cloud APIs (Twilio/MessageBird) are useful DX templates for status callbacks and OpenAPI, not as a cloud dependency.

### Cited Findings
- SMS Gateway for Android (SMSGate) exposes Local Server mode on the device (`http://<device_local_ip>:8080/message`) with Basic auth, same request shape as cloud/private, plus webhook registration for events such as `sms:received`, `sms:sent`, `sms:delivered`, `sms:failed`, `sms:cancelled`, MMS events, `app:started`, and `system:ping` — [Local Server](https://docs.sms-gate.app/getting-started/local-server/); [Webhooks](https://docs.sms-gate.app/features/webhooks/); [GitHub README](https://github.com/capcom6/android-sms-gateway).
- SMSGate’s third-party/cloud-style API groups include Messages, Devices, Webhooks, Inbox, Settings, Health, and Auth under `/api/3rdparty/v1/…`, with JWT scopes (`messages:send`, `webhooks:write`, etc.), Prometheus `/metrics`, and `/.well-known/api-catalog` — [android-sms-gateway/server](https://github.com/android-sms-gateway/server/).
- SMSGate documents an OpenAPI specification for the integration API, Basic (legacy) vs JWT (recommended) auth, and notes Local Server is LAN-only while Cloud is internet-reachable — [API integration](https://docs.sms-gate.app/integration/api/).
- SMSGate also bridges industry protocols: SMPP v3.4, Email-to-SMS, Telegram bridge, and a Twilio fallback service — [SMSGate overview](https://docs.sms-gate.app/).
- Send body supports optional client `id`, `phoneNumbers`, `simNumber`, `ttl`/`validUntil`, `scheduleAt`, `withDeliveryReport`, `priority`, and mutually exclusive `textMessage` / `dataMessage` / `mmsMessage` — [Sending Messages](https://docs.sms-gate.app/features/sending-messages/).
- Self-hosted modem gateway `morgverd/sms-server` advertises HTTP OpenAPI, HTTP webhooks, WebSocket live events (`ws://…/ws` with `?events=` filter for `incoming`, `outgoing`, `delivery`, `modem_status_update`, `gnss_position_report`), and a Rust client library — [sms-server](https://github.com/morgverd/sms-server); [websocket.md](https://github.com/morgverd/sms-server/blob/b638b71e921254939b97f0be8342c7242dd1d0cb/docs/websocket.md).
- Android MQTT gateways publish JSON on topics for `received` / `sent` / `delivered` and consume send commands from a broker (often a public broker in hobby designs) — [ibnux/Android-SMS-Gateway-MQTT](https://github.com/ibnux/Android-SMS-Gateway-MQTT/); [deunix-educ/TermuxSmsGateway](https://github.com/deunix-educ/TermuxSmsGateway).
- Twilio Messages API: `POST …/Messages.json` creates a Message resource; statuses include `queued`, `sending`, `sent`, `failed`, `delivered`, `undelivered`, plus inbound `receiving`/`received`; status callbacks are form-urlencoded POSTs with `MessageStatus` and `ErrorCode`, and for SMS/MMS DLR passthrough `RawDlrDoneDate` — [Messages resource](https://www.twilio.com/docs/messaging/api/message-resource); [Track outbound status](https://www.twilio.com/docs/messaging/guides/track-outbound-message-status).
- MessageBird SMS API: `POST https://rest.messagebird.com/messages`; recipient statuses `scheduled`, `sent`, `buffered`, `delivered`, `expired`, `delivery_failed`; status reports are HTTP GETs to `reportUrl` only when both `reference` and a report URL are set; 429 means request not processed and is retryable — [MessageBird SMS Messaging](https://developers.messagebird.com/api/sms-messaging/).

### Inferences
- For a local product serving sibling apps + third parties, the implementable default is: **REST accept (202 + message id)**, **GET status**, **webhook push** (works across process boundaries), plus **optional WebSocket** for same-LAN UIs; reserve MQTT for sites that already run a broker, and prefer a **local** broker if used (hobby MQTT designs often imply cloud brokers, which violates the no-vendor-cloud constraint unless self-hosted).
- Match Twilio/MessageBird DX where cheap: stable message IDs, explicit status enum, StatusCallback/reportUrl-style registration, OpenAPI, and clear 429 semantics — without copying form-encoding or account SIDs.
- Prefer JSON event envelopes with `event` + `payload` (SMSGate style) or typed `event_type` (sms-server style) over channel-specific form bodies, so one client library can consume webhook and WebSocket identically.
- Expose a thin SMPP facade only if aggregator/legacy PBX clients matter; first-party shop apps should use REST/WebSocket.

### Gaps
- No strong primary source documenting a single widely adopted **local** SMS gateway that exposes all four of REST + webhooks + WebSocket + MQTT with mutual TLS and capability discovery; patterns must be composed from SMSGate, sms-server, and cloud DX.
- GitHub fetch for `morgverd/sms-server` root page timed out once; WebSocket details rely on the documented `docs/websocket.md` blob and search snippets.
- Exact SMSGate Local Server path parity with `/3rdparty/v1/messages` vs `/message` differs by mode; integrators must treat local path aliases carefully.

## What queue/outbox patterns work for unreliable radios (at-least-once, idempotency keys, retry/backoff, rate limits per SIM)?

### Takeaway
Treat radio send as an at-least-once side effect: durable outbox on the gateway, client idempotency keys, exponential webhook/API retries, per-SIM (per-sender) rate limits and validity/TTL — never claim exactly-once across the air interface.

### Cited Findings
- Client/local-first outbox pattern: persist the write intent first, drain on connect/resume, delete on success, keep on transient failure, drop on permanent failure; at-least-once requires server `Idempotency-Key` (or equivalent) or duplicates become real side effects; stuck records need attempt/age safety valves — [Flutter Outbox Pattern](https://dev.to/guimg/flutter-outbox-pattern-16m3); [Voltro useOutbox](https://docs.voltro.dev/docs/ui/client-utilities/use-outbox).
- Classic transactional outbox: business row + outbox row in one local transaction; relay may publish duplicates after crash; consumers need idempotency; outbox is not exactly-once end-to-end — [Arcflow Outbox](https://www.arcflow.me/patterns/outbox-pattern); [Agily outbox/inbox](https://agily.app/kafka/processing-patterns/outbox-and-inbox/); [Decodable Revisiting the Outbox Pattern](https://www.decodable.co/blog/revisiting-the-outbox-pattern).
- Stripe API: clients send `Idempotency-Key` (suggest V4 UUID); server returns the first saved response for retries with the same key; do not reuse a key with different parameters; keys are for safely retrying mutating requests after connection errors — [Stripe idempotent requests](https://docs.stripe.com/api/idempotent_requests); [Stripe idempotency blog](https://stripe.com/blog/idempotency).
- SMSGate queue: API enqueue → device local queue `Pending` → apply limits/delays (skipped for priority ≥100) → Android SMS API → `Processed`; optional client `id`, `ttl`/`validUntil`, priority −128..127 — [Sending Messages](https://docs.sms-gate.app/features/sending-messages/); [client-go constants](https://pkg.go.dev/github.com/android-sms-gateway/client-go/smsgateway).
- SMSGate webhook delivery: require 2xx within 30s; exponential backoff starting 10s then doubling; default 14 retries (~2 days), configurable — [Webhooks](https://docs.sms-gate.app/features/webhooks/).
- SMSGate deliberately will not auto-resend from stuck `Processed` because the SMS may already have been sent — [Status Tracking](https://docs.sms-gate.app/features/status-tracking/).
- Gammu SMSD: multi-client enqueue into shared SQL `outbox`; `Retries`, `Priority`, `SendingTimeOut`, `DeliveryReport`, `CreatorID`; status enums include `SendingOK`, `SendingError`, `DeliveryOK`, `DeliveryFailed`, `DeliveryPending`, etc. — [SMSD tables](https://docs.gammu.org/smsd/tables.html); [SMSD config](https://docs.gammu.org/smsd/config.html).
- Twilio: per-sender (and increasingly account/campaign) MPS limits; queues hold up to ~10 hours of segments then fail (30001 queue overflow); Validity Period can shorten expiry; HTTP 429/20429 means request not processed and is safe to retry with backoff; concurrent REST calls must stay under account concurrency — [Queueing and latency](https://www.twilio.com/docs/messaging/guides/scaling-queueing-latency); [Rate limits FAQ](https://help.twilio.com/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-); [20429](https://www.twilio.com/docs/api/errors/20429); [30001](https://www.twilio.com/docs/api/errors/30001).
- MessageBird: `validity` seconds after which undelivered messages are discarded; 429 = not processed, retry later — [MessageBird SMS Messaging](https://developers.messagebird.com/api/sms-messaging/).
- MQTT Android gateway documents retry of failed sends 3 times — [ibnux/Android-SMS-Gateway-MQTT](https://github.com/ibnux/Android-SMS-Gateway-MQTT/).

### Inferences
- **Gateway-owned outbox** (SQLite/Postgres on the host that owns the radio) is the right center of truth: clients POST intents; a single drain worker claims rows (lease/`SendingTimeOut`-style) and talks to Android/modem backends.
- **Two idempotency layers**: (1) HTTP accept idempotency key scoped to client/API key → same message id, no second enqueue; (2) backend submit idempotency / “do not auto-retry after ambiguous submit” (SMSGate `Processed` lesson).
- Implementable SIM rate limiter: token bucket or leaky bucket **per backend + SIM slot**, with message `priority` bypass for OTP-like traffic (SMSGate’s ≥100 pattern), and `expires_at` so drained rows past TTL become `expired` instead of sent.
- Client retry policy: retry only on transport failure / 429 / 5xx before accept ack; after 2xx/202 with id, poll/subscribe status — do not POST a new body without the same idempotency key.
- Webhook consumers must be idempotent on `messageId`+`event` (at-least-once delivery with backoff).

### Gaps
- Public SMSGate docs describe priority bypass of limits/delays but do not publish a precise default MPS/delay table per carrier in the pages reviewed.
- Little primary literature specifically on **SIM thermal/carrier throttling** as API-visible capability; product will need empirical defaults (e.g. 1 msg/sec long-code style) tunable per backend.

## How should desktop↔Android pairing work without cloud (mDNS/LAN token, QR, USB, Bluetooth)? Compare tradeoffs.

### Takeaway
Best local UX is DNS-SD/mDNS discovery + short-lived single-use QR/token exchange with TLS fingerprint pinning; USB is strongest for initial trust on a desk; Bluetooth is optional proximity glue — none should ship long-lived secrets in QR codes.

### Cited Findings
- RFC 6763 DNS-SD: discover via PTR `<Service>.<Domain>`, then SRV (host/port) + mandatory TXT key=value metadata; unknown TXT keys should be ignored; TXT is for small capability/metadata, not a replacement for connecting — [RFC 6763](https://www.rfc-editor.org/rfc/rfc6763.html); [TXT record format notes](https://www.zeroconf.org/rendezvous/txtrecords.html).
- IETF dnssd pairing draft: QR can carry name/IP/port for discovery and/or authenticate a shared secret; warns adversaries with cameras can read QR; embedding long-lived passwords/PINs in QR is hazardous — [draft-ietf-dnssd-pairing-info-02](https://www.ietf.org/archive/id/draft-ietf-dnssd-pairing-info-02.txt).
- Autobutler QR+mDNS pattern: advertise `_autobutler._tcp` / `autobutler.local` with TXT `port`, `product`; authenticated session generates pairing QR; public `POST /pair` exchanges single-use token (10 min TTL, SHA-256 hashed at rest, rate-limited) for a session — [PR #1423](https://github.com/autobutler-org/autobutler/pull/1423); [issue #1403](https://github.com/autobutler-org/quark/issues/1403).
- Cognia companion pairing: LAN scanner (mDNS + IP probe), QR payload with host/tenant/expiry/version/TLS fingerprint; pin SPKI hash; register device keys; private keys stay in OS secure store — [Cognia Discovery & Pairing](https://docs.cognia.cn/en/docs/ui/mobile-architecture/discovery-pairing).
- Aethon mobile: desktop QR + 8-digit code; phone browses `_aethon._tcp`, tries hosts in order, pins fingerprint from TXT; 120s pairing window — [aethon issue #460](https://github.com/utensils/aethon/issues/460).
- SMSGate Local Server shows credentials on-device and expects clients to hit the LAN IP with Basic auth; no mDNS/QR pairing flow in the local-server getting-started doc — [Local Server](https://docs.sms-gate.app/getting-started/local-server/).

### Inferences
- **Recommended pairing matrix for this product**:

  | Mode | Trust | UX | Offline | Risk |
  |------|-------|----|---------|------|
  | mDNS browse + QR token | High if TLS pin + single-use token | Best LAN UX | Yes | Shoulder-surf QR; multicast blocked on some guest Wi‑Fi |
  | Manual LAN IP + paste token | Medium | Poor | Yes | Typos; MITM without pin |
  | USB (accessory/ADB-like channel) | Highest physical presence | Great for desktop radio phone | Yes | Requires cable; OS permission prompts |
  | Bluetooth LE | Medium proximity | Handy when Wi‑Fi isolated | Yes | Pairing complexity; still need app-layer auth |

- Implementable flow: Desktop (or phone-as-gateway) advertises `_wadhu-sms._tcp` with TXT `ver`, `proto`, `fp=<tls-spki>`, `caps=sim,modem`; peer scans QR containing `https://<host>:<port>/pair?t=<token>` or a compact CBOR/base64 payload; `POST /v1/pair` returns client id + API key / mTLS cert; revoke via UI.
- Do **not** put the long-lived API key in the QR; put a **pairing grant** that is single-use and short TTL (Autobutler/IETF guidance).
- When Android is the radio and desktop is the client: phone runs gateway + mDNS; desktop discovers. When desktop owns a USB modem and phone is just a client: reverse the advertiser. Same pair protocol both ways.

### Gaps
- No widely standardized SMS-gateway-specific DNS-SD service type; product must define its own `_…._tcp` profile.
- Limited high-quality primary writeups comparing **USB accessory** vs **Bluetooth** specifically for SMS gateway pairing (most IoT UX writeups are QR+mDNS). USB/Bluetooth tradeoffs above are inferred from general local-pairing practice plus the cited QR/mDNS sources.
- Guest Wi‑Fi / AP isolation and Android mDNS background limits are environment-specific; need product QA matrix, not covered quantitatively here.

## How do multi-client scenarios work (several shop apps sharing one gateway) without double-sends?

### Takeaway
One gateway process owns the radio and the outbox; many clients only enqueue with scoped API keys and idempotency keys — never give multiple apps direct concurrent modem/SMS API access.

### Cited Findings
- Gammu: multiple SMSD instances can share one SQL DB with distinct `PhoneID`; each sends outbox rows whose `SenderID` matches or is empty; empty `SenderID` means first daemon wins; **you cannot use the same phone from more programs at the same time** — [SMSD config Multiple modems](https://docs.gammu.org/smsd/config.html); [SMSD known limitations](https://docs.gammu.org/smsd/smsd.html); [tables](https://docs.gammu.org/smsd/tables.html).
- Gammu outbox includes `CreatorID` to identify the injecting program — [SMSD tables](https://docs.gammu.org/smsd/tables.html).
- SMSGate cloud/private: multiple devices on one account; messages distributed across connected devices; optional `deviceId` / `simNumber` targeting; optional client message `id` — [SMSGate features](https://docs.sms-gate.app/); [Sending Messages](https://docs.sms-gate.app/features/sending-messages/).
- Stripe-style idempotency prevents duplicate side effects when clients retry — [Stripe idempotent requests](https://docs.stripe.com/api/idempotent_requests).
- Outbox literature: multiple publishers claiming the same row without coordination is a listed failure mode — [Arcflow Outbox](https://www.arcflow.me/patterns/outbox-pattern).
- Twilio Messaging Services abstract multiple senders behind one SID and queue/load-balance — [Queueing and latency](https://www.twilio.com/docs/messaging/guides/scaling-queueing-latency).

### Inferences
- **Architecture**: `N clients → 1 gateway API → 1 outbox → M backends (SIM slots / USB modems)`. Claim rows with `UPDATE … SET lease_owner, lease_until WHERE status='queued' AND (lease_until IS NULL OR lease_until < now())` (or equivalent) so only one drain worker submits.
- **Per-client isolation**: API keys with scopes (`messages:send`, `messages:read`, `inbox:read`, `admin`); store `client_id` / `CreatorID` on each outbox row for audit and fair queuing; optional per-client quotas.
- **Idempotency key scope**: unique per `(api_key_id, idempotency_key)` so two shops can independently use `order-123` without colliding, while one shop’s retries collapse.
- **Routing**: clients may set `backend_id` / `sim` / `from`; if omitted, gateway policy picks (round-robin, least-queue, sticky). Mirror Gammu’s PhoneID/SenderID for multi-modem.
- **Double-send prevention checklist**: single radio owner; idempotent accept; no client-side “send SMS directly”; ambiguous submit → mark indeterminate, do not auto-retry (SMSGate `Processed` rule); cancel only while still queued.

### Gaps
- SMSGate Local Server docs reviewed do not detail multi-tenant API keys or per-client quotas on-device; cloud JWT scopes are the closer model to copy.
- No cited standard for “fair share” scheduling across clients on one SIM; product must choose (FIFO vs weighted fair queue).

## What status/lifecycle model is useful (queued, submitting, submitted, delivered, failed, expired) and how do DLRs map?

### Takeaway
Use a small public state machine that separates **accept/queue**, **radio submit**, **SMSC accept**, **handset DLR**, and **expiry/cancel** — map Android/SMPP/modem DLRs into those states, and document indeterminate states explicitly.

### Cited Findings
- SMSGate states: `Pending` → `Processed` → `Sent` → `Delivered`, plus `Failed`, `Cancelling`/`Cancelled`; `Processed` = handed to Android SMS API; `Sent` = SMSC accepted; `Delivered` needs `withDeliveryReport: true`; stuck `Processed` is indeterminate (no auto-resend); multipart/multi-recipient aggregation rules documented; DLR permanent vs temporary errors — [Status Tracking](https://docs.sms-gate.app/features/status-tracking/).
- SMSGate SMPP mapping: pending→Scheduled(0), processed/sent→Enroute(1), delivered→Delivered(2), failed→Undeliverable(5) — [SMPP](https://docs.sms-gate.app/integration/smpp/).
- Twilio outbound callbacks: after queue, `sent` or `failed`; then `delivered` or `undelivered`; optional `read` on some channels; `RawDlrDoneDate` passthrough from carrier DLR Done Date (minute precision, no reliable TZ) — [Outbound status callbacks](https://www.twilio.com/docs/messaging/guides/outbound-message-status-in-status-callbacks); [RawDlrDoneDate changelog](https://www.twilio.com/en-us/changelog/addition-of-rawdlrdonedate-to-delivered-and-undelivered-status-webhooks).
- MessageBird recipient statuses: `scheduled`, `sent`, `buffered`, `delivered`, `expired`, `delivery_failed`, plus rich `statusReason` strings; validity causes discard/expiry — [MessageBird SMS Messaging](https://developers.messagebird.com/api/sms-messaging/).
- Gammu sentitem-ish statuses: `SendingOK`, `SendingOKNoReport`, `SendingError`, `DeliveryOK`, `DeliveryFailed`, `DeliveryPending`, `DeliveryUnknown`, `Error` — [SMSD tables](https://docs.gammu.org/smsd/tables.html).

### Inferences
- **Recommended public enum for a local product** (Twilio-ish names, SMSGate rigor):

  | State | Meaning | Maps from |
  |-------|---------|-----------|
  | `queued` | Accepted into gateway outbox | SMSGate Pending, Twilio queued |
  | `submitting` | Drain worker claimed; calling Android/modem | SMSGate Processed / Twilio sending |
  | `submitted` | SMSC/modem accepted PDU | SMSGate Sent, Twilio sent, MB sent/buffered |
  | `delivered` | Positive DLR / handset ack | SMSGate Delivered, Twilio delivered, MB delivered |
  | `failed` | Permanent send or DLR failure | Failed / undelivered / delivery_failed |
  | `expired` | Past `validUntil`/TTL before submit or carrier expiry | MB expired, Twilio validity |
  | `cancelled` | Cancelled while still queued | SMSGate Cancelled |
  | `indeterminate` | Ambiguous after submit attempt (optional explicit) | SMSGate stuck Processed |

- Always store **per-recipient** and **per-segment** substates (SMSGate recipients[]; Twilio/MessageBird multi-recipient models).
- Emit webhook/WS events on each transition: `message.queued`, `message.submitted`, `message.delivered`, `message.failed`, `message.expired`, plus `message.inbound`.
- Pass through raw DLR codes/timestamps as opaque metadata (`dlr_code`, `dlr_done_at`) like Twilio’s `RawDlrDoneDate` — carriers are inconsistent.
- **Do not** collapse `submitted` and `delivered`; many local radios never get DLRs (`SendingOKNoReport` / remain `Sent`).

### Gaps
- Android Delivery Intent / SMSC error code matrices are carrier- and OEM-specific; GSM 03.40 is referenced by SMSGate but not expanded in the reviewed page.
- No universal guarantee that temporary DLR errors ever resolve to a final state on Android (SMSGate documents messages remaining `Sent`).

## What makes a gateway “library/product grade” for third parties (OpenAPI, client SDKs, capability discovery, sandbox/dry-run, audit log)?

### Takeaway
Third parties adopt gateways that look like cloud SMS APIs on the LAN: versioned OpenAPI, official SDKs, discoverable capabilities/health, dry-run/sandbox, scoped tokens, and auditable message history — SMSGate and sms-server already point in that direction.

### Cited Findings
- SMSGate publishes OpenAPI, CLI (`smsgate`), Go client package with typed states/priorities, JWT scopes, health probes (`/health/live|ready|startup`), Prometheus metrics, API catalog at `/.well-known/api-catalog`, webhook HMAC-style signing headers (`X-Timestamp`, `X-Signature`) described in webhook docs — [API integration](https://docs.sms-gate.app/integration/api/); [server README](https://github.com/android-sms-gateway/server/); [Webhooks](https://docs.sms-gate.app/features/webhooks/); [client-go](https://pkg.go.dev/github.com/android-sms-gateway/client-go/smsgateway); [CLI](https://docs.sms-gate.app/integration/cli/).
- sms-server positions OpenAPI UI, Rust client crate, webhook + WebSocket, delivery tracking, modem status events — [sms-server](https://github.com/morgverd/sms-server).
- DNS-SD TXT is explicitly intended to advertise small capability metadata before connecting — [RFC 6763](https://www.rfc-editor.org/rfc/rfc6763.html).
- MessageBird/Twilio provide stable send + get + status webhook contracts and rich status reason vocabularies that integrators already understand — [MessageBird](https://developers.messagebird.com/api/sms-messaging/); [Twilio Messages](https://www.twilio.com/docs/messaging/api/message-resource).
- Stripe documents sandboxes for safe integration testing of idempotent APIs — [API v2 overview](https://docs.stripe.com/api/v2-overview).

### Inferences
- **Minimum product-grade surface**:
  1. OpenAPI 3 + generated SDKs (at least Rust + TypeScript for Gotigin/web).
  2. `GET /v1/capabilities` (and mDNS TXT mirrors): backends present, SIM MSISDNs if known, max segment rate, DLR support, multipart, Unicode, dry-run support, auth modes.
  3. `GET /v1/health` ready vs live (radio up, queue depth, backend errors).
  4. `dry_run: true` or `X-Dry-Run: 1` that allocates message ids and walks state machine without touching radio.
  5. Append-only **audit log**: who enqueued, idempotency key, backend chosen, state transitions, webhook delivery attempts (hash/redact bodies by policy).
  6. Webhook signing + replay window; WebSocket auth same as HTTP.
  7. Semantic versioning of event schemas; unknown fields ignored forward-compat (Twilio/MessageBird warn attributes may be added).
- Easier/safer than incumbents: LAN-only default bind, pairing with pinned TLS, scoped keys, explicit indeterminate state, dry-run, no mandatory cloud account, copy-paste OpenAPI, and a single outbox so integrators cannot double-drive the modem.

### Gaps
- Reviewed sources do not show a mature **sandbox/dry-run** mode on SMSGate Local Server; this is a differentiation opportunity rather than an incumbent feature to copy verbatim.
- Formal “API catalog” / Well-Known usage beyond SMSGate’s mention was not deeply audited.

## Security: local-only bind, mutual auth, least privilege SMS permission UX, PII retention?

### Takeaway
Default bind to loopback or LAN interface with auth required; prefer scoped tokens or mTLS after pairing; on Android, default-SMS-handler policy dominates permission UX; minimize and expire stored message PII.

### Cited Findings
- SMSGate Local Server: LAN accessibility; Basic auth only in local mode; docs note public IP may be shown but CGNAT/firewall often block; webhook HTTPS required except `http://127.0.0.1` — [Local Server](https://docs.sms-gate.app/getting-started/local-server/); [Webhooks](https://docs.sms-gate.app/features/webhooks/).
- SMSGate recommends JWT with scopes over Basic for new integrations; E2E encryption of content/recipients with device public keys and out-of-band fingerprint pinning for cloud/private paths — [API integration](https://docs.sms-gate.app/integration/api/).
- Pairing patterns hash tokens at rest, single-use + TTL, rate-limit pair endpoint, require authenticated session to mint QR — [Autobutler PR](https://github.com/autobutler-org/autobutler/pull/1423); Cognia pins TLS SPKI — [Cognia](https://docs.cognia.cn/en/docs/ui/mobile-architecture/discovery-pairing/).
- Google Play: SMS/Call Log permissions generally require being the default SMS/Phone/Assistant handler before requesting; must stop using when no longer default; core functionality, privacy policy, prominent disclosure/consent required — [Default handlers](https://developer.android.com/guide/topics/permissions/default-handlers); [Play SMS/Call Log policy](https://support.google.com/googleplay/android-developer/answer/10208820); [Sensitive permissions](https://support.google.com/googleplay/android-developer/answer/9888170); [User Data](https://support.google.com/googleplay/android-developer/answer/10144311).
- Android permissions best practices reiterate default-handler prompt before SMS runtime permissions — [Usage notes](https://developer.android.com/training/permissions/usage-notes).
- IETF pairing draft: QR-observable secrets are a threat; prefer designs that do not embed durable passwords in QR — [draft-ietf-dnssd-pairing-info-02](https://www.ietf.org/archive/id/draft-ietf-dnssd-pairing-info-02.txt).
- MessageBird/Twilio transmit full message bodies to customer webhooks — integrators inherit PII handling obligations (status/report payloads include recipient and content-related fields) — [MessageBird status reports](https://developers.messagebird.com/api/sms-messaging/); [Twilio status callbacks](https://www.twilio.com/docs/messaging/guides/track-outbound-message-status).

### Inferences
- **Bind policy**: default `127.0.0.1` for desktop-owned modem; opt-in `0.0.0.0` / LAN interface only after pairing enabled; never expose without auth; prefer TLS even on LAN (self-signed + pin via QR `fp=`).
- **Auth ladder**: pairing grant → long-lived API key **or** client cert (mTLS); scopes least privilege (`send` without `inbox:read` for POS printers that only notify).
- **Android UX**: sideload/enterprise may avoid Play default-handler rules, but Play distribution requires default SMS handler + disclosure; product should implement a real SMS UI if it needs `RECEIVE_SMS`/`SEND_SMS` on Play. Document sideload vs Play paths.
- **PII retention**: configurable retention (e.g. 0/24h/7d); store hashes of bodies for idempotency/audit when possible; redact webhook logs; encrypt DB at rest on shared machines; separate inbox retention from outbox status retention.
- **Webhook security**: HMAC over raw body + timestamp; reject skew; allowlist localhost/LAN URLs in local mode; never require vendor cloud for delivery.

### Gaps
- No single cited “local SMS gateway GDPR retention default” standard; retention recommendations are inferred from general sensitive-data policy and messaging PII practice.
- Mutual TLS on Android Local Server is not documented in SMSGate local mode (Basic only); mTLS would be a product differentiator needing custom design.
- Bluetooth/USB channel security specifics for this domain were not covered by SMS-specific primary sources in this pass.
