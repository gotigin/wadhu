# SMS Gateway — research index

Full narrative report (market, stacks, architecture, compliance, Wadhu
recommendation):

[`reports/Local SMS gateway solutions.md`](../../../reports/Local SMS gateway solutions.md)

Supporting notes (same research pass):

| Notes | Topic |
|---|---|
| [`research_notes/.../android_local_gateways.md`](../../../research_notes/Local SMS gateway solutions/android_local_gateways.md) | Android LAN gateways vs cloud-relayed “SIM” apps |
| [`research_notes/.../modem_hardware_stacks.md`](../../../research_notes/Local SMS gateway solutions/modem_hardware_stacks.md) | USB modems, Gammu/Kannel/smstools, appliances |
| [`research_notes/.../api_architecture_patterns.md`](../../../research_notes/Local SMS gateway solutions/api_architecture_patterns.md) | Outbox, idempotency, pairing, multi-client |
| [`research_notes/.../compliance_platform_limits.md`](../../../research_notes/Local SMS gateway solutions/compliance_platform_limits.md) | Android/Play, carriers, India TCCCPR, EU/US |
| [`research_notes/.../gaps_and_opportunities.md`](../../../research_notes/Local SMS gateway solutions/gaps_and_opportunities.md) | Where incumbents fail; how to win |

## How to read incumbents (provider, not client)

Products like Capcom **SMSGate Local**, LibreSMS, and Traccar’s phone HTTP
API are **gateways**: other software connects *to them*. That is our role.
We study them as peers of **SMS Gateway**, not as something our app should
call. Cloud-relayed “SIM” apps (SimGate, Telerivet, …) are also providers,
but with a vendor control plane we refuse.

## Findings that shape **this** gateway

1. **Peer gateways:** SMSGate Local, LibreSMS, Traccar phone HTTP — LAN API
   on the radio device. Match that provider shape.
2. **Ops failure mode:** silent stalls (OEM kill, Wi-Fi sleep, HiLink vs
   modem) — gateway health UX matters more than another send endpoint.
3. **Desktop radio:** Gammu / ModemManager / Kannel / smstools when *this*
   app runs on PC with a stick — still the same gateway API to clients.
4. **Client DX we expose:** REST accept + queue + status/webhooks,
   Idempotency-Key, capabilities, dry-run.
5. **Compliance on the gateway:** caps, STOP, jurisdiction defaults —
   clients must not each invent policy.

Doctrine: [`docs/LOCAL_SMS.md`](../../../docs/LOCAL_SMS.md).
Plan: [`PLAN.md`](PLAN.md).
