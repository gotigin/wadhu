# Third-party integrator guide

You can send SMS through Wadhu without any Gotigin SDK.

## Choose an endpoint

1. **Local only** — install [SMS Gateway](../../wadhu-sms-gateway/README.md) on an
   Android phone (or desktop for API dry-runs). Call its LAN HTTP API.
2. **Remote** — run [SMS Relay](../README.md) yourself; pair the phone; call
   the relay’s HTTP API from anywhere.

Message JSON and status strings are the **same** on both.

## Auth

```http
Authorization: Bearer <token>
```

or `X-Api-Key: <token>`.

- On the **phone**: token is shown in the operator UI (persisted).
- On the **relay**: mint a client key with the admin token (`POST /v1/keys`).

## Send

```bash
curl -s -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"to":"+919876543210","body":"Hello","idempotency_key":"order-42"}' \
  http://HOST:PORT/v1/messages
```

Response `202` with a message object. Poll `GET /v1/messages/{id}`.

## Idempotency

Always send a stable `idempotency_key` for retries. After the radio has
accepted a send (`submitted` / `failed` / `indeterminate`), the same key
must **not** cause another SIM send.

## Bluetooth

See [BLUETOOTH.md](../../wadhu-sms-gateway/docs/BLUETOOTH.md) for length-prefixed
JSON framing (same fields as HTTP).

## Rust helper

Optional: [`wadhu-sms-client`](../../../crates/wadhu-sms-client/README.md).
