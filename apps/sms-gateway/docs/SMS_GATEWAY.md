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
| **Clients** | Kumkaj / other apps / third parties — LAN API, Bluetooth framing, or [`sms-relay`](../../sms-relay/README.md) for internet |

## What it is

SMS **execution device** for Android. Install where the SIM lives:

- **LAN** — HTTP on `:18765` (OpenAPI above)
- **Bluetooth** — length-prefixed JSON framing ([BLUETOOTH.md](BLUETOOTH.md)); TCP `:18766` framing for lab/desktop clients
- **Relay** — optional outbound WebSocket to a self-hosted [`sms-relay`](../../sms-relay/README.md)

Desktop builds are **dry-run only** (operator UI + API; no SIM). Shipping
and day-to-day use are Android-first.

Telephony is declared **optional** in the manifest so tablets without a
SIM can install; the radio reports unavailable until a SIM is ready.

API key + outbox persist across restarts (`WADHU_SMS_GATEWAY_DATA` override).

## Client API (LAN)

Listen: Android `0.0.0.0:18765`.  
Auth: `X-Api-Key` / `Authorization: Bearer` (token on the operator screen).

| Method | Path | Purpose |
|---|---|---|
| GET | `/v1/health` | Radio + transports + queue |
| GET | `/v1/capabilities` | What this instance can do |
| POST | `/v1/messages` | Accept `{to,body,idempotency_key?}` → queued |
| GET | `/v1/messages` | Recent messages |
| GET | `/v1/messages/{id}` | One message status |

## Scope

**In (now):** Android SIM, LAN, Bluetooth framing, relay client, operator UI, persistence.  
**Out for now:** USB modem / desktop radio as a product path; commercial SMS providers; brand business logic.
