# SMS Gateway — Google Play Data safety (paste guide)

Application id: `com.wadhu.smsgateway`  
Privacy policy URL: **https://wadhu.com/privacy**  
Canonical policy text: [`docs/legal/PRIVACY.md`](../../../docs/legal/PRIVACY.md)

Answer Play Console **Data safety** from this sheet. Re-check when permissions
or cloud features change.

## Overview answers

| Question | Answer |
| --- | --- |
| Does your app collect or share any of the required user data types? | **Yes** (SMS-related content and app activity on device; see below) |
| Is all user data encrypted in transit? | **Yes** for HTTPS relay / internet paths you enable; local LAN may be HTTP on the shop network — disclose as such if Play asks for “encrypted in transit” only for internet |
| Do you provide a way for users to request deletion? | **Yes** — https://wadhu.com/data-deletion |
| Independent security review? | **No** (unless you commission one later) |

## Data types (typical local-first build)

Declare collection that happens **on the device** even when Gotigin servers
never see it — Play’s form covers data the app handles.

| Data type | Collected? | Shared with other companies? | Purpose | Ephemeral? | Required? |
| --- | --- | --- | --- | --- | --- |
| SMS or MMS messages (bodies, numbers you send) | Yes | No* | App functionality | No | Yes (core) |
| Phone number (device / SIM related as exposed by OS for send) | Yes (as needed to send) | No* | App functionality | No | Yes |
| App activity / diagnostics (send queue, errors) | Yes | No* | App functionality, Analytics (if you only use local logs — prefer **App functionality**) | No | No |
| Device or other IDs (API tokens you generate) | Yes | No* | App functionality, Developer communications | No | Yes |
| Approximate / precise location | **No** (do not declare unless you add it) | — | — | — | — |
| Personal info (name, email) inside the app | **No** by default | — | — | — | — |
| Photos / files | **No** | — | — | — | — |

\* **Shared:** Do **not** mark “shared” for Gotigin cloud unless the user
enables a **Gotigin-hosted SMS Relay**. Carrier delivery of SMS is OS/carrier
transport for a message the user initiated — follow Play’s current guidance;
usually you still declare SMS **collected** by the app. If a hosted relay is
offered and on by default, mark relevant types as shared with “Developer” /
service provider and point at the privacy policy.

## Security practices checklist

- [x] Data encrypted in transit for public HTTPS endpoints (Connect / site / hosted relay)
- [x] Users can request deletion (wadhu.com/data-deletion + clear app storage)
- [ ] Committed to Play Families / kids — **No** (business tool)
- [ ] Account creation required — **No** for local LAN/Bluetooth

## After you change the product

If you add accounts, analytics SDKs, or a mandatory cloud relay, update
[`docs/legal/PRIVACY.md`](../../../docs/legal/PRIVACY.md) first, regenerate
the site (`present-legal.sh`), then update this sheet and Play Console.
