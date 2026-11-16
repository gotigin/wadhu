# SMS Gateway — app brief

> **Package:** `wadhu-sms-gateway`\
> **Primary target:** Android OS devices (phones, tablets, and other form
> factors with a usable SIM)\
> **Ids:** production `com.wadhu.smsgateway` · development
> `com.wadhu.smsgateway.debug`\
> **Doctrine:** [`docs/LOCAL_SMS.md`](../../../docs/LOCAL_SMS.md)\
> **OpenAPI:** [`openapi-v1.yaml`](openapi-v1.yaml) · **Bluetooth:** [`BLUETOOTH.md`](BLUETOOTH.md)\
> **Plan:** [`PLAN.md`](PLAN.md) · **Research:** [`RESEARCH.md`](RESEARCH.md)

## Role

| Role | Who |
|---|---|
| **Gateway (this app)** | Runs on an Android device; owns the SIM radio; accepts jobs over LAN / Bluetooth / optional relay |
| **Clients** | Other Gotigin brands / third parties — LAN API, Bluetooth framing, or [`sms-relay`](../../wadhu-sms-relay/README.md) for internet |

## What it is

SMS **execution device** for Android. Install where the SIM lives:

- **LAN** — HTTP on `:18765` + UDP discover `:18767` + **Allow a new app** pairing code (no IP typing for brand users). See [`docs/TRANSPORTS.md`](../../../docs/TRANSPORTS.md).
- **Bluetooth** — Classic RFCOMM SPP on Android + length-prefixed JSON ([BLUETOOTH.md](BLUETOOTH.md)); desktop lab TCP `:18766` (same framing; `wadhu-sms-client::framed`)
- **Relay** — optional outbound WebSocket to a self-hosted [`sms-relay`](../../wadhu-sms-relay/README.md)

Desktop builds are **dry-run only** (operator UI + API; no SIM). Shipping
and day-to-day use are Android-first.

Telephony is declared **optional** in the manifest so tablets without a
SIM can install; the radio reports unavailable until a SIM is ready.

Operator key, paired apps, outbox persist across restarts (`WADHU_SMS_GATEWAY_DATA` override).

## Operator UI (security)

| Action | Effect |
|--------|--------|
| **Allow a new app** | Shows a 6-digit code (~5 min). Brand app discovers the phone and pairs. |
| **Connected apps** | Label, SMS count, Allowed / Blocked |
| **Block / Unblock** | Stops or restores sends for that app |
| **Change access key** | New operator key; clears paired apps (they must Allow again) |

## Client API (LAN)

Listen: Android `0.0.0.0:18765`. Discover: UDP `:18767`.  
Auth: per-app token from `/v1/pair`, or operator key (advanced).

| Method | Path | Purpose |
|---|---|---|
| GET | `/v1/health` | Radio + transports + queue |
| GET | `/v1/capabilities` | What this instance can do |
| POST | `/v1/pair` | `{code,label}` → per-app token (no prior auth) |
| POST | `/v1/messages` | Accept `{to,body,idempotency_key?}` → queued |
| GET | `/v1/messages` | Recent messages |
| GET | `/v1/messages/{id}` | One message status |
| GET | `/v1/clients` | Operator: list apps + counts |
| POST | `/v1/clients/{id}/block` | Operator: block |
| POST | `/v1/clients/{id}/unblock` | Operator: unblock |
| POST | `/v1/admin/rotate-key` | Operator: rotate key + revoke apps |

## Scope

**In (now):** Android SIM, LAN discover/pair, Bluetooth framing, relay client, operator UI, persistence, connected-app controls.  
**Out for now:** USB modem / desktop radio as a product path; commercial SMS providers; brand business logic.
