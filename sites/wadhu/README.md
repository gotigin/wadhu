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
  /js/firebase-init.js
```

The SPA is one **scrolling** page (`ScrollYView`): hero → products → Connect →
about → close. Nav opens `/privacy`, `/terms`, `/about` with `open_link`
(`OpenUrlInPlace::No`) so the home SPA stays loaded — legal is **not** an
in-app Makepad route.

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

One tree: Makepad wasm + `/privacy` `/terms` … + Analytics.

```bash
./sites/wadhu/scripts/deploy-firebase.sh
./sites/wadhu/scripts/deploy-firebase.sh --dry-run
```

Firebase project: **`wadhu-ac8dc`** (`.firebaserc`). Analytics:
`client/firebase-init.js` → bundled `static/js/firebase-init.js`. Custom
domain **wadhu.com** in Hosting if not already.

## URLs

| Path | What |
| --- | --- |
| `/` | Makepad brand SPA (`wadhu-site`) |
| `/privacy` | Privacy — `docs/legal/PRIVACY.md` (Play / Store / Meta) |
| `/terms` | Terms — `docs/legal/TERMS.md` |
| `/data-deletion` | Data deletion — Play / Meta instructions |
| `/support` | Support contact |
| `/about` | Company + etymology — `docs/legal/ABOUT.md` |
| connect.wadhu.com | Wadhu Connect API / ES UI (separate host) |
