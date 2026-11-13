# Desktop/modem/hardware non-cloud SMS sending solutions

Research date: 2026-11-08. Scope: local SIM / on-prem radio hardware (USB sticks, industrial modules, multi-port GSM gateways, appliances) and the open-source/self-hosted software that drives them—not cloud CPaaS APIs as the radio path.

## What software stacks are standard (Gammu/Wammu, smstools3, gnokii, ModemManager + mmcli, Asterisk chan_mobile, Jasmin, Kannel, playSMS, FreeSWITCH mod_sms)?

### Takeaway
For attached AT-command modems on Linux servers, the durable core stacks are **Gammu SMSD**, **SMS Server Tools 3 (smstools3)**, **Kannel (AT SMSC)**, and **ModemManager/mmcli**; playSMS is a common web/management layer on top. Jasmin’s open-source core is SMPP/HTTP-oriented (serial modem needs a commercial plugin). FreeSWITCH `mod_sms` is SIP SIMPLE chat routing, not cellular modem SMS. Several older phone tools (Wammu, gnokii) are effectively legacy.

### Cited Findings
- Gammu provides CLI + libGammu + **Gammu SMS Daemon** for automated send/receive; the project forked historically from Gnokii. — [Gammu project page](https://wammu.eu/gammu/)
- Gammu released **1.45.0** (2026-09-16) and **1.45.1** (2026-10-19) with AT-modem SMS fixes (Quectel/Alcatel notifications, Huawei USSD, multipart CLI splitting, SMSD delivery-report/multipart recovery in earlier 1.44.x). — [Gammu releases](https://github.com/gammu/gammu/releases); [wammu.eu news](https://wammu.eu/gammu/)
- **Wammu** (GUI) is explicitly “not currently maintained” and does not work with current Python / python-gammu. — [Wammu page](https://wammu.eu/wammu/)
- SMS Server Tools 3 sends/receives via GSM modems/phones using a **spool-directory** model; supports up to 64 GSM devices; AT command set per GSM 07.05 / 03.38; runs on Unix and Windows (Cygwin); current version listed as **3.1.21**, site last updated **2017-05-03**. — [smstools3.kekekasvi.com](https://smstools3.kekekasvi.com/)
- A 2026 sysadmin guide treats SMSTools3 as the simple send-focused daemon and Gammu SMSD as better for receive/two-way with SQL backends; both share ~6–10 SMS/min per channel throughput. — [Big Iron: Gammu vs SMSTools3](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)
- ModemManager exposes SMS create/list/send/store via D-Bus and `mmcli` (`--messaging-create-sms`, `--send`, delivery-report-request, storages sm/me/mt/sr/bm/ta); it is a general modem RIL, not a full SMS gateway product. — [mmcli man page](https://www.freedesktop.org/software/ModemManager/man/latest/mmcli.1.html)
- SMSTools3 Docker docs instruct stopping/disabling ModemManager so it does not claim the same serial port. — [3apaxicom/smstools3 Docker](https://hub.docker.com/r/3apaxicom/smstools3)
- Kannel is a compact open-source WAP/SMS gateway in C; supports SMPP 3.4, UCP/EMI, CIMD, and **GSM phones/modems as a pseudo SMSC**; latest upstream stable listed as **1.4.5**. — [VoIPMatcher Jasmin vs Kannel](https://voipmatcher.com/guides/jasmin-vs-kannel-sms-gateway); [kannel.org news](https://www.kannel.org/news.shtml)
- Debian continues packaging Kannel 1.4.5 with packaging updates through **2025–2026** (e.g. 1.4.5-21 Sep 2025, 1.4.5-22 Nov 2025, 1.4.5-23 Jun 2026) including systemd units and toolchain fixes—upstream feature development appears stagnant while distro maintenance continues. — [Debian kannel changelog](https://tracker.debian.org/media/packages/k/kannel/changelog-1.4.5-23); [Debian kannel news](https://tracker.debian.org/pkg/kannel/news/)
- playSMS is a web SMS management portal supporting gateways including **Gammu, Kannel, SMS Server Tools, Jasmin**, multiple SMSCs, Unicode, long SMS, webservices for send/DLR/credits, inbox/forwarding. — [playsms/playsms README](https://github.com/playsms/playsms); [playsms.org about](https://playsms.org/about/)
- playSMS book documents dual Wavecom-compatible USB modems as two Kannel `smsc = at` SMSCs with HTTP callback into playSMS. — [playSMS Kannel 2-modem example](https://github.com/playsms/book-playsms/blob/master/book-contents/en/Installation/Gateway-Installation/Kannel/Example-Kannel-configuration-for-2-GSM-modems.md)
- Jasmin OSS features SMPP/HTTP client+server, AMQP store-and-forward, routing/failover/billing, Unicode and concatenated SMS; web backend is commercial. — [docs.jasminsms.com](https://docs.jasminsms.com/)
- Jasmin maintainers stated open-source Jasmin only accepts SMPP as upstream; a **commercial multi-protocol plugin** is required for serial modem/HTTP upstreams. — [jookies/jasmin#697](https://github.com/jookies/jasmin/issues/697)
- gnokii is a historic phone toolkit (AT-capable phones, Nokia protocols, CLI SMS); Savannah upstream commits trail off around **2020-01**; Open Hub marks the project **Inactive** with 0 commits in the 12 months ending Mar 2026. — [gnokii Savannah log](https://cgit.git.savannah.nongnu.org/cgit/gnokii.git/log/); [Open Hub gnokii](https://www.openhub.net/p/gnokii)
- Debian/Ubuntu still ship gnokii **0.6.31** with QA packaging updates (2023–2024+), not new upstream features. — [Debian gnokii changelog](https://metadata.ftp-master.debian.org/changelogs/main/g/gnokii/gnokii_0.6.31+dfsg-6_changelog)
- FreeSWITCH **mod_sms** routes SIP SIMPLE / chatplan messages (XMPP-like IM over SIP), not cellular modem SMS; external SMS requires SIMPLE-capable trunks or chatplan/ESL scripting to an API/gateway. — [FreeSWITCH mod_sms docs](https://github.com/signalwire/freeswitch-docs/blob/main/docs/FreeSWITCH-Explained/Modules/mod_sms_6587288.mdx); [freeswitch-users 2015](http://lists.freeswitch.org/pipermail/freeswitch-users/2015-October/116816.html)
- Asterisk **chan_mobile** is a Bluetooth phone channel; community threads in 2024 still discuss Bluetooth voice setup. For USB LTE modules, community forks such as **chan_quectel** (Quectel/SIM7600) add SMS+voice over serial AT/audio ports. — [Asterisk community chan_mobile](https://community.asterisk.org/t/chan-mobile-bluetooth-usage/104042); [wildy/asterisk-chan-quectel](https://github.com/wildy/asterisk-chan-quectel)
- FreeSWITCH historically pointed cellular/modem work at **mod_gsmopen** (voice-oriented); SMS often handled outside FreeSWITCH via gateway AMI/HTTP. — [freeswitch-users 2013/2015 threads](http://lists.freeswitch.org/pipermail/freeswitch-users/2013-January/091486.html)

### Inferences
- **Actively useful in 2025–2026:** Gammu (active upstream releases in 2026), ModemManager (desktop/server Linux standard), Kannel (still packaged and used with playSMS despite frozen upstream), playSMS (still documented as modem-gateway front-end), smstools3 (stable/“frozen but works,” not actively developed since ~2017 site update).
- **Avoid as primary new builds:** Wammu, gnokii (legacy), FreeSWITCH mod_sms for modem SMS (wrong abstraction), Jasmin OSS alone for local USB serial modems (needs commercial plugin or front with Kannel/Gammu/smstools).
- Typical production pattern for self-hosted modem SMS: **AT modem daemon (Kannel AT / Gammu SMSD / smstools3) ± playSMS or custom HTTP wrapper**.

### Gaps
- No single authoritative 2025–2026 “market share” ranking of modem SMS stacks; usage is inferred from docs, packaging activity, and sysadmin guides.
- Asterisk chan_mobile SMS capabilities vs voice-only Bluetooth were not fully documented in primary Asterisk docs in this pass; community practice often offloads SMS to Gammu/smstools or channel drivers like chan_quectel.

## What hardware is commonly used (USB sticks, Wavecom/Sierra modules, multi-port gateways, USB 4G dongles)?

### Takeaway
Hardware falls into four practical classes: (1) consumer USB 3G/4G sticks used in **modem/stick mode**, (2) industrial USB/PCIe modules (Quectel/SIMCom/Sierra), (3) classic **Wavecom/Sierra Fastrack-class** serial GSM modems, and (4) multi-port **Dinstar/OpenVox-class** VoIP/SMS appliances with HTTP/SMPP SMS APIs. Dedicated SMS appliances (SMSEagle, Hypermedia, etc.) sit above the same radio idea with vendor firmware.

### Cited Findings
- 2026 homelab guidance lists Huawei **E3531 / E3372** (stick/NCM switched to serial), **SIM7600**, and **Quectel EC25/EG25-G** as workable permanent modems; warns 2G/3G-only hardware is dead where those networks are shut down. — [Big Iron hardware section](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)
- Huawei **E3372h** ships HiLink (USB Ethernet + web UI at 192.168.8.1); **E3372s** ships stick firmware with AT serial ports. HiLink units can be flashed to stick firmware for Gammu/router PPP/NCM use. — [zaage.IT E3372h flash guide](https://zaage.it/writing/flashing-huawei-e3372h-4g-lte-from-hilink-to-modem-mode/); [pixelspark/tymodem](https://github.com/pixelspark/tymodem)
- HiLink E3372 can also expose AT via **debug mode** (HTTP API), creating `/dev/ttyUSB*` without permanent flash; debug mode is **not persistent across reboot** unless automated. — [GNUton E3372 blog](https://www.gnuton.org/blog/huawei-e3372/); [JJTRONICS Jeedom E3372 tutorial 2025](https://www.jjtronics.com/wordpress/2025/03/21/%f0%9f%9a%80-utiliser-un-dongle-4g-huawei-e3372-hilink-pour-la-domotique-sous-jeedom-envois-sms/)
- Wavecom Fastrack M1306B (later Sierra Wireless lineage) is a classic RS232/USB AT GSM modem; SMS uses standard `AT+CMGF`, `AT+CMGS`, `AT+CNMI`, `AT+CMGR` with ~20-message SIM buffer limits documented in field guides. — [Sierra Wireless forum](https://forum.sierrawireless.com/t/sending-an-sms-m1306b-gsm-modem/2249); [Wavecom AT guide (Ozeki copy)](https://ozeki.hu/attachments/588/wavecom_command.pdf); [puntoflotante Wavecom SMS tests](https://www.puntoflotante.net/CONFIGURACION%20INICIAL%20Y%20PRUEBAS%20DE%20MODEM%20WAVECOM%20M1306B.pdf)
- playSMS/Kannel examples treat **Wavecom-compatible** USB modems as the default AT modem type (`modemtype = wavecom`). — [playSMS Kannel 2-modem config](https://github.com/playsms/book-playsms/blob/master/book-contents/en/Installation/Gateway-Installation/Kannel/Example-Kannel-configuration-for-2-GSM-modems.md)
- Older CentOS SMS server writeups used Huawei USB modems and Falcom hardware with Kannel + playSMS. — [visibilityspots SMS server blog](https://visibilityspots.github.io/blog/sms-server.html)
- **Dinstar** UC2000-VE/VF/VG: 4/8/16/32 GSM/LTE channels with voice+SMS+USSD, **HTTP SMS API** and **SMPP**, optional multi-SIM (4 SIMs per channel, SIM rotation) up to 128 SIMs on 32 channels. — [Dinstar UC2000-VG](https://www.dinstar.com/GSM-3G-LTE-voip-gateway/32-ports/); [Dinstar Multi-SIM](https://www.dinstar.com/GSM-3G-LTE-voip-gateway/multi-sim-gateway/); [Dinstar brochure PDF](https://zen.ae/docs/Dinstar.pdf)
- **OpenVox** wireless gateways (VS-GW2120 historically up to 44 GSM channels; current SWG/GWP lines) advertise SMS send/receive, bulk SMS, SMS-to-email, **HTTP SMS interface**, USSD, SIP/IAX, Asterisk AMI. VS-GW2120 series is listed under EOL products. — [OpenVox VS-GW2120](https://www.openvoxtech.com/products/eolproduct/59-vs-gw2120-gsm-series); [OpenVox wireless gateways](https://www.openvoxtech.com/products/voip-gateways/wireless-voip-gateways/)
- 2026 appliance comparison lists dedicated hardware SMS gateways: SMSEagle (~1000 EUR+, 30–240 SMS/min), Hypermedia (~2100 EUR+), Dinstar (~690 EUR+), Skyline multi-channel bulk units, plus Teltonika routers with basic SMS. — [SMSEagle Best SMS Gateway Solutions 2026](https://www.smseagle.eu/2026/08/07/best-sms-gateway-solutions-in-2026-comparison-ranking-and-buying-guide/)
- SIM7600E Raspberry Pi HAT deployments with Asterisk need substantial USB current (up to ~2.1A cited) or the Pi may brown out. — [wildy/asterisk-chan-quectel](https://github.com/wildy/asterisk-chan-quectel)

### Inferences
- “USB 4G dongle in modem mode” is the dominant cheap DIY path; HiLink/router firmware is the main trap.
- Multi-port gateways are preferred by call centers / bulk SMS / PBX mobile-trunk users who want HTTP/SMPP without hand-rolling AT daemons.
- Industrial modules (Quectel/SIMCom) are displacing classic 2G Wavecom boxes in new builds where LTE is required.

### Gaps
- Exact current MSRP/SKU availability for Wavecom Fastrack equivalents varies by region; many units in the field are used/refurbished.
- Vendor throughput claims (e.g. Skyline hundreds–thousands SMS/min) should be treated as marketing until validated against operator rate limits—primary independent measurements were not found in this pass.

## Feature sets: queues, DLR, USSD, multi-SIM failover, concatenation, Unicode, receive inbox?

### Takeaway
File/SQL queues and receive inboxes are strong in Gammu/smstools3/Kannel/playSMS. Unicode and concatenation are widely supported at the protocol/stack layer. Native multi-SIM failover and USSD are strongest on appliance gateways (Dinstar/OpenVox) and weaker/ad-hoc on single USB sticks. DLR quality depends on modem + stack + whether the operator returns status reports.

### Cited Findings
- smstools3: spool queues (`outgoing`/`sent`/`failed`/`incoming`), Unicode/binary messages, WAP Push, event/alarm handlers, multi-device HA up to 64 modems. — [smstools3 site](https://smstools3.kekekasvi.com/)
- Gammu SMSD: files or SQL (MySQL/PgSQL/SQLite) outbox/inbox; deliveryreport config; RunOnReceive hooks; recent releases improved DLR handling, multipart grouping, weekday scheduling for SQL outbox, USSD on some Huawei models. — [Big Iron guide](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting); [Gammu 1.45/1.44 release notes](https://github.com/gammu/gammu/releases)
- ModemManager `mmcli` can request delivery reports at SMS create time and auto-split long UTF-8 text into multiple parts. — [mmcli man page](https://www.freedesktop.org/software/ModemManager/man/latest/mmcli.1.html)
- Kannel AT SMSC configs in playSMS docs enable `concatenation = true`, `sim-buffering`, validity period, dual SMSC for two SIMs/modems, and delivery-report checkboxes in playSMS’s Kannel plugin. — [playSMS Kannel config](https://github.com/playsms/book-playsms/blob/master/book-contents/en/Configuration/Gateway-Configuration/Kannel-for-playSMS.md); [2-modem example](https://github.com/playsms/book-playsms/blob/master/book-contents/en/Installation/Gateway-Installation/Kannel/Example-Kannel-configuration-for-2-GSM-modems.md)
- playSMS: Unicode, flash SMS, long SMS, inbox + email forward, webservices for send and DLR retrieval, multi-SMSC routing by prefix/user. — [playsms features](https://github.com/playsms/playsms)
- Jasmin (when used over SMPP/HTTP, not local serial in OSS): AMQP queueing, DLR via Redis/lookup paths, Unicode/UTF-16, concatenated SMS, failover routing. — [docs.jasminsms.com](https://docs.jasminsms.com/)
- Dinstar gateways advertise SMS send/receive, USSD, HTTP API, SMPP, SMS-to-email/email-to-SMS, PIN management, multi-SIM rotation by runtime/balance. — [Dinstar UC2000-VG](https://www.dinstar.com/GSM-3G-LTE-voip-gateway/32-ports/); [Dinstar Multi-SIM](https://www.dinstar.com/GSM-3G-LTE-voip-gateway/multi-sim-gateway/)
- OpenVox gateways advertise SMSC/SMS/USSD, bulk SMS, auto-resend, SMS-to-email, HTTP SMS interface, SMS forwarding. — [OpenVox VS-GW2120](https://www.openvoxtech.com/products/eolproduct/59-vs-gw2120-gsm-series)
- UCS-2 Unicode cuts GSM SMS payload from 160 to **70** characters; multipart reassembly may be incomplete if apps treat raw spool files as one logical message. — [Big Iron gotchas](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)
- Wavecom field docs: modem SMS storage limited (example: 20 messages); full buffer blocks new MO SMS until deleted with `AT+CMGD`. — [puntoflotante Wavecom guide](https://www.puntoflotante.net/CONFIGURACION%20INICIAL%20Y%20PRUEBAS%20DE%20MODEM%20WAVECOM%20M1306B.pdf)

### Inferences
- “Multi-SIM failover” on DIY USB stacks usually means **multiple modem instances + application routing**, not one stick with intelligent SIM switch—unless using multi-SIM appliance hardware.
- DLR from consumer SIMs is often “submitted to network” rather than true handset delivery confirmation; stacks can request it, operators may not return useful reports.

### Gaps
- Side-by-side matrix of USSD support across Gammu vs smstools3 vs Kannel AT was not fully primary-sourced; Gammu has modem-specific USSD enablement notes, appliances document USSD explicitly.
- Exact DLR reliability rates by operator/modem combination require field measurement; not available as general statistics here.

## Operational pain: drivers/mode switching, AT quirks, SIM lock, rate limits, thermal, Windows vs Linux?

### Takeaway
The recurring failure modes are **HiLink vs modem mode**, **ModemManager fighting AT daemons**, unstable **ttyUSB numbering**, SIM PIN/carrier locks, operator anti-spam throttling of machine SMS, USB power/thermal brownouts, and 2G/3G sunset. Linux is the well-trodden path; Windows exists (smstools3 via Cygwin, Gammu Windows builds) but most gateway documentation assumes Linux.

### Cited Findings
- HiLink presents as USB-Ethernet/RNDIS with web UI; stick/modem mode presents serial AT ports. Mixing them up is a primary setup failure. — [zaage.IT](https://zaage.it/writing/flashing-huawei-e3372h-4g-lte-from-hilink-to-modem-mode/); [GNUton](https://www.gnuton.org/blog/huawei-e3372/)
- Debug-mode AT ports on HiLink E3372 require re-enable after reboot; systemd oneshot services are used in home-automation setups. — [JJTRONICS 2025](https://www.jjtronics.com/wordpress/2025/03/21/%f0%9f%9a%80-utiliser-un-dongle-4g-huawei-e3372-hilink-pour-la-domotique-sous-jeedom-envois-sms/)
- Common ops failures: ttyUSB renumbering after reboot (fix with udev symlinks); PIN-locked SIMs; running two daemons on one port; ModemManager claiming the device; 2G/3G sunset; carrier filtering of bursty identical machine SMS; underpowered USB hubs causing reset storms. — [Big Iron gotchas](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)
- Gammu release notes repeatedly cite AT quirks: CNMI compatibility, Quectel/Alcatel notifications, Huawei devices, ZTE unsolicited messages, bare SMS prompts, CPMS parameter variants. — [Gammu releases](https://github.com/gammu/gammu/releases)
- Huawei AT interfaces also expose vendor lock commands (`AT^CARDLOCK`, `AT^DATALOCK`) in debug/hacking notes. — [gnuton gist](https://gist.github.com/gnuton/3ba6dca3ce27d13833da)
- Modem-pool vs Android comparisons cite modem-pool tradeoffs: **drivers, USB reliability, RF engineering**. — [sms-gateway.app comparison](https://sms-gateway.app/comparisons/gsm-modem-pool-vs-android-sms-gateway)
- SMSEagle “build vs buy” notes self-built SMS servers shift maintenance/on-call to the team; hardware gateways add vendor support and built-in failover. — [SMSEagle build or buy 2026](https://www.smseagle.eu/2026/11/06/sms-server-build-or-buy/)
- smstools3 officially supports Windows via Cygwin and Unix/Linux; Gammu publishes Windows installer builds (recently bundling native MySQL). — [smstools3](https://smstools3.kekekasvi.com/); [Gammu 1.45.0 notes](https://github.com/gammu/gammu/releases)
- Single-channel DIY throughput cited ~**6–10 messages/minute**; higher volume needs more radios or commercial interconnects. — [Big Iron comparison table](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)

### Inferences
- Production DIY setups almost always need: udev stable names, ModemManager ignore/mask rules, PIN disabled or configured, external antenna for rack/metal enclosures, and rate limiting in software to avoid SIM blacklisting.
- Appliances reduce AT/driver pain but introduce vendor firmware, SIM bank complexity, and still face operator anti-fraud limits.

### Gaps
- Quantitative thermal failure rates / MTBF for USB stick farms were not found in reliable sources.
- Windows-specific driver stories for modern LTE modules are under-documented relative to Linux in the sources reviewed.

## How do business apps integrate (HTTP API in front of modem, database outbox, CLI)?

### Takeaway
Three integration patterns dominate: (1) **file spool / CLI** (smstools3, gammu CLI), (2) **SQL outbox** (gammu-smsd), (3) **HTTP/SMPP API** in front of the radio—either via Kannel/playSMS/Jasmin or appliance HTTP SMS APIs. Apps rarely speak raw AT in production.

### Cited Findings
- smstools3 integration: drop text files in `/var/spool/sms/outgoing`; eventhandler scripts push to DB/email/webhooks. — [smstools3](https://smstools3.kekekasvi.com/); [Big Iron walkthrough](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)
- Gammu: CLI send; SMSD SQL insert for outbox; RunOnReceive for inbound routing (e.g. OTP to Home Assistant). — [Big Iron Gammu section](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting); [wammu.eu](https://wammu.eu/)
- Kannel exposes HTTP sendsms interface (example port 13131) and can callback playSMS on MO; apps call Kannel HTTP or use playSMS webservices. — [visibilityspots blog](https://visibilityspots.github.io/blog/sms-server.html); [playSMS Kannel docs](https://github.com/playsms/book-playsms/blob/master/book-contents/en/Configuration/Gateway-Configuration/Kannel-for-playSMS.md)
- playSMS provides web UI + webservices for sending, DLR retrieval, credits; routes by SMSC/prefix. — [playsms features](https://github.com/playsms/playsms)
- Dinstar/OpenVox expose **HTTP SMS API** (and often SMPP) so PBX/CRM apps integrate without AT. — [Dinstar UC2000-VG](https://www.dinstar.com/GSM-3G-LTE-voip-gateway/32-ports/); [OpenVox VS-GW2120](https://www.openvoxtech.com/products/eolproduct/59-vs-gw2120-gsm-series)
- Jasmin provides HTTP + SMPP + REST for apps, but local modem attachment is not the OSS default path. — [docs.jasminsms.com](https://docs.jasminsms.com/); [jasmin#697](https://github.com/jookies/jasmin/issues/697)
- ModemManager offers scripting via `mmcli` / D-Bus for light automation, not a full multi-tenant gateway. — [mmcli man page](https://www.freedesktop.org/software/ModemManager/man/latest/mmcli.1.html)
- Asterisk dialplans can shell out to SMS tools or use channel drivers that inject SMS into extensions (`System()`, chan_quectel SMS → email scripts). — [wildy/asterisk-chan-quectel](https://github.com/wildy/asterisk-chan-quectel)

### Inferences
- For business apps in 2026, preferred on-prem shape is **HTTP API (appliance or Kannel/playSMS) → modem pool**, with SQL/file spool as the low-level layer.
- Homelab/ops alerting often stays at spool/CLI; product OTP platforms lean toward HTTP + multi-modem appliances or Android fleets.

### Gaps
- No universal OpenAPI standard across modem gateways; each vendor/stack HTTP schema differs.
- Concrete examples of Jasmin commercial serial-modem plugin APIs were not publicly detailed beyond the GitHub issue statement.

## Cost/ops comparison vs keeping an Android phone as the radio?

### Takeaway
A cheap Android phone + prepaid SIM is lower CAPEX and faster to start, but weaker for 24/7 critical OTP/alerts (OEM battery kills, no durable queue, power/OS churn). Dedicated modems/appliances cost more up front and need RF/driver ops, but are designed for continuous duty, buffering, and multi-SIM density. Both still pay operator airtime and face the same consent/carrier fair-use constraints.

### Cited Findings
- Android gateway vendor comparison: phone path = rapid setup, medium volume, flexibility; hardware GSM modem = 24/7 design, message buffer during interruptions, auto-restart after power loss, better for OTP/alarms. — [SMSAlert Sync vs hardware modem](https://smsalert.mobi/en/sms-gateway-android)
- DIY Android example: ~$20 phone + prepaid SIM (~$8/mo unlimited plan cited), REST API local or cloud relay; must disable battery optimization, enable start-on-boot, monitor with ping webhooks. — [jonno.nz Android SMS gateway](https://jonno.nz/posts/built-an-sms-gateway-with-a-20-dollar-android-phone/)
- Modem pool strengths: dense ports, industrial racks; tradeoffs: drivers, USB reliability, RF engineering. Android strengths: own MSISDN, operator airtime economics, device inventory; ops burden: power, OEM limits, fleet health. — [sms-gateway.app modem pool vs Android](https://sms-gateway.app/comparisons/gsm-modem-pool-vs-android-sms-gateway)
- SMSEagle 2026: self-built SMS server = low hardware cost, high team time; bought gateway = higher CAPEX, vendor support, built-in multi-modem failover, listed throughputs 30–240 SMS/min depending on model. — [SMSEagle build or buy](https://www.smseagle.eu/2026/11/06/sms-server-build-or-buy/); [SMSEagle 2026 ranking](https://www.smseagle.eu/2026/08/07/best-sms-gateway-solutions-in-2026-comparison-ranking-and-buying-guide/)
- Homelab 2026 guidance: DIY modem justified mainly for **out-of-band alerts when WAN is down**; for two-way app SMS at scale, CPaaS is usually better; DIY throughput ~6–10/min/channel. — [Big Iron](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)
- Indicative appliance list prices (Aug 2026): SMSEagle from ~1000 EUR; Dinstar from ~690 EUR; Hypermedia from ~2100 EUR; low-end multi-channel units from ~240 EUR—plus SIMs/airtime. — [SMSEagle 2026 ranking](https://www.smseagle.eu/2026/08/07/best-sms-gateway-solutions-in-2026-comparison-ranking-and-buying-guide/)

### Inferences
- **Choose Android radio** for low/medium volume, non-critical, fast PoC, or when you need a “real phone number experience” cheaply.
- **Choose USB modem + Gammu/smstools/Kannel** for out-of-band ops alerts and small OTP volumes with Linux ops skill.
- **Choose multi-port appliance** when you need many concurrent SIMs, HTTP/SMPP for business apps, and paid support.
- Hybrid designs (Android for domestic volume + modem/appliance for critical path) are commonly recommended by vendors.

### Gaps
- True TCO depends heavily on local prepaid SMS rates and whether consumer SIMs allow commercial use—country-specific; no global average compiled here.
- Vendor pages comparing Android vs modem are commercially interested; treat qualitative reliability claims as directional, not independent lab results.

## Regulatory caveats (brief, global)

### Takeaway
Using your own SIM/modem does **not** erase consent, opt-out, or carrier anti-spam rules. US A2P 10DLC registration is mainly a CPaaS/long-code interconnect program, but TCPA/CTIA-style consent and filtering still apply to business-originated texts; many consumer plans forbid bulk commercial SMS. EU GDPR and local telecom rules also apply.

### Cited Findings
- CTIA Messaging Principles distinguish P2P vs A2P; A2P should follow consumer consent (express for informational; express written for marketing) and revocation; providers may filter unwanted traffic. — [CTIA Messaging Principles PDF](https://api.ctia.org/docs/default-source/default-document-library/170119-ctia-messaging-principles-and-best-practices.pdf)
- Guides for SIM/Android gateways note A2P 10DLC registration may not apply to own-SIM paths, but TCPA consent/opt-out and carrier suspension for spam still do; consumer SIMs often limit commercial bulk use. — [VoidFix SMS without A2P](https://gateway.voidfix.com/sms-without-a2p-registration); [VoidFix compliance guide](https://gateway.voidfix.com/blogs/sms-compliance-best-practices)
- Twilio’s A2P 10DLC help center frames US long-code business messaging registration for platform senders; carrier ecosystem treats app-originated traffic as A2P regardless of some legal P2P framings. — [Twilio A2P 10DLC](https://help.twilio.com/articles/4408675845019-SMS-Compliance-and-A2P-10DLC-in-the-US)
- Hardware gateway sellers note local licensing, consent, opt-out, and traffic monitoring can block otherwise functional modem farms. — [Telarvo SMPP gateway blog](https://blog.telarvostore.com/4-port-smpp-sms-gateway-enterprise-messaging-hardware-for-authorized-bulk-sms/)

### Inferences
- On-prem modem SMS is best positioned for **transactional/ops/low-volume authenticated messaging** under a business SIM plan—not stealth bulk marketing.
- Operators can still rate-limit or terminate SIMs that look like unauthorized A2P blasts.

### Gaps
- Jurisdiction-by-jurisdiction licensing (e.g. who may operate GSM gateways / SIM banks) was not exhaustively surveyed; treat local telecom counsel as required for multi-SIM commercial use.

## Maintenance snapshot (2025–2026)

| Stack | Modem-local SMS? | Maintenance signal |
| --- | --- | --- |
| Gammu / gammu-smsd | Yes (primary) | Active upstream releases through Oct 2026 |
| Wammu | GUI only | Explicitly unmaintained |
| smstools3 | Yes (primary) | Mature; last site update 2017; still widely packaged/used |
| ModemManager + mmcli | Yes (basic) | Actively maintained Linux infrastructure project |
| gnokii | Yes (legacy) | Upstream inactive (~2020); distro packaging only |
| Kannel | Yes (AT SMSC) | Upstream 1.4.5 frozen; Debian packaging active 2025–2026 |
| playSMS | Via gateways | Actively documented; depends on backend gateway |
| Jasmin OSS | Not serial (needs commercial plugin) | Active docs/product; modem via commercial add-on |
| Asterisk chan_mobile | Phone BT, not USB LTE | Niche; community still asks setup questions |
| Asterisk chan_quectel (community) | Yes (Quectel/SIMCom) | Community forks; not core Asterisk |
| FreeSWITCH mod_sms | No (SIP SIMPLE) | Wrong tool for cellular modem SMS |
| Dinstar/OpenVox/SMSEagle | Appliance HTTP/SMPP | Commercial vendor products; some older OpenVox GSM series EOL |

## Who uses these solutions (from sources)

### Cited Findings
- Homelab / SRE: out-of-band monitoring alerts and OTP capture into automation. — [Big Iron](https://www.bigiron.cc/guides/sms-self-hosted-gammu-vs-smstools3-for-otp-and-alerting)
- Domotics (e.g. Jeedom) after 2G sunset, using LTE sticks for SMS alerts. — [JJTRONICS 2025](https://www.jjtronics.com/wordpress/2025/03/21/%f0%9f%9a%80-utiliser-un-dongle-4g-huawei-e3372-hilink-pour-la-domotique-sous-jeedom-envois-sms/)
- Nonprofits/events and small orgs: Kannel + playSMS quiz/comms portals with Huawei/Wavecom modems. — [visibilityspots](https://visibilityspots.github.io/blog/sms-server.html); [playsms about](https://playsms.org/about/)
- Enterprises/call centers/rural landline replacement: Dinstar/OpenVox multi-port GSM/LTE gateways for voice+SMS. — [Dinstar UC2000-VG](https://www.dinstar.com/GSM-3G-LTE-voip-gateway/32-ports/); [OpenVox wireless gateways](https://www.openvoxtech.com/products/voip-gateways/wireless-voip-gateways/)
- Critical IT/BMS/healthcare/gov alerting buyers of dedicated appliances (SMSEagle positioning). — [SMSEagle 2026 ranking](https://www.smseagle.eu/2026/08/07/best-sms-gateway-solutions-in-2026-comparison-ranking-and-buying-guide/)
