# wadhu.com — one Makepad brand site

Package `wadhu-site`. **One website**, one Firebase Hosting tree (same shape
as Khaynjo):

```text
public/
  index.html          ← Makepad SPA (one scrolling brand page)
  wadhu-site.wasm
  /privacy/…          ← same site, formatted legal HTML (Play / Meta URL)
  /terms/…
  /about/…
  /account/…          ← register / sign-in (Firebase Auth)
  /js/firebase-init.js
  /js/account.js
```

The SPA is one **scrolling** page (`ScrollYView`): hero (*means more*) →
brand → reuse → products → names → close. On a phone the top bar stacks and
the nav becomes four equal tabs on one row; headlines step down per tier
(30 / 36 / 42 / 48 px); product cards stack below 960px. Nav opens `/privacy`,
`/terms`, `/about`, and `/account` in the same tab on the web
(`OpenUrlInPlace::Yes`). Legal is **not** an in-app Makepad route. Connect
opens on connect.wadhu.com.

Text selection is **page-level** (`selection_root` + `select_from_gaps` on
`page`) via the workspace Makepad `text_selection` patch.

Legal Markdown lives in [`docs/legal/`](../../docs/legal/README.md). Present
with `./sites/wadhu/scripts/present-legal.sh` (also run by deploy). Do not
hand-edit generated HTML under `static/privacy/` etc. Connect embeds the
same files.

## Run

```bash
./sites/wadhu/scripts/run-desktop.sh   # native
./sites/wadhu/scripts/run-web.sh       # one tree → :8020
```

## Deploy

One tree: Makepad wasm + `/privacy` `/terms` `/account` … + Analytics/Auth.

```bash
./sites/wadhu/scripts/deploy-firebase.sh
./sites/wadhu/scripts/deploy-firebase.sh --dry-run
```

Firebase project: **`wadhu-ac8dc`** (`.firebaserc`). Analytics + Auth:
`npm run build` in this dir → `static/js/firebase-init.js` and
`static/js/account.js`. Auth notes: [`docs/FIREBASE_AUTH_SETUP.md`](../../docs/FIREBASE_AUTH_SETUP.md).
Custom domain **wadhu.com** in Hosting if not already.

## URLs

| Path | What |
| --- | --- |
| `/` | Makepad brand SPA (`wadhu-site`) |
| `/privacy` | Privacy — `docs/legal/PRIVACY.md` (Play / Store / Meta) |
| `/terms` | Terms — `docs/legal/TERMS.md` |
| `/data-deletion` | Data deletion — Play / Meta instructions |
| `/support` | Support contact |
| `/about` | Company + etymology — `docs/legal/ABOUT.md` |
| `/account` | Register / sign-in (Firebase Auth) |
| connect.wadhu.com | Wadhu Connect API / ES UI (separate host) |
