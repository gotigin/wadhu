# Local SMS — Wadhu-wide doctrine

> **Gateway (SIM execution device):**
> [`apps/wadhu-sms-gateway/`](../apps/wadhu-sms-gateway/README.md)
> ([brief](../apps/wadhu-sms-gateway/docs/SMS_GATEWAY.md),
> [OpenAPI](../apps/wadhu-sms-gateway/docs/openapi-v1.yaml),
> [Bluetooth](../apps/wadhu-sms-gateway/docs/BLUETOOTH.md)).\
> **Relay (optional remote plane):**
> [`apps/wadhu-sms-relay/`](../apps/wadhu-sms-relay/README.md)
> ([brief](../apps/wadhu-sms-relay/docs/SMS_RELAY.md),
> [self-host](../apps/wadhu-sms-relay/docs/SELF_HOST.md),
> [OpenAPI](../apps/wadhu-sms-relay/docs/openapi-v1.yaml)).\
> **Client helpers:** [`crates/wadhu-sms-client/`](../crates/wadhu-sms-client/README.md).\
> **Research:**
> [`reports/Local SMS gateway solutions.md`](../reports/Local SMS gateway solutions.md).\
> **Independence:** [`WADHU_VISION_AND_PRINCIPLES.md`](WADHU_VISION_AND_PRINCIPLES.md) §3.

------------------------------------------------------------------------

## 1. Roles (do not mix them)

| Role | Meaning | In Wadhu |
|---|---|---|
| **Gateway** | Android-first app (phones, tablets, other Android OS devices with a SIM). Owns the radio. Accepts jobs over **LAN**, **Bluetooth**, and optional **relay**. | [`apps/wadhu-sms-gateway`](../apps/wadhu-sms-gateway/README.md) |
| **Relay** | Optional self-hosted remote plane. Device registry, client keys, internet `POST /v1/messages`, push to phones over outbound WebSocket. **Does not own a radio.** | [`apps/wadhu-sms-relay`](../apps/wadhu-sms-relay/README.md) |
| **Client** | Any app that needs SMS — calls the gateway (LAN/BT) or the relay (internet). | Other Gotigin brands / third parties + [`crates/wadhu-sms-client`](../crates/wadhu-sms-client/README.md) + desktop demo [`apps/wadhu-sms-client-demo`](../apps/wadhu-sms-client-demo/README.md) |

Local LAN/BT send **never** requires the relay. Killing the relay must not break local send.
Hosted identity / org billing (when built) monetizes **relay/platform only** —
see [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md).

**How each approach feels for non-experts** (no IP typing on LAN; gateway
controls for block / usage / rotate): [`TRANSPORTS.md`](TRANSPORTS.md).

`wadhu-sms-client` compiles for native (Linux / Windows / macOS / Android / iOS)
and `wasm32` (browsers / Edge). The **demo app** is desktop-only for now.
Gateway and relay answer CORS so browser third parties can call LAN/HTTP.

## 2. Transports

```text
  Clients
    ├── LAN HTTP (+ discover / pair code) ──┐
    ├── Bluetooth framing ──────────────────┼──►  sms-gateway  ──SIM──►  mobile network
    └── Internet HTTP ──► sms-relay ──WSS───┘
```

One job model: all transports feed the same outbox and status machine.

**LAN default UX:** brand app discovers the phone on Wi‑Fi and pairs with a
short **Allow** code from SMS Gateway — users do not enter IP addresses.
Phone owner can list connected apps, see send counts, block an app, and
change the access key. Details: [`TRANSPORTS.md`](TRANSPORTS.md).

## 3. Shared HTTP message API

Same paths on the **phone** and the **relay** so third parties learn once:

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/v1/health` | Liveness + summary |
| `GET` | `/v1/capabilities` | What this endpoint can do |
| `POST` | `/v1/messages` | Accept `{to,body,idempotency_key?}` → `202` |
| `GET` | `/v1/messages` | List recent |
| `GET` | `/v1/messages/{id}` | Status |

Auth: `Authorization: Bearer <token>` or `X-Api-Key`.

CORS: `OPTIONS` preflight + `Access-Control-Allow-Origin: *` (and allow
`Authorization`, `Content-Type`, `X-Api-Key`) on gateway and relay HTTP.

## 4. Status machine

`queued` → `submitting` → `submitted` → `delivered` | `failed` | `expired` | `cancelled`

Optional: `indeterminate` when radio handoff is unknown (e.g. process died mid-submit).

After handoff, **do not** auto-resend the same idempotency key.

## 5. Non-goals

- Mandatory Gotigin cloud to send
- Commercial SMS provider adapters (future, behind the same job model)
- WhatsApp / RCS replacement
- Brand business logic inside gateway / relay / SMS crates
