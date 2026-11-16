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
5. Allowed usage / App Review:
   [`ALLOWED_USAGE_CHECKLIST.md`](ALLOWED_USAGE_CHECKLIST.md),
   [`APP_REVIEW_ANSWERS.md`](APP_REVIEW_ANSWERS.md),
   [`DATA_HANDLING_AND_REVIEWER.md`](DATA_HANDLING_AND_REVIEWER.md)

## Docs in this folder

| File | Role |
| --- | --- |
| `INTEGRATOR.md` | Brand-app integration |
| `DEPLOY.md` | Hosting connect.wadhu.com |
| `DATA_HANDLING_AND_REVIEWER.md` | Meta form paste |
| `APP_REVIEW_*.md` | Permission text + video notes |
| `ALLOWED_USAGE_CHECKLIST.md` | What’s left before Next |
| `PRIVACY.md` / `TERMS.md` | Pointers → `docs/legal/` |
