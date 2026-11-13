# Android local / physical-SIM SMS gateways (research notes)

Research date: 2026-11-08. Focus: non-cloud / self-hosted / local-SIM paths; cloud-backed products noted explicitly. Star counts and pricing are as reported by primary sources at research time and may drift.

## Which notable Android SMS gateway apps/projects exist?

### Takeaway
The densest local-first API gateways are Capcom’s SMSGate (`capcom6/android-sms-gateway`), Traccar SMS Gateway, LibreSMS, PlainApp, and KDE Connect (desktop companion). Large adjacent ecosystems (SmsForwarder, textbee, Telerivet, SimGate, commercial “SMS Forwarder”) mix true LAN-local paths with cloud-relayed control planes.

### Cited Findings
- **SMS Gateway for Android / SMSGate** (`capcom6/android-sms-gateway`): Apache-2.0 Android app that sends/receives SMS via API on-device (Local Server) or via public/private cloud servers; README reports ~5,987 stars / ~1,080 forks; homepage `https://docs.sms-gate.app`; latest pre-release noted as v1.78.2 (2026-11-03). — [GitHub README](https://github.com/capcom6/android-sms-gateway/); [Releases](https://github.com/capcom6/android-sms-gateway/releases); [sms-gate.app](https://sms-gate.app/)
- SMSGate operating modes: **Local** (data on device, LAN API), **Cloud** (`api.sms-gate.app`, uses FCM), **Private** (self-hosted server, single-command deploy). Local mode needs no account/email. — [sms-gate.app](https://sms-gate.app/); [Getting Started](https://docs.sms-gate.app/getting-started/)
- **textbee** (`textbee/textbee`): AGPLv3 (as of ~2026-11-04 changelog; pre-2026-11-04 code noted as MIT); NestJS API + Next.js dashboard + Kotlin Android app; ~3,295 stars / ~504 forks; claims “over 40,000 developers and businesses”; Android 7.0+; self-host requires MongoDB, Redis, Firebase FCM, SMTP, custom APK build. — [GitHub](https://github.com/textbee/textbee); [Self-hosting docs](https://textbee.dev/docs/self-hosting); [Pricing](https://textbee.dev/docs/pricing)
- **Traccar SMS Gateway** (`traccar/traccar-sms-gateway`): GPL-3.0 Android messaging app based on Fossify Messages; exposes HTTP API for sending SMS; ~682–700 stars / ~141 forks; Play Store distribution; originally for Traccar GPS notifications but usable as generic HTTP SMS API. — [GitHub](https://github.com/traccar/traccar-sms-gateway); [Traccar docs](https://www.traccar.org/sms-gateway/); [HTTP SMS API](https://www.traccar.org/http-sms-api/)
- **LibreSMS**: .NET MAUI Android app; local HTTP server (default port 8686); send SMS/MMS, inbox read, webhooks; no subscriptions/SIM proxies; requires setting itself as default SMS app to receive inbound; build needs .NET 10 SDK / VS 2026 MAUI. — [LibreSMS site](https://fictus.github.io/LibreSMS/)
- **manjitrana/Messages**: Default SMS/MMS app with embedded REST API on port 8080 (ServerSocket + foreground service), API-key auth, webhooks, MMS send; Android 8.0+. — [GitHub](https://github.com/manjitrana/Messages)
- **pppscn/SmsForwarder**: Very popular Chinese FOSS forwarder (~28,760 stars); monitors SMS, calls, app notifications; forwards to DingTalk/WeCom/Feishu/email/Bark/webhook/Telegram/ServerChan/PushPlus/SMS; v3.0+ remote control (send SMS, query SMS/calls/contacts/battery); v3.x claims Android 4.4–13.0; v3.5.0 released 2026-02-14; package rename to `cn.ppps.forwarder` at 3.5.0. — [GitHub](https://github.com/pppscn/SmsForwarder); [English README](https://github.com/pppscn/smsforwarder/blob/main/README_en.md); [v3.5.0 release](https://github.com/pppscn/SmsForwarder/releases/tag/v3.5.0)
- **ibnux/Android-SMS-Gateway-MQTT**: MQTT-driven send/receive with sent/delivered reports, USSD, multi-SIM, retry×3; Apache-2.0; ~116 stars; author in Indonesia; release activity into 2026 (e.g. v2026-01-30). Companion Firebase-push version: `ibnux/Android-SMS-Gateway` (~129 stars). — [MQTT repo](https://github.com/ibnux/Android-SMS-Gateway-MQTT)
- **CGenie/sms-sender**: Minimal local HTTP gateway (port 8080) for send + webhook registration for inbound. — [GitHub](https://github.com/CGenie/sms-sender)
- **pramay88/OpenSMS**: Self-hosted Node.js+PostgreSQL server; Android apps poll HTTPS; API keys, registration tokens, delivery states; Docker Compose. Architecture is server-mediated (not pure on-device HTTP). — [GitHub](https://github.com/pramay88/OpenSMS)
- **KDE Connect**: FOSS phone↔desktop suite; “Access SMS” — read/send/reply SMS and MMS from computer over LAN with TLS; Android via Play/F-Droid; desktop Linux/Windows. Related: `kdeconnect-cli --send-sms`, Plasma SMS plasmoids, `phone-sms` MCP over D-Bus, `kdeconnect-sms-tui` (author pausing due to RCS). — [kdeconnect-kde README](https://github.com/KDE/kdeconnect-kde); [phone-sms](https://phone-sms.vercel.app/); [kdeconnect-sms-tui](https://github.com/firecat53/kdeconnect-sms-tui)
- **PlainApp**: FOSS local HTTPS web portal on phone (browser on same Wi-Fi); SMS send/read, files, call logs, screen mirror; “No Cloud · No Ads”; compares itself to AirDroid/KDE Connect/Pushbullet. — [plainapp.app](https://plainapp.app/)
- **EasyJoin**: Commercial Pushbullet-like; claims serverless/P2P LAN or internet; SMS/MMS from PC/Mac/Linux; no account/external SMS cloud for message path (per marketing). — [easyjoin.net](https://easyjoin.net/); [Features](https://easyjoin.net/features.html)
- **Telerivet Gateway app**: Free Android app (4.4+); phone+local SIM becomes SMS route into Telerivet cloud platform (campaigns/API/automation). Explicitly marketed for countries without cheap virtual numbers, including India. — [Telerivet app product](https://www.telerivet.com/product/app); [India page](https://www.telerivet.com/product/messaging/country/in)
- **SimGate**: Commercial Android SIM → HTTP API (`api.simgate.app`); free 250 SMS/mo; paid €7.49–€39/mo; **cloud API**, not LAN-local. — [simgate.app](https://www.simgate.app/); [Pricing](https://www.simgate.app/pricing)
- **SMS Forwarder** (`smsforwarder.app`): Receive-only; REST/webhook/MCP/CLI via `api.smsforwarder.app`; free 1 phone; Pro multi-phone; explicitly cloud API; cannot send. Distinct from FOSS `pppscn/SmsForwarder`. — [smsforwarder.app](https://smsforwarder.app/)
- **httpSMS**: Mentioned in SimGate comparison as open-source alternative with free tier (200 msgs/mo, US/Canada on free) and AES-256 with key on phone; cloud-oriented. — [SimGate comparison](https://www.simgate.app/compare/httpsms-alternative)
- **Tasker / Automate patterns**: Tasker HTTP Request event spins a local HTTP server; Received Text SMS → HTTP POST to own server; community Slack SMS gateway projects. Not a packaged product. — [Tasker HTTP Request docs](https://tasker.joaoapps.com/userguide/en/help/eh_http_request.html); [Daniel Ritter blog](https://www.daniel-ritter.de/blog/small-sms-gateway-with-tasker-android-and-a-webserver/); [r/tasker Slack project](https://www.reddit.com/r/tasker/comments/nujpb6/project_share_sms_gateway_for_slack/)
- Older/alt HN-cited projects: `RebekkaMa/android-sms-gateway-server`, `juancrescente/SMSHub`, `niryariv/KalSMS`, Matrix SMS bridge; HN also points at USB 3G/4G dongles as non-phone alternatives. — [HN 28125074](https://news.ycombinator.com/item?id=28125074); [HN 42818337](https://news.ycombinator.com/item?id=42818337)
- F-Droid “SMS Forward” (`com.pierreduchemin.smsforward`) is a simple redirector (last suggested version 1.7.3, 2023-02-09) — not an HTTP API gateway; last update appears stale relative to 2026. — [F-Droid](https://f-droid.org/packages/com.pierreduchemin.smsforward/)

### Inferences
- For “refuse cloud APIs, prefer physical SIM,” the strongest first-class candidates are SMSGate Local mode, LibreSMS, Traccar (direct phone URL), PlainApp, KDE Connect, and DIY Tasker — not SimGate/SMS Forwarder.app/Telerivet hosted platforms (those still use a physical SIM but relay control through vendor cloud).
- SmsForwarder’s star count dwarfs gateway APIs; popularity is driven by OTP-forward-to-chat use cases in CN/Asia, not by REST gateway API design.

### Gaps
- No reliable public star/download counts for LibreSMS, EasyJoin, or PlainApp comparable to GitHub stars.
- “GateSMS” as a distinct named product was not found as a clearly active 2026 project under that exact brand in search results; may be conflated with SMSGate or regional apps.
- Play Store review volumes for SMSGate (sideloaded APK-first) and many FOSS gateways are sparse because distribution is GitHub/F-Droid/APK.

## What features do they typically offer?

### Takeaway
Common feature floor: send SMS over HTTP, optional inbound webhook/forward, basic auth or API key, multi-SIM hooks, and foreground-service keep-alive. Richer stacks add MMS, delivery reports, queues/rate limits, schedules, templates, SMPP (SMSGate), and OTP extraction (commercial forwarders).

### Cited Findings
- SMSGate feature set includes: send/receive SMS API; multipart; status tracking; inbound viewer (SMS/Data SMS/MMS); webhooks; MMS send (base64 attachments) and inbox attachments; batch webhooks; cancel pending; filter by state/type; send rate limiting; `scheduleAt`; E2E encryption (cloud path); private server; multi-SIM; multi-device (cloud/private); Data SMS; working-hours scheduling; manifest receivers for background capture; delivery reports toggle (`withDeliveryReport`, default true); no RCS. — [GitHub README](https://github.com/capcom6/android-sms-gateway/); [FAQ](https://docs.sms-gate.app/faq/general/)
- SMSGate FAQ documents operator-throttling mitigations: random delay between messages, period limits (minute/hour/day), SIM rotation when SIM unspecified, working hours (priority ≥100 bypasses). — [FAQ](https://docs.sms-gate.app/faq/general/)
- LibreSMS: `/sendsms`, `/sendmms`, `/getmessages`, `/status`, `/health`; webhook secret; foreground wake lock; optional boot start; MMS via base64 or image URL. — [LibreSMS](https://fictus.github.io/LibreSMS/)
- textbee hosted Free: 1 device, 50/day, 300/month, webhooks; Pro $14.99/mo: 5 devices, 5,000/mo, templates, scheduled messages, analytics; Scale $29.99/mo: 15 devices, 25,000/mo; self-host “no plan limits.” Sent+received both count. — [Pricing](https://textbee.dev/docs/pricing)
- Traccar SMS Gateway: full messaging UI + HTTP send API; config example uses JSON `{"to","message"}` with Authorization token; can use Traccar.org relay URL or phone-local URL + API key. — [HTTP SMS API](https://www.traccar.org/http-sms-api/)
- SmsForwarder: rule-based forward of SMS/calls/notifications; remote send/query; auto tasks; MQTT/TCP/UDP socket channels (v3.5.0); not primarily a documented OpenAPI REST gateway. — [GitHub](https://github.com/pppscn/SmsForwarder); [v3.5.0](https://github.com/pppscn/SmsForwarder/releases/tag/v3.5.0)
- ibnux MQTT: sent/delivered/received MQTT topics; USSD (accessibility-dependent); multi-SIM (vendor-API dependent); retry failed send 3×. — [ibnux MQTT README](https://github.com/ibnux/Android-SMS-Gateway-MQTT)
- SMS Forwarder.app: OTP detection (multi-language), wait-for-code (≤120s), notification-code capture, dual-SIM receive, receive-only (no send), 7-day free retention / Pro 90-day. — [smsforwarder.app](https://smsforwarder.app/)
- Telerivet: two-way SMS via phone into platform automation; dual-SIM only partially supported (receive both, send primary only; recommend two phones). — [Telerivet FAQ](https://guide.telerivet.com/hc/en-us/articles/360038572452-Frequently-Asked-Questions)
- KDE Connect / EasyJoin / PlainApp emphasize human desktop SMS UI + contacts rather than bulk queue/SMPP. — [KDE Connect](https://github.com/KDE/kdeconnect-kde); [EasyJoin features](https://easyjoin.net/features.html); [PlainApp](https://plainapp.app/)

### Inferences
- “Gateway for apps” and “SMS on desktop like Pushbullet” are overlapping but different products: API gateways emphasize REST/webhooks/queues; companions emphasize chat UI/contacts/MMS conversation sync.
- MMS support is present in modern leaders (SMSGate, LibreSMS, Messages app, KDE Connect) but remains carrier/APN fragile compared to SMS.

### Gaps
- Side-by-side confirmation of contact/template APIs across all FOSS projects was incomplete; SMSGate and textbee document templates/scheduling more clearly than Traccar’s public SMS-gateway page.
- Exact SMPP feature maturity for SMSGate private server was advertised on marketing pages but not fully audited in this pass.

## How do desktops/other apps talk to them?

### Takeaway
Dominant integration pattern is **local HTTP REST on the phone’s LAN IP** (often :8080/:8686) with Basic/API-key auth. Alternatives: self-hosted or vendor HTTPS APIs woken by FCM, MQTT brokers, companion apps over encrypted LAN/P2P, browser-to-phone HTTPS, Tasker local HTTP events, and D-Bus/`kdeconnect-cli` for KDE Connect.

### Cited Findings
- SMSGate Local: `POST http://<device_ip>:8080/message` with Basic auth; Swagger at `/docs`; same payload shape as cloud `https://api.sms-gate.app/3rdparty/v1/messages`; official clients for JS/TS, Python, Go, PHP; CLI `smsgate`. Cloud/Lambda cannot reach Local API without network path. Webhooks fire **from the device** (device needs outbound reachability to webhook URL). — [README](https://github.com/capcom6/android-sms-gateway/); [API docs](https://docs.sms-gate.app/integration/api/); [sms-gate.app](https://sms-gate.app/)
- LibreSMS: GET/POST on LAN `:8686` for send/inbox/status/health; inbound via webhook URL. — [LibreSMS](https://fictus.github.io/LibreSMS/)
- Traccar: phone exposes HTTP API; Traccar server config uses `sms.http.url` either phone URL or `https://www.traccar.org/sms/` relay with token. — [HTTP SMS API](https://www.traccar.org/http-sms-api/)
- textbee / OpenSMS / SimGate: clients talk to a **server** (self-hosted or SaaS); Android agent polls or is pushed (textbee: FCM); not pure offline LAN. — [textbee self-hosting](https://textbee.dev/docs/self-hosting); [OpenSMS](https://github.com/pramay88/OpenSMS); [SimGate](https://www.simgate.app/)
- ibnux: publish commands to MQTT broker; app publishes received/sent/delivered. — [ibnux MQTT](https://github.com/ibnux/Android-SMS-Gateway-MQTT)
- KDE Connect: desktop daemon ↔ phone over local network with TLS; CLI and D-Bus; MCP wrappers talk to daemon, not to a REST SMS API. — [kdeconnect-kde](https://github.com/KDE/kdeconnect-kde); [phone-sms](https://phone-sms.vercel.app/)
- PlainApp: open `https://<phone-ip>:8443` in any browser on same Wi-Fi. — [plainapp.app](https://plainapp.app/)
- EasyJoin: companion desktop apps; LAN preferred; internet via E2E P2P per marketing. — [easyjoin.net](https://easyjoin.net/)
- Tasker: local HTTP server via HTTP Request event (user-chosen port); or outbound HTTP POST on SMS received. — [Tasker docs](https://tasker.joaoapps.com/userguide/en/help/eh_http_request.html)
- HN user (2025) used Capcom gateway + Go intermediary deployed to phone via ADB, rooted to disable battery management. — [HN 45097327](https://news.ycombinator.com/item?id=45097327)
- OpenClaw skill lists SMS Gateway API (`itsmeichigo/SMSGateway`), SMSGate, SmsForwarder, and Capcom as HTTP-on-LAN integrations. — [openclaw skill](https://github.com/openclaw/skills/blob/main/skills/nadjihamid/android-sms-gateway/SKILL.md)

### Inferences
- True air-gapped LAN use almost always means on-device HTTP (or KDE Connect / PlainApp / EasyJoin LAN modes). Anything using FCM (SMSGate cloud, textbee, many commercial apps) needs Google/internet for the control plane even if SMS rides the SIM.
- USB/Bluetooth are uncommon as primary SMS-gateway transports in current projects; USB appears mainly for ADB install/debug or tethering the phone’s Wi-Fi/IP path.

### Gaps
- Little primary documentation found for Bluetooth-as-SMS-gateway among current projects.
- Exact WebSocket usage beyond marketing mentions was not systematically verified for SMSGate private server vs local.

## What Android OS constraints bite them?

### Takeaway
Android KitKat+ SMS architecture, runtime SMS permissions, OEM battery killers, Doze, foreground-service rules, dual-SIM API fragmentation, Play Protect on sideloaded APKs, and lack of public RCS APIs are the recurring hard limits—not the HTTP layer.

### Cited Findings
- Android 4.4+ technical limitation: SMS sent/received via non-default apps remain visible in the default SMS app; SMSGate intentionally does **not** become default SMS app to avoid misuse/transparency issues. LibreSMS **requires** default SMS app to receive inbound. — [SMSGate FAQ](https://docs.sms-gate.app/faq/general/); [LibreSMS](https://fictus.github.io/LibreSMS/)
- SMSGate permissions: SEND_SMS required; READ_PHONE_STATE for SIM select; READ_SMS / RECEIVE_SMS / RECEIVE_MMS / RECEIVE_WAP_PUSH optional for inbox/webhooks. — [README](https://github.com/capcom6/android-sms-gateway/)
- Android 15+: Play Protect may block sideloaded APKs; SMS permission may appear grayed out and need manual grant. — [FAQ](https://docs.sms-gate.app/faq/general/); [Installation](https://docs.sms-gate.app/installation/)
- SMSGate FAQ: Local mode uses foreground service + wake lock; recommends disabling battery optimizations and Don’t Kill My App OEM guides; Cloud mode leans on FCM (possible delays under power saving). — [FAQ](https://docs.sms-gate.app/faq/general/)
- Industry maintenance guides (Telerivet, generic gateway blogs): keep phone plugged in; disable battery optimization / set Unrestricted; Samsung “Put unused apps to sleep”; Xiaomi/HyperOS autostart; Huawei app launch; OnePlus/Oppo/realme optimize toggles. Telerivet explicitly discourages Huawei/Tecno/ZTE/Xiaomi and Android Go for reliability. — [Telerivet maintenance](https://guide.telerivet.com/hc/en-us/articles/360038959491-Maintenance-Instructions); [Telerivet FAQ](https://guide.telerivet.com/hc/en-us/articles/360038572452-Frequently-Asked-Questions); [sms-gateway.app blog 2025](https://sms-gateway.app/blog/android-sms-gateway-app)
- Dual-SIM: SMSGate supports multi-SIM send/rotation; Telerivet receive-both/send-primary only; ibnux warns vendor API differences. — [SMSGate FAQ](https://docs.sms-gate.app/faq/general/); [Telerivet FAQ](https://guide.telerivet.com/hc/en-us/articles/360038572452-Frequently-Asked-Questions); [ibnux](https://github.com/ibnux/Android-SMS-Gateway-MQTT)
- RCS: SMSGate states no RCS support (no public Google API / fragmented OEMs); recommends disabling RCS in Google Messages; kdeconnect-sms-tui author cites RCS migration as reason to pause. — [SMSGate FAQ](https://docs.sms-gate.app/faq/general/); [kdeconnect-sms-tui](https://github.com/firecat53/kdeconnect-sms-tui)
- Carrier/regulatory: sender ID cannot be spoofed via app (uses SIM number); rate limits/spam filters; US 10DLC discussion on HN leads users to prepaid SIM + Capcom gateway on old Pixel. — [SMSGate FAQ](https://docs.sms-gate.app/faq/general/); [HN 45097327](https://news.ycombinator.com/item?id=45097327)
- India-relevant: Telerivet documents India connectivity via virtual number (Vonage) **or** Android phone + local SIM for local rates/low volume; airtime is carrier-billed; Telerivet plans are platform fees ($49/$149/$399 list monthly) plus message-processing capacity when BYO connectivity. — [Telerivet India](https://www.telerivet.com/product/messaging/country/in); [Telerivet pricing](https://www.telerivet.com/pricing); [Telerivet FAQ](https://guide.telerivet.com/hc/en-us/articles/360038572452-Frequently-Asked-Questions)
- SmsForwarder English docs claim Android 4.4–13.0 only for v3.x — newer Android may be unsupported or untested. — [README_en](https://github.com/pppscn/smsforwarder/blob/main/README_en.md)

### Inferences
- Operational reliability is dominated by OEM power management and keeping a dedicated handset on charge on Wi-Fi—not by choice of HTTP library.
- Choosing “become default SMS app” (LibreSMS/Messages) unlocks richer inbound/MMS control but changes user UX and security surface vs companion gateways that refuse default-app status (SMSGate).

### Gaps
- No comprehensive 2026 matrix of dual-SIM SubscriptionManager behavior per OEM was found.
- India-specific TRAI/DLT bulk-SMS registration implications for phone-as-gateway were not found in the sources retrieved this pass (important for commercial Indian use; treat as open question).

## What do users praise and complain about?

### Takeaway
Praise centers on cost (use existing SIM/plan), local number familiarity, and FOSS/local control. Complaints center on background kills, fragile 24/7 phone uptime, OEM quirks, and eventual migration to real CPaaS when reliability matters.

### Cited Findings
- r/androiddev (2025): Capcom gateway recommended for quick client bridge (webhooks + outbound API, on-device); another commenter says phone gateways are fragile (drops, battery kill) and moved to commercial sms-api long-term. — [r/androiddev thread](https://www.reddit.com/r/androiddev/comments/1mpzx7o/whats_the_best_android_sms_gateway_app_for_a_client_project_in_2025/)
- r/selfhosted (2025-01): Capcom project shared positively as simple FOSS developer gateway. — [r/selfhosted](https://www.reddit.com/r/selfhosted/comments/1hwu5bz/android_sms_gateway_for_developers/)
- HN: Traccar SMS Gateway repeatedly recommended for MFA drawer-phone / Antarctica MFA / general HTTP SMS; users also suggest SMSHub, Matrix SMS bridge, hardware GSM modems, Telnyx vs Twilio pricing debates; concerns about carrier ToS/spam and unlimited-plan abuse. — [HN 28125074](https://news.ycombinator.com/item?id=28125074); [HN 33414760](https://news.ycombinator.com/item?id=33414760); [HN 47621825](https://news.ycombinator.com/item?id=47621825); [HN 42818337](https://news.ycombinator.com/item?id=42818337)
- HN (2025): Capcom + cheap Tello unlimited SMS + old Pixel 2 cited as workable US 10DLC workaround; rooting to disable battery mgmt. — [HN 45097327](https://news.ycombinator.com/item?id=45097327)
- kdeconnect-sms-tui documents pain: unclean disconnects leave device “reachable” 10–20 minutes; sends may be queued/lost — also affects official kdeconnect-sms GUI. — [kdeconnect-sms-tui](https://github.com/firecat53/kdeconnect-sms-tui)
- SMS Forwarder.app FAQ admits phone stops forwarding until “Keep running” / OEM autostart; Android 13+ “Restricted setting” for SMS on sideloads. — [smsforwarder.app](https://smsforwarder.app/)
- ibnux issues include MQTT reconnect after reboot, multipart/JSON escaping, RCS questions — signals real-world edge cases. — [ibnux issues](https://github.com/ibnux/Android-SMS-Gateway-MQTT/issues)
- Telerivet: Chinese OEM Android and Android Go called out as unreliable for gateway duty. — [Telerivet FAQ](https://guide.telerivet.com/hc/en-us/articles/360038572452-Frequently-Asked-Questions)

### Inferences
- User satisfaction correlates with **dedicated spare phone** setups; using a daily driver as gateway draws more kill/Doze complaints.
- FOSS Capcom/Traccar win developer mindshare for LAN/API; SmsForwarder wins consumer/OTP-forward mindshare (especially CN tooling integrations).

### Gaps
- Systematic Play Store review mining for Traccar/EasyJoin/Telerivet was not completed this pass.
- Quantitative failure rates (messages lost / day) are anecdotal only.

## Which are truly offline/local vs secretly cloud-relayed?

### Takeaway
Truly local (SMS payload and control can stay on LAN without vendor servers): SMSGate Local mode, LibreSMS, Traccar direct phone API, PlainApp, KDE Connect (LAN), EasyJoin LAN/P2P (per vendor claims), CGenie sms-sender, Tasker local HTTP, ibnux with self-hosted MQTT. Cloud-relayed despite physical SIM: SMSGate Cloud, textbee hosted, SimGate, SMS Forwarder.app, Telerivet, Traccar.org SMS relay URL, ibnux Firebase-push variant, OpenSMS (self-host server still required). Hybrid: SMSGate Private / textbee self-host (your server + usually FCM).

### Cited Findings
- SMSGate docs: Local mode — no internet required for client↔device; Cloud mode requires internet and FCM; Private mode = your infrastructure. Webhooks in local still originate from device. — [sms-gate.app FAQ snippet](https://sms-gate.app/); [Getting Started](https://docs.sms-gate.app/getting-started/)
- LibreSMS marketing: “No middleman… No SIM proxies, no subscriptions, no third-party services”; HTTP on device. — [LibreSMS](https://fictus.github.io/LibreSMS/)
- textbee self-host still depends on **Firebase Cloud Messaging** to wake the phone and hand it messages — control plane is not LAN-only even when API is self-hosted. — [Self-hosting](https://textbee.dev/docs/self-hosting)
- SimGate curl examples hit `https://api.simgate.app/v1/sms/send` — vendor cloud. — [simgate.app](https://www.simgate.app/)
- SMS Forwarder.app APIs are `https://api.smsforwarder.app/...`; messages stored on their servers (7/90 days). — [smsforwarder.app](https://smsforwarder.app/)
- Telerivet: phone forwards between mobile network and Telerivet cloud; Android app free, platform paid. — [Telerivet app](https://www.telerivet.com/product/app)
- Traccar: supports both phone-direct URL and `https://www.traccar.org/sms/` authorization-token path. — [HTTP SMS API](https://www.traccar.org/http-sms-api/)
- KDE Connect / PlainApp / EasyJoin market themselves as no third-party SMS cloud / local or P2P. — [KDE Connect](https://github.com/KDE/kdeconnect-kde); [PlainApp](https://plainapp.app/); [EasyJoin](https://easyjoin.net/)
- SmsForwarder destinations often are third-party chat clouds (Telegram, DingTalk, etc.) even though the Android app is FOSS/self-run — “local phone” + “cloud chat sink.” — [SmsForwarder README](https://github.com/pppscn/SmsForwarder)

### Inferences
- “Uses my SIM” ≠ “offline/local.” Many commercial products are SIM-backed CPaaS substitutes with cloud APIs and plan quotas.
- For Wadhu/desktop integration preferring no vendor cloud: prioritize SMSGate Local, LibreSMS, Traccar phone URL, PlainApp, or KDE Connect; treat textbee self-host as private but FCM-dependent unless further customized.

### Gaps
- EasyJoin’s internet P2P path was not independently audited (NAT traversal / any relay servers) beyond marketing claims.
- Whether SMSGate Private server can fully avoid FCM was not confirmed from docs fetched this pass (custom builds historically integrated Firebase per custom-gateway docs). — [Custom Gateway docs](https://docs.sms-gate.app/getting-started/custom-gateway/)

---

## Quick reference matrix (condensed)

| Project | Local control plane? | SIM send | Inbound | Interface | Pricing signal | Popularity signal |
|---|---|---|---|---|---|---|
| SMSGate Local | Yes (on-device HTTP) | Yes | Webhooks/API | REST :8080 | Free FOSS | ~6k★ |
| SMSGate Cloud/Private | Cloud / self-host | Yes | Yes | REST + FCM | Free FOSS (+ your infra) | Same app |
| LibreSMS | Yes | Yes | Webhook + inbox API | REST :8686 | Free (build) | Newer/smaller |
| Traccar SMS Gateway | Yes (or Traccar.org relay) | Yes | Via messaging app | HTTP API | Free FOSS | ~0.7k★ |
| textbee | Self-host or SaaS; FCM | Yes | Yes | REST/webhooks/MCP | Free–$29.99+; self-host unlimited | ~3.3k★; claims 40k users |
| SmsForwarder | App local; sinks often cloud | Remote send (v3+) | Forward rules | Bots/webhook/MQTT | Free FOSS | ~28.8k★ |
| ibnux MQTT | If you host MQTT | Yes | MQTT/HTTP | MQTT | Free FOSS | ~116★ |
| KDE Connect | LAN TLS | Yes | Yes | Desktop/D-Bus/CLI | Free FOSS | Mature KDE project |
| PlainApp | Local HTTPS portal | Yes | Yes | Browser | Free FOSS | — |
| EasyJoin | LAN/P2P claimed | Yes | Yes | Companion apps | Commercial | — |
| Telerivet | Cloud platform | Yes | Yes | Web/API | App free; plans ~$49+ | Established NGO/biz |
| SimGate | Cloud API | Yes | Webhooks | REST | €0–€39/mo | Commercial |
| SMS Forwarder.app | Cloud API | No (recv only) | Yes + OTP | REST/MCP/CLI | Free / Pro | Commercial |
| Tasker DIY | Local HTTP possible | Yes | Yes | Custom | Tasker paid | Pattern, not product |
