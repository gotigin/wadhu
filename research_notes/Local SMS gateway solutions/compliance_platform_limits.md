# Local SMS gateway: compliance and platform limits (product-design notes)

**Scope.** Dense design constraints for a non-cloud local SMS gateway (Android handset SIM + optional desktop modem) aimed at shops and third-party apps (e.g. a Wadhu SMS-Gateway). Covers Android platform rules, carrier A2P/P2P practice, India TRAI TCCCPR/DND, high-level EU/US consent expectations, risk-mitigating product features, and operational safety.

**Disclaimer.** This is **not legal advice**. Laws, Play policies, and carrier practices change; national implementations of EU ePrivacy differ; India TCCCPR was amended in 2025–2026. Treat every jurisdiction and use case as requiring local counsel before shipping commercial messaging features. Uncertainty is marked below.

**As-of.** Sources checked for 2025–2026 where available (TRAI Feb 2025 regulation text / 2025–2026 press materials; Google Play SMS/Call Log policy pages; FCC TCPA consent-revocation materials through Apr 2025; CTIA Messaging Principles 2023).

---

## Android: SEND_SMS, default SMS role, background limits, Play policy, sideloading

### Takeaway
Technically any app with `SEND_SMS` can send via `SmsManager`, but Play distribution effectively requires default-SMS (or listed exception) status; the default SMS role also removes Android’s per-app outgoing SMS rate gate—so a gateway product’s distribution path and “become default SMS app” UX are first-class design decisions, not afterthoughts.

