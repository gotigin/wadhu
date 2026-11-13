# Gaps and opportunities for local SMS gateways

## Common failure modes of Android SMS gateways and modem stacks in real shops/SMBs?

### Takeaway
Android phone gateways fail mainly because OEM battery killers, Doze, dual-SIM ambiguity, and carrier spam filters silently stop or throttle sends; USB/GSM modem stacks fail from vendor-divergent AT behaviour, freezes needing power-cycle, SIM credit/allowance exhaustion, and operator CMS rejects that look like hardware bugs.

### Cited Findings
- SMSGate (capcom6) maintainers attribute high unexplained send-failure rates on Xiaomi to vendor-specific OS error codes not in Android docs, and advise disabling MIUI optimizations — [GitHub discussion #177](https://github.com/capcom6/android-sms-gateway/discussions/177).
- SMSGate FAQ states Local Mode needs a foreground service + wake lock; battery optimization disable raises reliability but drains battery; OEM behaviour varies; they point operators to Don’t Kill My App — [SMSGate FAQ – General](https://docs.sms-gate.app/faq/general/); [SMSGate FAQ – Local Server](https://docs.sms-gate.app/faq/local-server/).
- SMSGate Local Server FAQ: Android may kill the local HTTP server (Xiaomi/Samsung aggressive); tell is missing persistent notification; Wi-Fi may sleep and make the phone unreachable even if cellular works — [SMSGate FAQ – Local Server](https://docs.sms-gate.app/faq/local-server/).
- textbee’s 2026 ops guide: classic failure pattern is “works a few minutes, then stops when screen off”; causes ranked as Doze/App Standby → OEM killers (Samsung sleeping apps re-apply after ~3 days; Xiaomi Autostart; Huawei App launch) → stuck non-final status while device offline → carrier throttling of identical bursts on consumer SIMs — [textbee battery optimization guide](https://textbee.dev/blog/android-sms-gateway-not-sending-battery-optimization).
- Tekservix gateway ops checklist: cleaner/booster apps, system updates resetting battery settings, swiping the app away, ultra power-saving, and SIM swaps undoing dual-SIM config are common silent undos; recommends dedicated charged phone + offline alerts — [tekservix SMS gateway battery settings](https://tekservix.com/blog/sms-gateway-app-battery-settings).
- Play Store / review aggregations for older cloud-tied gateways report pending forever, truncated multipart SMS, dual-SIM sending from both SIMs, server 502/504 outages, and “requires Internet” false positives even when LAN/cellular is fine — [AndroidBlip SMS Gateway API reviews](https://www.androidblip.com/android-apps/networked.solutions.sms.gateway.api.html); [Traccar SMS Gateway Play reviews](https://play.google.com/store/apps/details?id=org.traccar.gateway&hl=en).
- ProCat Solutions (operators of own modem pools): manufacturer AT dialects diverge; text mode fails for accents/multipart (need PDU); serial port must be serialized per modem; modems freeze and only USB reset / power cycle recovers — [ProCat GSM modem SMS gateway post](https://procatsolutions.com/en/blog/sms-gateway-and-gsm-modems/).
- Diafaan SMS Server support: USB “dongle” modems are hit-or-miss across PCs/OS/networks; AT reboot (`AT+CFUN=1,1`) often worsens USB sticks (disabled by default for USB, enabled for serial/LAN); CMS ERROR 21/28 are network rejects (spam/credit/number format), not modem faults, yet software that auto-reboots on send error can thrash the pool — [Diafaan Huawei E173 thread](https://www.diafaan.com/support-forum/using-diafaan-sms-server/huawei-e173-problems/); [Diafaan 8-port pool CMS errors](https://www.diafaan.com/support-forum/using-diafaan-sms-server/8-port-usb-gsm-modem-pool-refuses-to-send-messages/); [Diafaan GSM Modem gateway reference](https://www.diafaan.com/manuals/diafaansmsserver/gsm_modem_gateway.html).
- SIM ops are a first-class failure domain: credit/allowance exhaustion, operator suspension of heavy-traffic SIMs, and delivery-report matching that must key by modem + time window because network refs repeat — [ProCat GSM modem SMS gateway post](https://procatsolutions.com/en/blog/sms-gateway-and-gsm-modems/).
- SMSEagle support: modem status 302 often means the carrier blocked the SIM; recovery is daemon/modem restart or SIM replacement — [SMSEagle status 302 article](https://support.smseagle.eu/hc/en-us/articles/360016485253-Gateway-is-not-sending-messages-Modem-log-reports-Status-302).
- Prepaid balance/USSD while a daemon owns the serial port is historically painful (stop SMSD / inject USSD / operator-specific codes) — [Gammu issue #453](https://github.com/gammu/gammu/issues/453); [Kannel users archive on USSD credit](https://www.kannel.org/pipermail/users/2007-November/003545.html).
- Self-hosted Reddit advice (2025): phone-as-gateway is fine short-term but reliability “gets messy fast” at scale; prefer carrier API for production-critical traffic — [r/selfhosted Android SMS gateway 2025](https://www.reddit.com/r/selfhosted/comments/1mq017j/whats_the_best_android_sms_gateway_app_for_a/).

### Inferences
- For SMB shops, the dominant “is it broken?” moment is a silent queue stall (phone asleep / Wi-Fi slept / modem frozen), not an explicit API error — status UX and heartbeats matter as much as send throughput.
- Consumer SIMs + marketing-shaped bursts will keep failing even with perfect software; a shop product must separate transactional drain (bills, OTPs, order alerts) from blast semantics and rate-limit accordingly.
- USB modem support without a supported-device list and power-cycle story will generate support load disproportionate to feature value (Diafaan/ProCat experience).

### Gaps
- No public quantitative failure-rate study specific to Indian/SEA SMB retail shops was found; most sources are global self-host / OEM / European modem-operator blogs.
- Limited primary sources on Bluetooth-paired phone gateways (vs Wi-Fi local HTTP); most local-mode products assume LAN reachability to the phone IP.

## What’s missing for integrators (stable status API, offline-tolerant clients, capability discovery, multi-backend, typed clients, good docs)?

### Takeaway
Best incumbents already expose OpenAPI send/status/webhook surfaces, but integrators still lack a backend-agnostic outbox contract, durable offline client semantics when the gateway is unreachable, rich capability discovery (SIM slots, PDU/MMS, DLR, rate limits), and typed clients that treat “queued locally / accepted by radio / delivered by network” as distinct states with safe retry rules.

### Cited Findings
- SMSGate Local Mode: HTTP server on device, Basic Auth only (no JWT in local), Swagger at `/docs`, same message schema as cloud — [SMSGate Local Server](https://docs.sms-gate.app/getting-started/local-server/); [SMSGate API integration](https://docs.sms-gate.app/integration/api/).
- SMSGate tracks message state, cancel-pending, rate limits, working hours, multi-SIM, delivery-report toggle, inbox API (local-only), webhooks including `system:ping`; `/health` for liveness — [SMSGate GitHub README](https://github.com/capcom6/android-sms-gateway/); [SMSGate FAQ – General](https://docs.sms-gate.app/faq/general/); [feature request #170 health/status](https://github.com/capcom6/android-sms-gateway/issues/170).
- SMSGate notes external cloud functions cannot reach Local Server endpoints; cloud vs local accessibility differs even when request shape matches — [SMSGate API integration](https://docs.sms-gate.app/integration/api/).
- textbee documents a useful status ladder (`pending` → `dispatched` → `sent`/`delivered`/`failed`) and warns that retrying `dispatched` duplicates SMS — [textbee battery optimization guide](https://textbee.dev/blog/android-sms-gateway-not-sending-battery-optimization).
- ProCat’s production pattern: app never waits on modem; shared queue with priority, exponential backoff only for transient modem errors, idempotency window, and an internal send interface so modem vs aggregator is config — [ProCat GSM modem SMS gateway post](https://procatsolutions.com/en/blog/sms-gateway-and-gsm-modems/).
- SMSEagle (hardware appliance): OpenAPI v2 REST, access tokens, webhooks with up to 24h retry, webhook test button, signal/modem APIs, inbox/outbox model — [SMSEagle API](https://www.smseagle.eu/api/); [SMSEagle webhooks](https://www.smseagle.eu/features/webhooks/); [SMSEagle features](https://www.smseagle.eu/features/).
- Older Android gateways reviewed poorly for truncated long SMS, missing receive callbacks, dual-SIM selection, and cloud dependency that breaks when the vendor server is down — [AndroidBlip reviews](https://www.androidblip.com/android-apps/networked.solutions.sms.gateway.api.html).
- Kumkaj apps already queue SMS intents locally (`notify_outbox`); local drain paths (Android phone / USB modem / multi-SIM device) are Planned and must work over LAN/USB/Bluetooth without internet to Kumkaj Cloud — [Kumkaj SMS_MESSAGING.md](file:///home/eight/Gotigin/Kumkaj/kumkaj-workspace/docs/SMS_MESSAGING.md).

### Inferences
- **Wadhu-wide reusable (crate/protocol) gaps to close:** (1) durable outbox item ID + idempotency key; (2) explicit state machine separating `accepted_by_gateway` / `submitted_to_radio` / `network_accepted` / `delivered` / `permanent_failure` / `transient_failure`; (3) capability document (backends present, SIM count, max parts, DLR support, estimated SMS/min, supported transports); (4) offline-tolerant client: enqueue locally and drain when gateway returns; (5) typed Rust client + OpenAPI for non-Rust consumers.
- **App-specific gaps:** pairing UX, OEM battery setup wizard, SIM credit/health display, supported modem list, operator-facing queue screen — not library concerns.
- “Much better for integrators” means: one client library works against phone, USB modem, or appliance without rewriting retry logic; discovery replaces hardcoded curl-to-phone-IP docs; docs show safe retry tables (as textbee does) not only happy-path curl.

### Gaps
- No published cross-product matrix of typed SDKs (Rust/Go/TS) for purely local (non-cloud) SMS gateways was found; most “SDKs” assume cloud or HTTP-only examples.
- Capability-discovery as a first-class endpoint is weakly documented across Android gateways (health/ping exist; rich capability blobs less so).

## How should one product unify Android-SIM + USB modem + (later) multi-SIM appliance behind one outbox/API?

### Takeaway
Unify at an **outbox + backend adapter** layer: apps submit durable intents to one API; backends (Android agent, USB modem worker, multi-SIM appliance) claim jobs, report typed statuses, and publish capabilities; never put radio/AT details in product apps — matching both Kumkaj’s queue-first plan and ProCat’s “abstract the sending layer” practice.

### Cited Findings
- Kumkaj planned local paths all drain the **same** outbox; complementary approaches (phone SIM, USB modem, multi-SIM appliance); Cloud SMS is optional and must not hostage the local queue — [Kumkaj SMS_MESSAGING.md](file:///home/eight/Gotigin/Kumkaj/kumkaj-workspace/docs/SMS_MESSAGING.md).
- ProCat: one worker per modem owns the serial port, shared Redis queue, capacity shrinks when a modem drops, app talks only to an internal interface — [ProCat GSM modem SMS gateway post](https://procatsolutions.com/en/blog/sms-gateway-and-gsm-modems/).
- SMSGate already unifies Local/Cloud/Private behind one message API shape, but only for Android backends — [SMSGate Overview](https://docs.sms-gate.app/getting-started/).
- SMSEagle shows the appliance end-state: on-prem box, REST + webhooks, multi-modem throughput quoted per modem, queue in DB — [SMSEagle features](https://www.smseagle.eu/features/); [SMSEagle Alertmanager adapter rationale (offline hardware)](https://github.com/smseagle/alertmanager-sms-calls).
- Diafaan’s gateway properties (reboot-after-error, permanent error list, PDU modes, SMSC) illustrate how much modem-specific policy must stay **behind** the adapter, not in clients — [Diafaan GSM Modem gateway reference](https://www.diafaan.com/manuals/diafaansmsserver/gsm_modem_gateway.html).

### Inferences
- Recommended product shape for Wadhu:
  1. **Crate (`wadhu-sms` or similar):** outbox schema, status enum, capability schema, auth/pairing tokens, OpenAPI — brand-neutral.
  2. **SMS-Gateway app:** hosts the outbox on shop LAN (or attaches to Kumkaj Server’s outbox), runs pairing UI, owns Android companion / USB workers, later talks to supported appliances as just another backend.
  3. **Backends register** with `{backend_id, kind, sims[], sms_per_minute, supports_dlr, supports_ussd_balance, transport}`.
  4. **Routing policy** (app config): prefer dedicated modem > phone; failover; never fan-out same idempotency key to two radios.
  5. **Transport:** LAN HTTP(S) first; USB for modem stick on the PC running the gateway; Bluetooth as pairing/control assist for phone, not primary high-volume drain if Wi-Fi sleep is common.
- Do **not** make integrating apps (Kumkaj or external) depend on which radio is plugged in; they only call enqueue + watch status (Kumkaj independence / Wadhu crate rules align with this).

### Gaps
- No open standard equivalent to IPP/CloudPRNT for “local SMS outbox” exists in the sources reviewed; any Wadhu protocol would be greenfield (opportunity, but also adoption work).
- Bluetooth drain reliability for SMS payloads in shop RF environments was not evidenced in the sources found.

## What UX would make pairing and day-to-day operation trustworthy for non-technical shop staff?

### Takeaway
Trustworthy shop UX looks like a **counter appliance**: one pairing flow with a visible “online / sending / blocked” state, guided OEM battery setup, permanent-charge expectation, queue depth and last-error in plain language, and test-send — not raw local IPs, Basic Auth strings, and Don’t Kill My App wiki homework.

### Cited Findings
- SMSGate Local setup still surfaces local/public IP + Basic Auth credentials and curl examples — workable for developers, hostile for counter staff — [SMSGate Local Server](https://docs.sms-gate.app/getting-started/local-server/).
- SMSGate intentionally keeps SMS visible in the stock Messages app (transparency / anti-misuse) and recommends dedicated device + PIN — [SMSGate FAQ – General](https://docs.sms-gate.app/faq/general/).
- textbee/tekservix both reduce ops to a dedicated charged phone checklist and warn that cleaner apps / swiping recents / firmware updates undo settings — [textbee guide](https://textbee.dev/blog/android-sms-gateway-not-sending-battery-optimization); [tekservix guide](https://tekservix.com/blog/sms-gateway-app-battery-settings).
- Star Micronics shop hardware pattern: print self-test for IP → browser config → Save/Restart → physical “welcome/pairing” confirmation print — tangible success signal — [Star CloudPRNT configure URL](https://starmicronics.com/help-center/knowledge-base/how-to-configure-a-url-for-cloudprnt/); [starprinter.online connecting a device](https://docs.starprinter.online/articles/connecting_a_device.html).
- Stripe Terminal offline pairing: require a successful online pair first, then allow offline operation with clear error taxonomy when prerequisites aren’t met; store-and-forward with forward status callbacks — [Stripe Terminal offline overview](https://docs.stripe.com/terminal/features/operate-offline/overview?reader-type=bluetooth); [Collect card payments while offline](https://docs.stripe.com/terminal/features/operate-offline/collect-card-payments).
- SMSEagle webhooks include a **built-in destination test** before going live — [SMSEagle webhooks](https://www.smseagle.eu/features/webhooks/).
- Telerivet Play reviews include “hard to understand it worked locally” historically — [Telerivet Gateway Play listing](https://play.google.com/store/apps/details?id=com.rivetlabs.sms).

### Inferences
- **App-specific UX differentiators (Wadhu SMS-Gateway):**
  - Pairing: QR / short code from desktop Makepad app ↔ phone companion; no typing IPs.
  - Home screen: three lights — Radio (signal/SIM), Link (LAN to server), Queue (N waiting / oldest age).
  - First-run: OEM-specific battery wizard with “Done” checkboxes (Samsung/Xiaomi/…).
  - “Send test SMS to owner’s number” as the pairing completion moment (like Star’s welcome print).
  - Plain-language errors: “Phone went to sleep”, “SIM out of credit”, “Carrier blocked this number”, not CMS 28 / vendor error codes alone.
  - Staff affordances: lock screen PIN reminder, “don’t swipe this app”, charger plugged indicator.
- Copy Stripe’s rule: **pair while healthy, then tolerate offline** — integrating apps keep selling with queued intents (already Kumkaj doctrine).

### Gaps
- Little published UX research with non-technical shop owners on SMS gateway pairing; patterns above are inferred from adjacent POS hardware and integrator docs.

## What should stay out of scope (hosted blast SaaS, WhatsApp Business API replacement, requiring vendor cloud)?

### Takeaway
Stay a **local drain for already-queued, consented, mostly transactional SMS** on SIMs the shop owns; explicitly exclude hosted marketing blast SaaS, WhatsApp Business API automation, and any architecture that requires the vendor’s cloud to send.

### Cited Findings
- Kumkaj non-goals for local paths: not hosted blast/marketing SaaS; not WhatsApp Business API substitute; do not require internet to Kumkaj Cloud — LAN/USB/Bluetooth enough — [Kumkaj SMS_MESSAGING.md](file:///home/eight/Gotigin/Kumkaj/kumkaj-workspace/docs/SMS_MESSAGING.md).
- Kumkaj already has one-tap WhatsApp (`wa.me`) as a separate path; Cloud SMS is a later paid convenience that must not hostage books/outbox — [Kumkaj SMS_MESSAGING.md](file:///home/eight/Gotigin/Kumkaj/kumkaj-workspace/docs/SMS_MESSAGING.md).
- ProCat: own modems pay off for local number / two-way / independence when someone operates hardware; aggregators win for international / high volume / no ops staff — [ProCat GSM modem SMS gateway post](https://procatsolutions.com/en/blog/sms-gateway-and-gsm-modems/).
- textbee: Android gateway wrong tool for carrier-grade bulk; consumer SIMs get spam-filtered on identical bursts — [textbee battery optimization guide](https://textbee.dev/blog/android-sms-gateway-not-sending-battery-optimization).
- Many popular “Android SMS gateway” products are cloud-relayed (Telerivet forwards via internet to vendor servers; SMSGate offers optional public cloud) — [Telerivet Gateway Play listing](https://play.google.com/store/apps/details?id=com.rivetlabs.sms); [SMSGate Overview](https://docs.sms-gate.app/getting-started/).
- SMSGate FAQ: no RCS; no custom alphanumeric sender without carrier setup — [SMSGate FAQ – General](https://docs.sms-gate.app/faq/general/).

### Inferences
- Out of scope for Wadhu SMS-Gateway v1: campaign blast UI, DND scrubbing SaaS, WhatsApp/RCS channels, mandatory account at gotigin.cloud, phone-number provisioning, short codes.
- In scope adjacent (optional later, not core): Cloud SMS as **optional backend adapter** behind the same outbox (Kumkaj already plans this separately).
- Compliance (opt-in, quiet hours, TRAI/DND) belongs in **composing apps** (Kumkaj) and policy hooks — not as a hosted compliance product inside Wadhu.

### Gaps
- Jurisdiction-specific regulatory detail (e.g. India TRAI commercial SMS rules) was not deeply researched in this pass; product should consult local counsel before any marketing-adjacent features.

## Comparable local gateway products in adjacent domains — patterns worth copying?

### Takeaway
Copy **queue abstraction + capability-aware backends + tangible pairing confirmation + offline store-and-forward** from print (CUPS/IPP, Star CloudPRNT) and payments (Stripe Terminal); avoid copying cloud-mandatory relays that break the local-first promise.

### Cited Findings
- CUPS/Samba print servers: local spooler is the system of record; clients discover/share queues; Samba cannot itself be the remote print engine — backends sit under a stable client-facing queue model — [Ubuntu Samba print server](https://ubuntu.com/server/docs/how-to/samba/print-server.md); [SambaWiki print server](https://wiki.samba.org/index.php/Setting_up_Samba_as_a_Print_Server).
- Star CloudPRNT: printer polls or connects to a URL; staff configure once via self-test IP + web UI; success confirmed by printed welcome page; protocol documents device identity stable across Ethernet/Wi-Fi — [Star CloudPRNT protocol guide](https://star-m.jp/products/s_print/sdk/StarCloudPRNT/manual/en/); [Connecting a new device](https://docs.starprinter.online/articles/connecting_a_device.html).
- Stripe Terminal offline: encrypt/store payments locally, auto-forward on reconnect, explicit offline listener callbacks, prerequisite online pairing window — [Stripe offline overview](https://docs.stripe.com/terminal/features/operate-offline/overview?reader-type=bluetooth); [Collect offline payments](https://docs.stripe.com/terminal/features/operate-offline/collect-card-payments).
- SMSEagle positions as offline hardware gateway with OpenAPI + webhook retries — closest SMS analogue to a “local print server appliance” — [SMSEagle features](https://www.smseagle.eu/features/); [SMSEagle webhooks](https://www.smseagle.eu/features/webhooks/).
- ProCat’s modem-pool service is the SMS twin of a print spooler: serialize device access, shared queue, shrink capacity on failure — [ProCat GSM modem SMS gateway post](https://procatsolutions.com/en/blog/sms-gateway-and-gsm-modems/).

### Inferences
- Patterns worth copying into Wadhu:
  | Pattern | From | Apply as |
  |---|---|---|
  | Spooler as SoR | CUPS | Durable outbox owns intents; radios are backends |
  | Capability/URI discovery | IPP / CloudPRNT | `GET /capabilities` + pairing record |
  | Physical success signal | Star welcome print | Test SMS + green “Paired” state |
  | Store-and-forward | Stripe Terminal | Apps queue offline; gateway drains later |
  | Webhook retry budget | SMSEagle | Status push with multi-hour retry |
  | Supported device list | Diafaan/Star | Explicit USB modem / appliance allowlist |
- Patterns to **avoid**: vendor cloud as mandatory control plane (Telerivet-style); curl-and-IP as the only pairing story; treating CMS/network errors as modem reboot loops.

### Gaps
- Wadhu workspace search found no local `print-bridge` tree to mine for in-house precedents in this pass (path may live elsewhere or under another name).
- ESC/POS local bridge patterns inside Gotigin were not fetched here; worth a follow-up against Wadhu `apps/` / Kumkaj print docs when implementing.

---

## Cross-cutting differentiators (actionable)

### Wadhu-wide reusable (crates / protocol)
- Outbox item: `id`, `idempotency_key`, `to`, `body`, `priority`, `created_at`, `expires_at`, `backend_hint?`.
- Status enum with safe-retry table published in docs (mirror textbee’s ladder; extend with `radio_rejected` vs `gateway_unreachable`).
- Capability discovery document per backend.
- OpenAPI + `wadhu-sms` Rust client; no sibling-app imports (apps depend on crate only).
- Optional LAN mDNS/`wadhu-sms` service type for discovery (print-server analogue).

### SMS-Gateway app-specific (Makepad UI / ops)
- Pairing QR, OEM battery wizard, three-state health home, test send, SIM credit/signal panel, supported modem catalog, staff “dedicated phone” checklist.
- Android companion as a **backend agent**, not the product API surface integrators must learn.
- Never require Gotigin/Wadhu cloud to drain; optional cloud SMS only as another backend later.

### Explicit non-goals
- Hosted blast SaaS, WhatsApp Business API replacement, RCS, mandatory vendor cloud, unsupported USB gadget free-for-all.
