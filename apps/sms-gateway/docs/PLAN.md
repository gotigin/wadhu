# SMS Gateway — complete plan

> **Product brief:** [`SMS_GATEWAY.md`](SMS_GATEWAY.md)  
> **Wadhu-wide doctrine (provider vs client):** [`docs/LOCAL_SMS.md`](../../../docs/LOCAL_SMS.md)  
> **Research index:** [`RESEARCH.md`](RESEARCH.md) →
> [`reports/Local SMS gateway solutions.md`](../../../reports/Local SMS gateway solutions.md)

------------------------------------------------------------------------

## 1. Goal

Ship **one Wadhu app** that **is** the SIM execution device on **Android
OS** (phones, tablets, and other form factors with a usable SIM):

- Other apps (Gotigin brands, third parties) connect via **LAN HTTP**, **Bluetooth
  framing**, or (for internet) a self-hosted [`sms-relay`](../../sms-relay/README.md).
- Radio: **Android SIM** (primary product path). Desktop builds are UI/API
  dry-runs only for now; USB modem is explicitly deferred.
- Local LAN/BT send **never** depends on the relay.

```text
  Clients ──LAN/BT──► SMS Gateway ──SIM──► mobile network
  Clients ──HTTP──► sms-relay ──WSS──► SMS Gateway ──SIM──► …
```

## 2. Users

| User | Needs |
|---|---|
| **Operator** | Install this app on the phone (or PC with modem). See Radio / API / Queue health. Grant SMS permission. Share listen URL + API key with client apps. Send a test SMS. |
| **Client developer** | Stable local HTTP API: accept message, poll status, discover capabilities. No Gotigin account. |
| **Gotigin brand apps** | Same API (later a small Rust client crate). Their outboxes drain here. |

## 3. Platforms and radios

| Build | Radio now | Radio next |
|---|---|---|
| Android (`com.wadhu.smsgateway`) | **SIM via `SmsManager`** | USB/OTG modem if device supports host mode |
| Desktop (Linux / Windows) | API + outbox only (radio reports unavailable) | **USB modem** (ModemManager / AT, allowlist) |

Same app package, same API. `GET /v1/capabilities` tells clients which radio is live.

## 4. What exists today (first slice)

| Piece | Status |
|---|---|
| Outbox + status machine + drain loop | Done |
| Local HTTP API (`/v1/health`, `/capabilities`, `/messages`) | Done |
| API key auth (`X-Api-Key` / Bearer) | Done |
| Android SIM send (Java + JNI patches) | Done |
| Android manifest `SEND_SMS` + permission UI | Done |
| Operator UI (status chips, connection, compose, recent) | Done (redesigned) |
| Client crate [`crates/sms-client`](../../../crates/sms-client) | Done (native + wasm) |
| Desktop demo [`apps/sms-client-demo`](../../sms-client-demo) | Done (desktop only) |
| CORS on gateway / relay HTTP | Done |
| Desktop modem radio | Not yet |
| Persist outbox across restarts | Not yet |
| Delivery reports / inbound / STOP | Not yet |
| mDNS + QR pairing | Not yet |

### How to run (today)

```bash
cargo sms-gateway-dev                 # desktop UI + http://127.0.0.1:18765
cargo sms-gateway-android-dev         # phone gateway on LAN :18765
cargo sms-client-demo-dev             # desktop integrator UI (wadhu-sms-client)
cargo test -p wadhu-sms-gateway
cargo test -p wadhu-sms-client
```

Client (crate or curl):

```bash
curl -H "X-Api-Key: $TOKEN" http://127.0.0.1:18765/v1/health
curl -H "X-Api-Key: $TOKEN" -H 'Content-Type: application/json' \
  -d '{"to":"+91…","body":"hello","idempotency_key":"1"}' \
  http://127.0.0.1:18765/v1/messages
```

On Android: grant SMS permission in the app; clients use `http://<phone-ip>:18765` with the same key.

## 5. Architecture

### 5.1 Process layout

- **UI thread (Makepad):** operator shell; starts gateway; refreshes health on a timer.
- **HTTP thread:** accepts client requests; never blocks UI.
- **Drain thread:** takes `queued` → `submitting` → radio `send` → `submitted` / `failed`.

### 5.2 Modules (this app)

| Module | Responsibility |
|---|---|
| `gateway::message` | Id, status enum, timestamps |
| `gateway::outbox` | Accept (idempotent), list, next queued, set status |
| `gateway::radio` | `SmsRadio` trait; `AndroidSimRadio`; `DesktopNoRadio` |
| `gateway::http` | Minimal HTTP/1.1 JSON API |
| `gateway::service` | Wire outbox + radio + HTTP + drain |
| `permissions` | Android `SEND_SMS` check/request |
| `main` | Operator UI |

Reusable client types later move to `crates/<name>/` (e.g. `wadhu-sms-client`) so other brands depend on a crate, not this app.

### 5.3 Status machine

`queued` → `submitting` → `submitted` → (`delivered` later)  
Failures: `failed` · timeouts: `expired` · user cancel: `cancelled` · unknown after handoff: `indeterminate`

**Rule:** after radio handoff, do not auto-resend the same idempotency key on another radio.

