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

Until `wadhu.com` / `connect.wadhu.com` DNS are live, local review may use a
tunnel pointed at `cargo wadhu-connect-dev` — replace with production URLs
before submit.

---

## Data handling

| Field | Answer |
| --- | --- |
| Data processors / service providers? | No |
| Controller | GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED |
| Country | India |
| National security disclosures (12 months)? | No |
| Policies for authority requests | None of the above |

---

## Reviewer instructions

| Field | Answer |
| --- | --- |
| Site URL | `https://wadhu.com` |
| Facebook Login integrated? | **Yes** (Login for Business / Embedded Signup) |
| Access / gift / geo | *(leave empty)* |

### Paste for instructions-web-2

```
Brand site: https://wadhu.com
Connect product UI: https://connect.wadhu.com
Privacy: https://wadhu.com/privacy
Data deletion: https://wadhu.com/data-deletion
Terms: https://wadhu.com/terms
Support: https://wadhu.com/support
Contact: mearaj@gotigin.com

Wadhu Connect (GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED) is a Tech Provider
connector: businesses grant access to their WhatsApp Business Account via
Facebook Login for Business / Embedded Signup. Permissions under review:
whatsapp_business_messaging, whatsapp_business_management, public_profile.
We do not use consumer Facebook Login APIs (user_friends, user_birthday, etc.).

How to test:
1. Open https://connect.wadhu.com/
2. Click “Launch Embedded Signup (coexistence)”.
3. Complete Facebook Login for Business; keep an existing WhatsApp Business app number if prompted.
4. Click “Refresh status” and confirm WABA / phone IDs.
5. Send a test message to allowlisted recipient 919967717702.
6. Confirm delivery on WhatsApp.

Test Cloud API number: +1 555-156-0957 (1408752645660951), WABA 1324845626382929.
Screencasts of messaging and management flows are attached.
```
