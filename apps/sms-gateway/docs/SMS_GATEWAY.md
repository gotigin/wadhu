# SMS Gateway — app brief

> **Package:** `wadhu-sms-gateway`\
> **Ids:** production `com.wadhu.smsgateway` · development
> `com.wadhu.smsgateway.debug`\
> **Doctrine:** [`docs/LOCAL_SMS.md`](../../../docs/LOCAL_SMS.md)\
> **OpenAPI:** [`openapi-v1.yaml`](openapi-v1.yaml) · **Bluetooth:** [`BLUETOOTH.md`](BLUETOOTH.md)\
> **Plan:** [`PLAN.md`](PLAN.md) · **Research:** [`RESEARCH.md`](RESEARCH.md)

## Role

| Role | Who |
|---|---|
| **Gateway (this app)** | Owns the SIM radio; accepts jobs over LAN / Bluetooth / optional relay; queues; sends; reports status |
| **Clients** | Kumkaj / other apps / third parties — call this app’s LAN API, Bluetooth framing, or talk to `sms-relay` for internet |

## What it is

SMS **execution device**. Install on an Android phone with a SIM:

- **LAN** — HTTP on `:18765` (OpenAPI above)
- **Bluetooth** — length-prefixed JSON framing ([BLUETOOTH.md](BLUETOOTH.md)); desktop also exposes TCP `:18766` for the same framing
- **Relay** — optional outbound WebSocket to a self-hosted [`sms-relay`](../../sms-relay/README.md); local send never depends on it

API key + outbox persist across restarts under the app data dir (`WADHU_SMS_GATEWAY_DATA` override).

## Client API (LAN)

Listen: Android `0.0.0.0:18765`, desktop `127.0.0.1:18765`.  
Auth: `X-Api-Key` / `Authorization: Bearer` (token shown in the operator UI).

| Method | Path | Purpose |
|---|---|---|
| GET | `/v1/health` | Radio + transports + queue |
| GET | `/v1/capabilities` | What this instance can do |
| POST | `/v1/messages` | Accept `{to,body,idempotency_key?}` → queued |
| GET | `/v1/messages` | Recent messages |
| GET | `/v1/messages/{id}` | One message status |

## Scope

**In:** outbox, LAN HTTP, Bluetooth framing, relay client, Android SIM, operator UI, persistence.  
**Out:** commercial SMS providers; brand business logic; requiring cloud for local send.
