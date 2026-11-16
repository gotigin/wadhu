# Meta form paste — Wadhu Connect

Single source for Data handling + Reviewer instructions.

| Item | Value |
| --- | --- |
| Meta app display name | **Wadhu Connect** |
| Privacy source file | `docs/legal/PRIVACY.md` |
| App ID | `1606021637403895` |
| Site URL | `https://wadhu.com` |
| Connect / ES UI | `https://connect.wadhu.com` |
| Privacy policy URL | `https://wadhu.com/privacy` |
| Data deletion instructions URL | `https://wadhu.com/data-deletion` |
| Terms | `https://wadhu.com/terms` |
| Support | `https://wadhu.com/support` |
| Business contact | `mearaj@gotigin.com` |
| Controller | GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED · India |
| Test phone | `+1 555-156-0957` · ID `1408752645660951` · WABA `1324845626382929` |
| Allowlisted recipient | `919967717702` |

---

## Data handling

| Field | Answer |
| --- | --- |
| Data processors / service providers? | **No** |
| Controller (responsible entity) | **GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED** |
| Country | **India** |
| National security disclosures (12 months)? | **No** |
| Policies for authority requests | **None of the above** |

---

## Reviewer instructions

| Field | Answer |
| --- | --- |
| Site URL | `https://wadhu.com` |
| Facebook Login integrated? | **Yes** (Login for Business / Embedded Signup) |
| Access / gift / geo | *(leave empty)* |

### Site URL

`https://wadhu.com/`

### fblogin-web-1

**Yes** (Facebook Login for Business / Embedded Signup)

### accesscode-web-1

```
No payment or membership required. Free to open and test at https://connect.wadhu.com/onboard. Use a Facebook account that is a developer/tester/admin on this app (or a Facebook Test User with a role).
```

### accesscode-web-2

```
N/A — web app, not distributed via app stores.
```

### geo-web-5

```
No geo-blocking. Reviewers worldwide can open https://connect.wadhu.com/onboard.
```

### documents-web-1 (optional)

Upload from `apps/wadhu-connect/assets/app-review/`:
`whatsapp_business_messaging.mp4`, `whatsapp_business_management.mp4`,
`pages_show_list.mp4` (same as Allowed usage). Do **not** upload
`business_management.mp4` / `pages_read_engagement.mp4` for this submission.

### Paste for instructions-web-2

```
Brand site: https://wadhu.com
Connect UI: https://connect.wadhu.com/onboard
Privacy: https://wadhu.com/privacy
Data deletion: https://wadhu.com/data-deletion
Terms: https://wadhu.com/terms
Support: https://wadhu.com/support
Contact: mearaj@gotigin.com

Wadhu Connect (GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED) is a WhatsApp Cloud API Tech Provider connector. Businesses grant access to their WhatsApp Business Account via Facebook Login for Business / Embedded Signup. Permissions in this review: whatsapp_business_messaging, whatsapp_business_management, public_profile, pages_show_list, email. We omitted business_management and pages_read_engagement for this round (not required for Cloud API Tech Provider; not used by Connect runtime today). After approval we will try to integrate those two. We do not use consumer Facebook Login APIs (user_friends, user_birthday, etc.).

How to test:
1. Open https://connect.wadhu.com/onboard
2. Click “Launch Embedded Signup”.
3. Complete Facebook Login for Business; select the existing Cloud API business number (+91 75740 67702). Do not use the personal WhatsApp Business app number for this review.
4. Click “Refresh status” and confirm WABA / phone IDs.
5. Send a test message to allowlisted recipient 919967717702 (or another app-role tester).
6. Confirm delivery on WhatsApp.

Own-business Cloud API: +91 75740 67702 (Phone Number ID 1436067426258883), WABA 1592532032562448. Screencasts attached under Allowed usage / documents.
```