### 5.4 HTTP surface (v1)

| Method | Path | Auth | Body / notes |
|---|---|---|---|
| GET | `/v1/health` | key | Radio + queue counts |
| GET | `/v1/capabilities` | key | `kind`, `available`, multipart, etc. |
| POST | `/v1/messages` | key | `{to, body, idempotency_key?}` → 202 + message |
| GET | `/v1/messages` | key | Recent list |
| GET | `/v1/messages/{id}` | key | One message |

Bind: Android `0.0.0.0:18765` · Desktop `127.0.0.1:18765` (LAN bind optional later).

## 6. Phased roadmap

### Phase A — Android SIM gateway (current)

**Done when:** a client on the same Wi‑Fi can send through an Android install of SMS Gateway and see status move to `submitted` or a clear `failed` reason; operator UI shows Radio / API / Queue.

**Includes:** outbox, HTTP, SIM radio, permission UX, tests, docs.

### Phase B — Desktop USB modem

**Done when:** the same client binary works against a desktop install with an allowlisted stick; `capabilities.kind = usb_modem`.

**Work:**

1. Device allowlist (start with 1–2 known modem-mode sticks).
2. Worker via ModemManager (`mmcli`) and/or thin AT; document udev rules.
3. Wire as `SmsRadio` implementation; desktop default becomes modem when present.
4. Operator UI: show modem name / signal / error in Radio chip detail.
5. Soak test overnight.

### Phase C — Operator & integrator polish

**Done when:** a non-developer can pair a client without reading curl docs.

**Work:**

1. Persist outbox + API key (app data dir).
2. mDNS `_wadhu-sms._tcp` + QR that carries single-use grant (not long-lived secret in QR forever).
3. Copy-to-clipboard for URL/token; clearer “phone went to sleep” guidance.
4. Rate caps, quiet hours, basic audit log (gateway-enforced).
5. Optional inbound SMS + STOP keyword → suppress further sends to that number.

### Phase D — Extra radios & packaging

1. Android USB host modem (if hardware allows) behind same trait.
2. Allowlisted appliance HTTP adapters (Dinstar / SMSEagle class).
3. Optional cloud-SMS radio — never required for local drain.
4. Shipping packages (APK / desktop installers) when ready; MSI upgrade codes already scaffolded.

### Phase E — Client crate

1. ~~Brand-neutral HTTP client + types~~ → [`crates/sms-client`](../../../crates/sms-client) (native sync + wasm `*_async`).
2. ~~Desktop demo~~ → [`apps/sms-client-demo`](../../sms-client-demo) (`cargo sms-client-demo-dev`).
3. OpenAPI kept in sync with frozen v1 surface.
4. Other brands / third parties depend on the crate by path or git tag; mobile/browser *demos* later.

## 7. UI plan (operator)

| Section | Purpose |
|---|---|
| Header | Product name + one-line role (“other apps connect here”) |
| Status | Three chips: **Radio**, **API**, **Queue** — live refresh |
| For client apps | Listen URL + API key + endpoint hint |
| Send a test SMS | To + body + queue (proves the radio path) |
| Recent messages | Last statuses for operator confidence |
| This device | SMS permission (Android) + short radio hint |

Design tokens: [`docs/DESIGN.md`](../../../docs/DESIGN.md) (navy / indigo / cyan / lime on night ground).

## 8. Compliance (gateway responsibility)

- Automated business SMS is treated as A2P by carriers; handset SIMs can be throttled or blocked.
- India TCCCPR: unregistered commercial traffic is fragile — keep promotional defaults off until counsel/jurisdiction packs say otherwise.
- Gateway (not each client) should eventually enforce caps, quiet hours, consent refs, STOP.

See research report for sources. Not legal advice.

## 9. Non-goals

- Replacing WhatsApp Business API or RCS  
- Hosted marketing blast SaaS  
- Requiring Gotigin cloud to send  
- Supporting every USB stick without an allowlist  
- Putting books / KOT / AWB / brand CRM inside this app  
- Being a *client* of another SMS gateway product  

## 10. Success metrics

| Metric | Bar |
|---|---|
| Role | Clients only talk HTTP to this app; this app owns the radio |
| Android path | Real SIM send from LAN client in lab |
| Desktop path | Same client against USB modem (Phase B) |
| Ops | Radio/API/queue failures visible within ~2s in UI |
| Cloud | Zero Wadhu/Gotigin cloud required for local send |

## 11. Open decisions (owner)

| Topic | Proposal | Needs confirm |
|---|---|---|
| Android distribution | Private/MDM APK first; Play when default-SMS story is ready | Yes |
| First modem models | 1–2 Quectel/SIM7600 or known E3372-in-modem-mode | Yes |
| Default desktop bind | Loopback until “allow LAN” toggle | Yes |
| Client crate name | `sms-client` or `sms-outbox` | At Phase E |

## 12. Immediate next engineering steps

1. Run Android build on a physical device; confirm SIM send end-to-end.  
2. Persist outbox + API key.  
3. Start Phase B modem spike on one allowlisted stick.  
4. Freeze OpenAPI document from the live `/v1` routes before the client crate.
