# SMS Relay — app brief

> **Package:** `wadhu-sms-relay`\
> **Role:** optional remote plane for Wadhu SMS Gateway\
> **Doctrine:** [`docs/LOCAL_SMS.md`](../../../docs/LOCAL_SMS.md)

## What it is

A **self-hostable** Rust service. It does **not** own a SIM. It:

1. Registers Android `sms-gateway` devices
2. Issues client API keys for third parties / Gotigin apps
3. Accepts `POST /v1/messages` (same JSON as the phone LAN API)
4. Pushes jobs to online devices over `/v1/device/ws`
5. Stores status updates from devices

## Quick admin flow

```bash
export ADMIN=dev-admin-token
export RELAY=http://127.0.0.1:9876

# 1. Pair a device (returns device_id + device_secret for the phone UI)
curl -s -H "Authorization: Bearer $ADMIN" -X POST $RELAY/v1/devices/pair

# 2. Issue a client key
curl -s -H "Authorization: Bearer $ADMIN" -H 'Content-Type: application/json' \
  -d '{"label":"my-app"}' $RELAY/v1/keys

# 3. Send (after the phone is connected)
curl -s -H "Authorization: Bearer $CLIENT_TOKEN" -H 'Content-Type: application/json' \
  -d '{"to":"+91…","body":"hello","idempotency_key":"1"}' \
  $RELAY/v1/messages
```

## Non-goals

- Owning a radio / sending SMS itself
- Commercial SMS provider adapters
- Mandatory Gotigin cloud account
