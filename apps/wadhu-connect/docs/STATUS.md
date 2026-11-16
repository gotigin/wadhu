# Wadhu Connect — status

| | |
| --- | --- |
| Product | **Wadhu Connect** (under brand **Wadhu**) |
| Folder | `apps/wadhu-connect` only |
| Company | GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED |
| Trademark | Gotigin |
| Brand site | https://wadhu.com |
| Privacy | https://wadhu.com/privacy ← `docs/legal/PRIVACY.md` |
| Data deletion | https://wadhu.com/data-deletion |
| Connect host | https://connect.wadhu.com |
| Local | `cargo wadhu-connect-dev` → `:3847` |
| Naming | [`docs/BRAND.md`](../../../docs/BRAND.md) |

### Operator contacts (do not mix)

| Role | Value |
| --- | --- |
| Personal email | mearajbhagad@gmail.com |
| Business email | mearaj@gotigin.com |
| Personal mobile | +91 99677 17702 |
| **Business mobile (WABA / Cloud API)** | **+91 75740 67702** |

There is **no** `apps/gotigin-connect`. Meta display name must be **Wadhu Connect**.

## Meta checklist (operator)

1. App settings → Basic: display name **Wadhu Connect**; Site URL → `https://wadhu.com`
2. Privacy → `https://wadhu.com/privacy`; data deletion → `https://wadhu.com/data-deletion`
3. Webhook → `https://connect.wadhu.com/webhook` (verify token from `.env` / Secret Manager)
4. **Facebook Login for Business → Settings** (no ngrok):
   - Valid OAuth Redirect URIs → `https://connect.wadhu.com/onboard`
   - Deauthorize callback URL → `https://connect.wadhu.com/deauthorize`
   - Data Deletion Request URL → `https://wadhu.com/data-deletion`
   - Login with JavaScript SDK → Yes; Allowed Domains → `connect.wadhu.com`
5. Cloud Run: `DATA_DIR=/data` on `gs://wadhu-ac8dc-connect-data` (survives redeploy).
   Current WABA `1592532032562448`, phone `1436067426258883`, Page `1233972929807232`.
   Default ES is Cloud API (not coexistence). Coexistence: `?coexistence=1` after Tech Provider clears #3441107.
6. Allowed usage / App Review:
   [`ALLOWED_USAGE_CHECKLIST.md`](ALLOWED_USAGE_CHECKLIST.md),
   [`APP_REVIEW_ANSWERS.md`](APP_REVIEW_ANSWERS.md),
   [`DATA_HANDLING_AND_REVIEWER.md`](DATA_HANDLING_AND_REVIEWER.md)
7. **Submitted** App Review (Meta download 2026-11-16 12:17 PM PST):
   [`assets/app-review/Meta_App_Review_Submitted_2026-11-16.pdf`](../assets/app-review/Meta_App_Review_Submitted_2026-11-16.pdf)  
   Included: `email`, `pages_show_list`, `whatsapp_business_messaging`,
   `whatsapp_business_management`, `public_profile`.  
   **Not** included: `business_management`, `pages_read_engagement` — optional
   for WhatsApp Cloud API / not used by Connect runtime; Testing counters
   stayed at 0 despite Graph 200s. After this review **passes**, we will try
   to integrate those two in a follow-up submission. Detail:
   [`APP_REVIEW_ANSWERS.md`](APP_REVIEW_ANSWERS.md) § Deferred.

## Docs in this folder

| File | Role |
| --- | --- |
| `INTEGRATOR.md` | Brand-app integration |
| `DEPLOY.md` | Hosting connect.wadhu.com |
| `DATA_HANDLING_AND_REVIEWER.md` | Meta form paste |
| `APP_REVIEW_*.md` | Permission text + video notes |
| `ALLOWED_USAGE_CHECKLIST.md` | What’s left before Next |
| `PRIVACY.md` / `TERMS.md` | Pointers → `docs/legal/` |