### Cited Findings
- `SmsManager.sendTextMessage` requires `Manifest.permission.SEND_SMS`. Beginning with Android 4.4 (API 19), if the app is **not** the default SMS app, the system automatically writes sent messages to the SMS Provider; the default SMS app is responsible for writing its own sent messages. — [Android SmsManager API](https://developer.android.com/reference/android/telephony/SmsManager)
- Only the default SMS app can write to the SMS Provider and receive `SMS_DELIVER_ACTION` / `WAP_PUSH_DELIVER_ACTION` for incoming SMS/MMS. — [Android Telephony](https://developer.android.com/reference/android/provider/Telephony)
- For Play-distributed apps that need SMS/Call Log permissions, Google Play generally requires the app to be the user’s default SMS, Phone, or Assistant handler (unless an exception applies). The app must ask to become the default handler **before** requesting associated permissions (e.g. request default SMS before `READ_SMS`). Default handlers must actually perform that function, publish a privacy policy, and document core functionality in the listing. — [Permissions used only in default handlers](https://developer.android.com/guide/topics/permissions/default-handlers); [Play: Use of SMS or Call Log permission groups](https://support.google.com/googleplay/android-developer/answer/10208820)
- Play’s sensitive-permissions policy states SMS permission group members (`READ_SMS`, `SEND_SMS`, `WRITE_SMS`, `RECEIVE_SMS`, `RECEIVE_WAP_PUSH`, `RECEIVE_MMS`) require active registration as default SMS or Assistant handler; apps lacking that capability may not declare these permissions in the manifest (including placeholders); they must stop using the permission when they lose default-handler status; SMS/Call Log data must not be used for advertising/marketing or other unapproved purposes. — [Play: Permissions and APIs that Access Sensitive Information](https://support.google.com/googleplay/android-developer/answer/16558241?hl=en)
- Play lists **exceptions** (temporary, declaration-form based) when permission enables listed core functionality and no alternative exists—including **device automation** and **enterprise archive / business & enterprise CRM / enterprise device management** (corporate login required; CRM has further permission marks). Exceptions still must comply with spyware / user-data policies; legacy exception APKs for older OS versions face install-base percentage limits. — [Play: Use of SMS or Call Log permission groups](https://support.google.com/googleplay/android-developer/answer/10208820)
- AOSP `SmsUsageMonitor` defaults: `DEFAULT_SMS_MAX_COUNT = 30`, `DEFAULT_SMS_CHECK_PERIOD = 60000` ms (1 minute), overridable via `Settings.Global.SMS_OUTGOING_CHECK_MAX_COUNT` / `SMS_OUTGOING_CHECK_INTERVAL_MS`. Comment and logic: **if the app holds `RoleManager.ROLE_SMS`, there is no send limit**; otherwise `check()` enforces the limit. — [AOSP SmsUsageMonitor.java (main)](https://android.googlesource.com/platform/frameworks/opt/telephony/+/refs/heads/main/src/java/com/android/internal/telephony/SmsUsageMonitor.java)
- SmsManager docs note a historical “per-application based SMS control” tied to `sentIntent`: if `sentIntent` is null the caller is checked against unknown applications and may be allowed a smaller number of SMS in the checking period. — [Android SmsManager API](https://developer.android.com/reference/android/telephony/SmsManager); older AOSP SmsManager source commentary — [SmsManager.java (googlesource)](https://android.googlesource.com/platform/frameworks/opt/telephony/+/idea133/src/java/android/telephony/SmsManager.java)
- Dual-SIM: use `SmsManager.createForSubscriptionId(subId)` (API 31+) or `getSmsManagerForSubscriptionId`; `getDefault()` can show a subscription disambiguation dialog, and background sends without a chosen default can fail or pick unpredictably (behavior depends on target SDK). Prefer binding to an explicit subscription ID. — [Android SmsManager API](https://developer.android.com/reference/android/telephony/SmsManager); [SubscriptionManager](https://developer.android.com/reference/android/telephony/SubscriptionManager)
- Sideloading / Play Protect: apps installed from internet sideloading sources (browser, messaging, file manager) that declare sensitive permissions including `RECEIVE_SMS` / `READ_SMS` are treated as high-risk and may be **automatically blocked** by Play Protect enhanced fraud protection; Play Protect guidance says SMS permissions for OTP verification alone are disallowed (use SMS Retriever / User Consent APIs). — [Play Protect warning developer guidance](https://developers.google.com/android/play-protect/warning-dev-guidance); enterprise commentary that managed Google Play / EMM DPC paths avoid enhanced fraud protection blocks that hit raw APK sideloads — [Bayton: enhanced fraud protection](https://bayton.org/android/android-enterprise-faq/enhanced-fraud-protection-enterprise/)
- April 2025 Play policy announcement clarified SMS/Call Log guidance especially around financial apps and SMS history—showing policy text remains actively maintained. — [Play policy announcement Apr 10, 2025](https://support.google.com/googleplay/android-developer/answer/15899442)

### Inferences
- For a Wadhu Android companion that must send reliably at shop volumes, **becoming the default SMS app** is the practical path: (1) Play policy for `SEND_SMS`, (2) exemption from AOSP’s 30/min-style gate, (3) control of inbox/outbox for delivery receipts and STOP keyword handling. Product UX must include a first-run “set as default SMS app” flow and a full messaging UI (Play requires the handler to actually handle SMS).
- If the product cannot honestly be a full SMS client, design for **sideload / private distribution** (shop-owned devices) or pursue a Play **exception** (device automation / enterprise CRM)—both are uncertain and operationally heavier; sideloading still risks Play Protect blocks when `READ_SMS`/`RECEIVE_SMS` are declared.
- OEM/carrier forks may alter `SmsUsageMonitor` defaults; design product-level rate limits below the strictest expected OS gate and never rely on “unlimited because default SMS.”
- Background gateway services should always use an explicit `subscriptionId`, never `getDefault()`, to avoid silent failure on dual-SIM shop phones.

### Gaps
- No authoritative public matrix of OEM deviations from AOSP’s 30 SMS / 60s defaults was found; Stack Overflow anecdotes conflict with AOSP’s 1-minute period (often claim “30 per 30 minutes”).
- Whether a pure “SMS gateway / modem bridge” qualifies for Play’s device-automation or enterprise-CRM exceptions is **uncertain**—depends on Play Console declaration review; no public approval of a similar gateway was sourced.
- Exact Play Protect blocklist signals beyond declared permissions (signature/similarity) are opaque; community reports of false positives for MDM agents exist but are secondary sources.

---

## Carriers: consumer SIM throttling, A2P vs P2P, SIM blocking risk when automating

### Takeaway
Carriers and industry guidelines treat **automated business messaging as Non-Consumer (A2P)** even when it originates from a handset long-code / consumer SIM; traffic that looks like high-volume, one-way, repetitive “P2P” is filtered or numbers/SIMs are blocked—so a local gateway must behave like a carefully rate-limited transactional sender, not a bulk blaster on a prepaid SIM.

### Cited Findings
- Twilio (industry explainer): **A2P** is any traffic where a person receives messages from an application (marketing, reminders, chatbots, OTPs); **P2P** is human↔human conversational messaging. US/Canada carriers treat messaging through application platforms as A2P; local regulations (e.g. India, France) can further restrict delivery. — [Twilio: What is P2P and A2P messaging?](https://help.twilio.com/articles/223133807-What-is-P2P-and-A2P-messaging-)
- CTIA Messaging Principles (2023): **Consumer (P2P)** vs **Non-Consumer (A2P)**; Non-Consumer includes messaging to/from businesses and **all messaging traffic that is automated in whole or in part** (except narrow Consumer automation such as assisting a consumer’s own messaging). Consumer traffic inconsistent with typical Consumer operation may be filtered under Unwanted Messaging mitigation. — [CTIA Messaging Principles and Best Practices (May 2023 PDF)](https://api.ctia.org/wp-content/uploads/2023/05/230523-CTIA-Messaging-Principles-and-Best-Practices-FINAL.pdf)
- CTIA Exhibit I “typical Consumer operation” attributes (illustrative, not exclusive): throughput **15–60 messages/minute**; volume **~1,000/day**; one telephone number per consumer; ≤**100** distinct recipients per message; roughly **1:1** in/out balance; **≤25** repetitive messages. Traffic from a Non-Consumer remains A2P even if it stays under those thresholds. — [CTIA 2023 PDF](https://api.ctia.org/wp-content/uploads/2023/05/230523-CTIA-Messaging-Principles-and-Best-Practices-FINAL.pdf); same table in [CTIA 2019 PDF](https://api.ctia.org/wp-content/uploads/2019/07/190719-CTIA-Messaging-Principles-and-Best-Practices-FINAL.pdf)
- CTIA: Service Providers may block/filter Unwanted Messages, suspend exchange, or disconnect senders to protect networks; consumers get blocking/reporting tools. — [CTIA 2023 PDF](https://api.ctia.org/wp-content/uploads/2023/05/230523-CTIA-Messaging-Principles-and-Best-Practices-FINAL.pdf)
- Aggregator best-practice example (Inteliquent): consumer P2P velocity caps aligned with CTIA-like numbers (60/min, 1000/24h, 1:1 balance, 25 repetitive); A2P uses registered 10DLC/toll-free paths; **blocklisting** of numbers sending repeated spam/unwanted content (up to 30 days or permanent); content **fingerprinting** of spam that does not age out. — [Inteliquent Messaging Best Practice Guidelines (2022)](https://wwwcontent.inteliquent.com/wp-content/uploads/2022/09/21182307/Inteliquent-Messaging-Best-Practice-Guidelines-2022-09-21.pdf)
- Platform ops note: carriers—not the SaaS—decide delivery; high STOP/spam rates lead to throttling or permanent number blocks that are hard to reverse. — [HighLevel: Phone Messaging Quality Monitoring](https://help.gohighlevel.com/support/solutions/articles/155000008745-lc-phone-messaging-quality-monitoring)

### Inferences
- A shop gateway sending order-ready / COD / invoice SMS from a consumer SIM will often **look like A2P** (one-way, template-repetitive, app-initiated). Design as if every carrier may treat it as unwanted P2P abuse unless volume, cadence, and consent are conservative.
- Product defaults should sit **well below** CTIA’s illustrative 1,000/day and 15–60/min consumer envelope (e.g. tens to low hundreds/day for a single SIM), with configurable hard caps—especially for prepaid SIMs.
- Dual-SIM “rotate SIMs to evade limits” is an anti-pattern that matches SIM-farm / unwanted-messaging detection narratives (see India TRAI SIM-farm detection below); product should **discourage** multi-SIM load balancing for volume.
- Desktop USB modems / GSM sticks face the same carrier classification problem; hardware does not convert A2P into P2P.

### Gaps
- **India-specific consumer-SIM daily SMS caps** (prepaid vs postpaid, Jio/Airtel/Vi plan terms) were not reliably sourced as a single official table; they vary by plan and change often—treat as per-carrier TOS research at ship time.
- No primary carrier document was found that states “handset automation from Android SmsManager is always blocked”; enforcement is opaque and complaint-/fingerprint-driven.
- Exact US 10DLC registration applicability to pure handset SIMs (vs cloud long codes) is industry-practice territory; handset paths generally lack a registration program and therefore lack A2P “safe harbor” throughput.

---

## India: TRAI TCCCPR / DND — own handset SIM vs registered telemarketer gateway

### Takeaway
Under TCCCPR (as amended), **purpose—not SIM type—drives regulation**: commercial communication from an unregistered sender (including a business’s ordinary handset SIM) is treated as **Unsolicited Commercial Communication (UCC)**; registered commercial traffic must use registered headers/special number series, templates, and DLT scrubbing—so a local handset gateway is a poor fit for promotional SMS in India and is legally fragile even for “transactional” shop messages unless counsel confirms the traffic is non-commercial or a registered path is used.

### Cited Findings
- Amended definition: any commercial communication by a Sender **not registered** with any Access Provider for sending commercial communication **shall be treated as Unsolicited Commercial Communication**. — [TRAI TCCCPR amendment regulation PDF (12 Feb 2025)](https://www.trai.gov.in/sites/default/files/2025-02/Regulation_12022025.pdf)
- Regulation 3 substitute: Access Providers must ensure commercial communication uses **registered headers** or **special series number resources**; **No Sender not registered** shall make commercial communication; unregistered commercial communication may lead to suspension/disconnection of telecom resources. — [same TRAI PDF](https://www.trai.gov.in/sites/default/files/2025-02/Regulation_12022025.pdf)
- Auto dialer / robo-calls: every Sender shall **notify the Originating Access Provider in advance** about Auto Dialer or Robo-Calls and intended objective. — [same TRAI PDF](https://www.trai.gov.in/sites/default/files/2025-02/Regulation_12022025.pdf)
- Message categories (2025 clarifying amendments / annex language):
  - **Transactional**: sender→its customer/subscriber in response to **customer-initiated transaction within 30 minutes** (OTP, transaction alerts/confirmations, balance alerts post-transaction, refund info, etc.), not promotional, no Explicit Consent required.
  - **Service**: non-promotional info to existing customers (warranty, recall, software upgrade, safety/security, periodic balance, delivery of goods/services) on inferred consent; or facilitating an ongoing purchase after Explicit Consent (Explicit Consent validity limited to **seven days** or as directed).
  - **Promotional**: commercial ad/upsell; mixing promo into transactional/service → entire message treated as promotional; promotional messages need Preference Register eligibility and/or Consent Register / explicit digital consent; promotional messages must include an **opt-out mechanism** as specified by the Authority.
  — [TRAI PDF](https://www.trai.gov.in/sites/default/files/2025-02/Regulation_12022025.pdf); [BSE annex quoting definitions](https://www.bseindia.com/Downloads/UploadDocs/Notices/Attach/Annexure$432a1954-9f9b-4abd-bc3e-fc22e2f2e858.pdf); TSP FAQ restatements — [Airtel DLT FAQ](https://dltconnect.airtel.in/faq/), [BSNL DLT FAQ](https://www.ucc-bsnl.co.in/faq/)
- Headers: TRAI press materials describe standardized header suffixes **-P / -S / -T / -G** (promotional / service / transactional / government) to help consumers identify message type (notation in one press release used “-F” wording—verify against final implementation docs). — [TRAI PR No. 11/2025 PDF](https://trai.gov.in/sites/default/files/2025-02/PR_No.11of2025.pdf)
- Enforcement against unregistered senders / UTMs: blacklisting, barring of telecom resources including **PRI/SIP trunks, SIMs**, device blocking language in amendments; Access Providers directed to detect **SIM Farm/SIM box** type usage, robotic/auto-dialer patterns; honeypots for spam logging. — [TRAI PDF](https://www.trai.gov.in/sites/default/files/2025-02/Regulation_12022025.pdf)
- Secondary legal/tech explainers state the regulations look at **purpose, not SIM kind**—sales/commercial calls from personal SIMs still carry sender duties; undeclared automated calling is high risk. — [Telodial TRAI DND summary](https://telodial.com/blog/trai-dnd-rules-for-telecallers-2026/) (**secondary**; verify against regulation text)
- PIB/DoT materials: subscribers opt via DND (1909 / TRAI DND App); UCC complaints feed TSP action under TCCCPR; Sanchar Saathi / Chakshu for fraud/UCC reporting. — [PIB release](https://www.pib.gov.in/PressReleasePage.aspx?PRID=2080641)
- Designated series (calls focus, but signals commercial-path architecture): **1600** for certain service/transaction calls (BFSI regulated entities / government); **140** for promotional calls requiring TCCCPR registration—illustrates that India is pushing commercial traffic onto **registered special series**, not ordinary mobile numbers. — [PIB PRID 2283309](https://www.pib.gov.in/PressReleasePage.aspx?PRID=2283309&lang=1&reg=48)

### Inferences
- **Product positioning for India:** Prefer integrating with **registered DLT / PE + header + template** gateways for any promotional or scaled commercial SMS. Position the local SIM gateway as (a) purely person-operated fallback, (b) strictly non-commercial personal messaging, or (c) narrowly scoped service/transactional alerts **only after Indian counsel confirms** the use case—and still expect complaint-driven SIM risk if content looks promotional.
- UX should force message-class tagging (transactional / service / promotional) and **block promotional sends** on unregistered handset mode with a clear “use registered telemarketer / DLT” path.
- Automating sends from the phone (gateway daemon) may resemble “Auto Dialer”-class automation for voice; for SMS there is no equally crisp public “declare automation” SMS clause found, but SIM-farm / bulk-pattern detection still applies—keep rates human-like and consent-backed.
- Opt-out keywords and quiet hours help consumer goodwill but **do not replace** TCCCPR registration for commercial traffic.

### Gaps
- Authoritative clarification of whether a **one-to-one transactional SMS** (e.g. “Order #123 ready for pickup”) sent from a kirana shop’s ordinary mobile SIM without DLT registration is lawful remains **uncertain**—regulation text sweeps “commercial communication” broadly and treats unregistered commercial senders as UCC, while transactional/service definitions assume a “Sender” in the regulated framework. **Needs Indian telecom counsel.**
- Mapping of TCCCPR “Auto Dialer” advance-notice duty to **SMS automation** (vs voice) was not found in the excerpts reviewed.
- Header suffix letter inconsistency (“-F” vs “-P”) across press materials—confirm against current TSP DLT docs before documenting in product UI.

---

## EU / US high-level: consent expectations and TCPA-like risk when automating from a phone

### Takeaway
Automating SMS from a phone does **not** escape electronic-marketing / autodialer consent regimes: EU ePrivacy generally requires prior consent for unsolicited SMS marketing (with a narrow existing-customer soft opt-in in many states); US TCPA/FCC rules impose prior express (written) consent for marketing robotexts, mandate easy revocation (STOP etc.), and create private-action risk—product must assume “app-sent = regulated automated send.”

### Cited Findings
- **EU ePrivacy Directive 2002/58/EC** summary: user consent required before unsolicited communications (spam), **including SMS** and other electronic messaging. — [EUR-Lex summary](https://eur-lex.europa.eu/EN/legal-content/summary/data-protection-in-the-electronic-communications-sector.html); full act reference Directive 2002/58/EC
- Industry/legal explainers (secondary): GDPR governs personal-data lawful basis; **ePrivacy Art. 13** governs the send itself for marketing email/SMS; transactional/service messages (order updates, reminders, OTPs) typically do not need marketing consent, but mixing promo into a service message can turn it into marketing; Art. 13(2) soft opt-in for similar products to existing customers with opt-out at collection and in each message—**national implementations vary**. — [Bird GDPR/ePrivacy explainer](https://bird.com/explained/compliance/what-does-gdpr-require-for-email-and-sms-marketing); [Arino SMS consent overview](https://arino.one/resources/sms-marketing-consent-gdpr); [gdprregister.eu direct marketing article](https://www.gdprregister.eu/articles/direct-marketing-rules-and-exceptions/)
- **US TCPA / FCC:** Prior express written consent definition for telemarketing ads via autodialer/prerecorded voice (agreement in writing, signature, clear disclosures, not a condition of purchase). — [Federal Register text restoring PEWC definition after court vacatur, Aug 2025](https://www.federalregister.gov/documents/full_text/text/2025/08/29/2025-16641.txt); earlier one-to-one consent rule materials — [Federal Register HTML Nov 2024](https://www.federalregister.gov/documents/full_text/html/2024/11/06/2024-24908.html); FCC FAQ PDF — [DOC-408396A1](https://docs.fcc.gov/public/attachments/DOC-408396A1.pdf) (**note:** one-to-one rule was litigated/vacated—status must be re-checked at implementation time)
- Consumer Action small-business guide (secondary): automated marketing texts to cell phones need prior express written consent; quiet hours for marketing calls to residences 8 a.m.–9 p.m. recipient local time; honor National DNC / state DNC; keep consent records; provide opt-out; revoke “in any reasonable manner.” — [Consumer Action TCPA guide](https://www.consumer-action.org/english/articles/TCPA_business/)
- FCC consent-revocation regime (effective ~Apr 11, 2025 for most pieces): consumers may revoke by **any reasonable means**; keywords such as **stop, quit, end, revoke, opt out, cancel, unsubscribe** are per se reasonable; callers may not designate an exclusive revocation channel; honor within **10 business days**; one confirmation text within **five minutes**, no marketing; limited waiver delayed until **Apr 11, 2026** only the piece requiring a marketing-revocation to also kill unrelated informational streams. — [FCC DA-25-312 PDF](https://docs.fcc.gov/public/attachments/DA-25-312A1.pdf); [Nixon Peabody summary](https://www.nixonpeabody.com/insights/alerts/2025/04/11/fcc-partially-delays-new-tcpa-consent-revocation-rules); [Krieg DeVault summary](https://www.kriegdevault.com/insights/fcc-delays-compliance-date-for-part-of-new-tcpa-consent-revocation-rule)

### Inferences
- For EU-facing shops: default product mode = **transactional/service templates only**; marketing SMS requires explicit opt-in flags and per-country policy packs; always include opt-out.
- For US-facing use: treat gateway automation as creating **autodialer/robotext risk**; require stored prior express consent for marketing; require STOP handling even for transactional if replies are possible; do not claim “it’s just my phone” as a defense in UX copy.
- One-to-one consent / PEWC rule churn (2024–2025 litigation) means the product should store **seller-specific consent artifacts** and avoid “lead-gen consent shared across brands” patterns.
- Quiet hours: implement recipient-local quiet hours as a **global safety default** even where only US residential call rules are hard-coded in statute—reduces complaint risk everywhere.

### Gaps
- Whether a specific Android `SmsManager` send qualifies as an “automatic telephone dialing system” under post-*Facebook v. Duguid* TCPA case law is **fact-specific and unsettled**—flag for US counsel; product should still assume risk when sending lists automatically.
- Member-state PECR/ePrivacy differences (B2B SMS, soft opt-in scope) were not surveyed country-by-country.
- Desktop modem AT-command sending has the same consent duties; no special exemption found.

---

## Product features that mitigate risk (consent, caps, templates, audit, opt-out)

### Takeaway
Safety features will not legalize unregistered commercial bulk SMS, but they are the minimum control plane for a local gateway: per-recipient consent and message-class gates, quiet hours, hard daily/hourly caps, locked templates, immutable audit logs, and inbound STOP/DND handling—plus India-specific blocks when DLT registration is absent.

### Cited Findings
- India: promotional messages must include Authority-specified opt-out; promo mixed into transactional/service → promotional; transactional window **30 minutes** from customer-initiated transaction; service explicit-consent validity **7 days**; unregistered commercial sender → UCC. — [TRAI PDF](https://www.trai.gov.in/sites/default/files/2025-02/Regulation_12022025.pdf)
- US: honor STOP and listed keywords; confirmation text rules; 10-business-day outer bound; no exclusive opt-out channel. — [FCC DA-25-312](https://docs.fcc.gov/public/attachments/DA-25-312A1.pdf); secondary consolidations — [Krieg DeVault](https://www.kriegdevault.com/insights/fcc-delays-compliance-date-for-part-of-new-tcpa-consent-revocation-rule)
- EU: prior consent for unsolicited SMS marketing; soft opt-in conditions; right to object to direct marketing (GDPR Art. 21 as commonly summarized). — [EUR-Lex ePrivacy summary](https://eur-lex.europa.eu/EN/legal-content/summary/data-protection-in-the-electronic-communications-sector.html); [Bird explainer](https://bird.com/explained/compliance/what-does-gdpr-require-for-email-and-sms-marketing)
- CTIA / aggregator practice: limit velocity, avoid repetitive spam fingerprints, maintain conversational balance where claiming P2P, expect blocklisting for repeated unwanted content. — [CTIA 2023](https://api.ctia.org/wp-content/uploads/2023/05/230523-CTIA-Messaging-Principles-and-Best-Practices-FINAL.pdf); [Inteliquent guidelines](https://wwwcontent.inteliquent.com/wp-content/uploads/2022/09/21182307/Inteliquent-Messaging-Best-Practice-Guidelines-2022-09-21.pdf)
- Android default-SMS role needed for reliable inbound intercept of STOP replies at the device. — [Android Telephony](https://developer.android.com/reference/android/provider/Telephony); [default handlers](https://developer.android.com/guide/topics/permissions/default-handlers)

### Inferences — recommended control plane for Wadhu SMS-Gateway
| Feature | Design implication |
| --- | --- |
| Per-recipient consent flags | Store `consent_type` (none / transactional_inferred / express_marketing), timestamp, source (POS checkout, form, verbal+logged), and selling entity; API refuses marketing without express flag. |
| Message class | Enum `transactional \| service \| promotional`; promotional disabled in “unregistered handset” India profile; warn when template contains promo language. |
| Quiet hours | Default e.g. 21:00–08:00 recipient-local; queue instead of send; configurable per shop. |
| Daily / hourly caps | Defaults far under CTIA 1000/day illustrative consumer volume; separate caps per SIM; burst spacing (e.g. ≥2–5 s) to avoid OS and carrier spikes. |
| Content templates | Versioned templates with fixed+variable slots (mirrors DLT template mindset); ban URL shorteners / opaque links in defaults if spam filters fingerprint them. |
| Audit logs | Immutable send log: timestamp, subscriptionId/MSISDN, recipient hash or number, template id, class, consent id, app client id, result code; retention policy for complaints. |
| Opt-out keywords | Inbound listener for STOP/QUIT/END/CANCEL/UNSUBSCRIBE/OPT OUT/REVOKE (+ localized words); suppress further automated sends; optional one confirmation; sync suppressions to desktop gateway API. |
| Client API contracts | Third-party apps must pass message class + consent reference; gateway enforces policy—not the client. |
| Human-in-the-loop | Optional “approve batch” for >N recipients; reduces autodialer-like optics and bulk mistakes. |
| Jurisdiction packs | `IN`, `EU`, `US`, `OTHER` policy presets toggling promotional allow, DLT warnings, quiet hours, keyword sets. |

### Gaps
- No single global standard keyword list beyond FCC’s English set; localization needs per-market research.
- “Audit log retention duration” is jurisdiction-specific (not sourced here).

---

## Operational safety: dual-SIM selection, roaming, prepaid balance / USSD

### Takeaway
Operational UX must pin **which SIM** sends, refuse or warn on roaming, and treat prepaid balance as a best-effort, carrier-specific USSD integration—not a universal API—while avoiding multi-SIM abuse patterns that look like SIM farms.

### Cited Findings
- Dual-SIM sending: bind `SmsManager` to a chosen `subscriptionId` via `createForSubscriptionId` / `getSmsManagerForSubscriptionId`; `getDefault()` may show disambiguation UI or fail in background if no default SMS subscription is set (target-SDK-dependent). — [Android SmsManager](https://developer.android.com/reference/android/telephony/SmsManager)
- USSD programmatic request: `TelephonyManager.sendUssdRequest(ussd, UssdResponseCallback, Handler)` (API 26+); use `createForSubscriptionId` for the correct SIM; typically needs `CALL_PHONE`; balance codes are **carrier-specific** (examples in community code `*100#`, `*556#`, etc.—not universal). — [Android TelephonyManager](https://developer.android.com/reference/android/telephony/TelephonyManager); community patterns — [Stack Overflow USSD](https://stackoverflow.com/questions/5477597/how-is-it-possible-to-do-ussd-requests-on-android)
- India enforcement signals explicitly call out **SIM Farm / SIM box** detection among UTM suspicion methods. — [TRAI PDF](https://www.trai.gov.in/sites/default/files/2025-02/Regulation_12022025.pdf)
- Play Protect / sideload risk for SMS-permission apps on non-Play distribution paths. — [Play Protect guidance](https://developers.google.com/android/play-protect/warning-dev-guidance)

### Inferences
- **SIM picker UX:** On first launch, list `SubscriptionInfo` (carrier name, slot, ICCID last digits); require explicit “send on SIM 1 (Airtel …)” choice; persist `subscriptionId`; show MSISDN if available; never silently fail over to the other SIM.
- **Roaming:** Use `TelephonyManager`/`ServiceState` roaming indicators to **pause automated sends** by default (cost + foreign spam rules + unexpected A2P filtering). Allow override with typed confirmation.
- **Prepaid balance:** Offer a pluggable “balance check” per carrier profile (USSD code + regex parse); on parse failure, degrade to “last known balance unknown—sends may fail”; never block the send pipeline solely on flaky USSD.
- **Modem path:** For desktop GSM modems, expose the same policy engine (caps, consent, quiet hours); AT+CUSD for balance is likewise carrier-specific.
- **Device posture:** Prefer dedicated shop handset left on Wi-Fi/power; document that factory-reset / losing default-SMS role stops the gateway; warn that multi-phone SIM farms violate carrier/TRAI expectations.

### Gaps
- No comprehensive public table of Indian (or global) prepaid USSD balance codes and response formats suitable to ship as defaults—must be maintained as community/carrier profiles.
- Roaming SMS commercial-message rules per visited country were not researched in depth.
- Reliability of `sendUssdRequest` across OEMs (some show UI, some fail in background) is incompletely documented in primary sources reviewed.

---

## Cross-cutting design implications for Wadhu SMS-Gateway

1. **Distribution:** Plan for (A) default-SMS Android app on Play **or** (B) private/MDM distribution to owned devices; do not assume sideload APK + `SEND_SMS` is frictionless in 2025–2026.
2. **Policy engine at the gateway:** Third-party shop apps must not be able to bypass consent class, caps, quiet hours, or STOP list.
3. **India:** Default to “registered commercial path required for promo”; local SIM mode is high-risk for marketing and legally ambiguous for automated commercial service texts—surface warnings, don’t silently enable bulk.
4. **US/EU:** Separate transactional templates from marketing; store consent; implement STOP; quiet hours on by default.
5. **Carrier survival:** Low defaults, template diversity without spamminess, no SIM rotation for volume, monitor failure/SEND errors as early block signals.
6. **Always label product docs:** “Not a substitute for carrier A2P / DLT registration or legal advice.”
