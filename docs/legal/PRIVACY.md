# Wadhu privacy policy

_Last updated: 20 November 2026_

## Who we are

| | |
| --- | --- |
| **Trademark** | **Gotigin** (house mark) |
| **Company** | **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED** (“we”, “us”) |
| **Brand** | **Wadhu** (*more* in Memon — *Tools · Apps · More*) — reusable tools, utils, and apps — [wadhu.com](https://wadhu.com) |
| CIN | U62011GJ2026PTC179660 |
| PAN | AANCG1628G |
| GSTIN | 24AANCG1628G1ZP |
| Registered office | Gosiya Manzil, Darbargadh, Dhoraji, Rajkot, Gujarat 360410, India |
| Contact | [support@gotigin.com](mailto:support@gotigin.com) |

This policy covers **Wadhu brand** products and surfaces listed below. Apps
under other Gotigin brands (for example Khaynjo under Kumkaj) publish **their
own** privacy policies on their own domains. Names: [About](/about).

## Products covered

| Product | Where | Application / host id |
| --- | --- | --- |
| Brand site | [wadhu.com](https://wadhu.com) | Public website and account pages |
| **SMS Gateway** | Google Play / sideload / desktop packages | `com.wadhu.smsgateway` |
| SMS Relay | Self-hosted or Gotigin-hosted relay | Operator-configured host |
| **Wadhu Connect** | [connect.wadhu.com](https://connect.wadhu.com) | Meta Tech Provider bridge |

## Summary

- **Local-first SMS:** LAN and Bluetooth use of SMS Gateway keeps message
  content and API tokens on **your devices**. We do not need a wadhu.com
  account for local send.
- **Optional cloud:** Hosted SMS Relay or multi-tenant Connect may process
  data on our (or your self-hosted) servers only when you choose those paths.
- **No sale of personal data.** We do not sell personal information or use
  Wadhu product data for third-party advertising.
- **Deletion:** see [Data deletion](https://wadhu.com/data-deletion).

## Data we process (by product)

### Brand site (wadhu.com)

- Pages you request; standard server / hosting logs (IP, user agent, time).
- Optional analytics **only if** enabled on a given deploy (can be omitted).
- Email or phone you send us for support.
- **wadhu.com account** (only if you register on [/account](/account)): the
  email address or phone number you sign in with, your display name, and, if
  you use Google, Microsoft, or GitHub sign-in, the account id and basic
  profile that provider shares. Sign-in is run by Firebase Authentication
  (Google). An account is for hosted features; local tools do not need one.

### SMS Gateway (`com.wadhu.smsgateway`)

Typical data **on the device you install**:

- Gateway settings (listen address, paired clients, Bluetooth state)
- API tokens you generate for LAN / Bluetooth clients
- Outbound SMS queue / history the app keeps for operation and debugging
- Message bodies and destination numbers you (or a client you authorised)
  submit for sending
- Android permission state for SMS / network / Bluetooth as required to
  operate the gateway

**What leaves the device**

- SMS payloads go to the mobile carrier / Android telephony stack when you
  send.
- LAN or Bluetooth clients on networks **you** control receive API responses
  you authorised.
- If you enable **SMS Relay**, message metadata and bodies needed to send are
  transmitted to that relay endpoint (yours or a Gotigin-hosted relay you
  opted into).

Uninstalling the Android app or clearing app storage removes on-device
gateway data for that install. Export or note tokens before you wipe a
device.

### SMS Relay

When you run or use a relay: API keys, paired gateway identifiers, and
message jobs required to deliver SMS. Self-hosted relays are under **your**
control; Gotigin-hosted relays retain data only as needed to provide the
service and for short-term abuse / debugging (see Retention).

### Wadhu Connect (WhatsApp Cloud API bridge)

After Embedded Signup / Login for Business:

- WhatsApp Business Account IDs, phone number IDs, business IDs
- Access tokens (stored server-side to call Meta Graph APIs)
- Webhook payloads Meta sends (messages, delivery statuses, account updates)
  so we can operate the bridge for your brand apps
- Operational logs (errors, session events) for debugging

We use Meta permissions only to send and receive WhatsApp messages, manage
templates / WABA linkage, and process webhooks after you (or your business
admin) grant access.

## Purposes

| Purpose | Examples |
| --- | --- |
| Provide the product | Send SMS; complete Connect onboarding; deliver webhooks to integrators |
| Secure and debug | Signature checks, rate limits, error logs |
| Support | Respond when you email us |
| Legal compliance | Respond to lawful requests; keep minimal records |

We do **not** use product message content to build advertising profiles.

## Permissions (SMS Gateway on Android)

Depending on build and features, the app may request:

- **SMS-related** permissions to send (and, if enabled for a default-SMS or
  approved Play path, read/receive) messages on the device SIM
- **Internet / network** for LAN API and optional relay
- **Bluetooth** for nearby client pairing
- **Notifications** (if used) for send status

Play distribution of SMS capabilities is subject to Google Play SMS/Call Log
policy (default handler and/or approved exception). Sideload builds may
differ; the privacy commitments in this file still apply.

## Sharing

We share data only as needed to run the product:

| Recipient | Why |
| --- | --- |
| Meta Platforms (WhatsApp Cloud API) | Connect messaging after you grant access |
| Mobile carriers / device OS | Deliver SMS you initiate |
| Hosting / infrastructure providers | Run wadhu.com, connect.wadhu.com, or a hosted relay |
| Google Firebase (Hosting, Authentication, optional Analytics) | Serve wadhu.com and run account sign-in |
| Sign-in providers you choose (Google, Microsoft, GitHub) | Confirm who you are when you sign in with them |
| Your own systems | Brand apps and clients you configure to call our APIs |

We do not sell personal data. We may disclose information if required by law
or to protect users, our rights, or safety.

## Retention

| Data | Retention |
| --- | --- |
| On-device SMS Gateway data | Until you clear storage, uninstall, or delete in-app |
| Connect tokens & active connection | While the connection is active; remove on your deletion request or disconnect |
| wadhu.com account | Until you ask us to delete it |
| Webhook / server logs | Limited period for abuse prevention and debugging, then deleted or aggregated |
| Support email | As needed to resolve your request, then routine mailbox retention |

## Security

We use HTTPS for public Connect and site traffic, store Connect tokens on the
server side (not in the Embedded Signup browser page as the app secret), and
verify Meta webhook signatures when configured. No method is perfect; protect
device PINs, API tokens, and backups as you would a cash box.

## International transfers

Servers for wadhu.com / connect.wadhu.com may be located outside India
(depending on hosting). Where we transfer personal data, we do so to provide
the service you requested and under applicable law (including India’s DPDP
Act, 2023).

## Children

Wadhu products are for businesses and developers. They are not directed at
children under 18 (or the digital age of consent in your jurisdiction). We
do not knowingly collect children’s data.

## Your rights

Depending on applicable law (including India’s Digital Personal Data
Protection Act, 2023), you may request access, correction, or deletion of
personal data we hold, or withdraw consent where processing is consent-based.
See [Data deletion](https://wadhu.com/data-deletion) and Contact below.

## Where this policy applies

This is the **one** privacy policy for the **Wadhu brand** products listed
above and for [wadhu.com](https://wadhu.com). The same text is published at:

- [wadhu.com/privacy](https://wadhu.com/privacy)
- [connect.wadhu.com/privacy](https://connect.wadhu.com/privacy)

Related: [Terms](https://wadhu.com/terms) ·
[Cancellation](https://wadhu.com/cancellation) ·
[Data deletion](https://wadhu.com/data-deletion) ·
[Support](https://wadhu.com/support).

## Contact / grievance

**GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED**

- Privacy / support: [support@gotigin.com](mailto:support@gotigin.com)
- Data deletion subject line: `Wadhu data deletion`
- Registered office: Gosiya Manzil, Darbargadh, Dhoraji,
  Rajkot, Gujarat 360410, India
- Primary contact (director): Mearaj Abdulaziz Bhagad

Under the DPDP Act, 2023, if Gotigin appoints a distinct Data Protection
Officer or grievance officer, those details will be added here without
changing the URL of this policy.

## Changes

We may update this policy when products change. The “Last updated” date
above will change. Continued use after an update means you accept the
revised policy for that version of the product.
