# Firebase Auth setup (wadhu.com registration)

Project: **`wadhu-ac8dc`** (Blaze + Identity Platform advanced features).  
Goal: cover as many users as possible for **register / sign-in**.  
Secrets for federated IdPs live only under gitignored [`local/wadhu-auth/`](../local/wadhu-auth/) (never commit).

OAuth redirect (custom domain): `https://wadhu.com/__/auth/handler`  
Authorized domains include: `wadhu.com`, `www.wadhu.com`, `connect.wadhu.com`, `localhost`.

## Enabled now

| Method | Status |
| --- | --- |
| Email / password | On |
| Google | On |
| Phone | On (SMS needs Blaze; regions allow-by-default) |
| GitHub | On (Gotigin org OAuth app) |
| Microsoft | On (Entra app “Wadhu”; client secret ~6‑month expiry — rotate before then) |
| SMS MFA | Optional / not required for basic registration (Identity Platform) |

## Apple Sign in — postponed

**Still in the plan** (needed for Apple-ID users and App Store expectations later). **Not blocked forever** — postponed until membership is ready.

### Issue we hit (2026-11-18)

1. Sign-in to [Apple Developer](https://developer.apple.com/account) succeeded (account showed as Mearaj Bhagad).
2. **Certificates, Identifiers & Profiles** loaded as a **blank page** (title only, no sidebar / list / “+”).
3. Account overview looked like a **free** developer account (tools/docs), not an active paid team with Identifiers/Keys.

### Cause

Creating Firebase **Sign in with Apple** requires an active **Apple Developer Program** membership (~USD 99/year) so we can create:

- App ID (Sign in with Apple capability)
- **Services ID** (Firebase “Client ID”) + domain `wadhu.com` + return URL `https://wadhu.com/__/auth/handler`
- **Key** (.p8) with Sign in with Apple → Team ID + Key ID + private key for Firebase

Without that membership (or access to a team that has it), the Identifiers UI stays empty/broken and Apple cannot be wired into Firebase.

### Solution (when we resume)

1. Enroll (prefer **organization: Gotigin** with D‑U‑N‑S / company domain) or join an existing Gotigin Apple team:  
   https://developer.apple.com/programs/enroll/
2. Or sign in with the Apple ID that already owns the paid Gotigin team.
3. Then create App ID → Services ID → Key; enable Apple in Firebase Auth with Services ID, Team ID, Key ID, and `.p8` private key; store secrets under `local/wadhu-auth/apple-signin.env` (gitignored).

Until then: registration goes ahead with Email, Google, Phone, GitHub, and Microsoft.

## Site UI (`/account`)

- Page: [`sites/wadhu/static/account/index.html`](../sites/wadhu/static/account/index.html)
- Client: `sites/wadhu/client/account-app.js` → build with `npm run build` in `sites/wadhu/` (outputs `static/js/account.js` + `firebase-init.js`)
- `authDomain` is **`wadhu.com`** so popups/redirects use `https://wadhu.com/__/auth/handler`
- Methods on the page: email register/sign-in (+ verification email), Google, Microsoft, GitHub, phone SMS
- Overlay into the Hosting package with `./sites/wadhu/scripts/overlay-legal-onto-pkg.sh` (same as legal pages), then deploy Hosting

## Related product notes

- Firebase authenticates the **person**. Next platform step: map login → Wadhu
  **`user_id`** in our DB. Firebase UID is not the billing FK.
- Signup does **not** require an **org**. Org is only for multi-purchase /
  multi-member / scoped billing — [`IDENTITY_MODEL.md`](IDENTITY_MODEL.md).
- This file records **what was actually enabled** and the Apple postponement.
