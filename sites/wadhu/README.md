# wadhu.com — one Makepad brand site

Package `wadhu-site`. **One website**, one Firebase Hosting tree (same shape
as Khaynjo):

```text
public/
  index.html          ← Makepad SPA (one scrolling brand page)
  wadhu-site.wasm
  /privacy/…          ← same site, formatted legal HTML (Play / Meta URL)
  /terms/…
  /cancellation/…     ← subscription cancellation & refund
  /data-deletion/…
  /support/…
  /about/…
  /account/…          ← register / sign-in (Firebase Auth)
  robots.txt
  sitemap.xml
  /js/firebase-init.js
  /js/account.js
  /media/link-preview.jpg ← picture for link previews (see below)
  /build-id.txt       ← what is live
```

The SPA is one **scrolling** page (`ScrollYView`): hero (*means more*) →
brand → reuse → products → names → close. On a phone the top bar stacks and
the nav becomes four equal tabs on one row; headlines step down per tier
(30 / 36 / 42 / 48 px); product cards stack below 960px. Nav opens `/privacy`,
`/terms`, `/about`, and `/account` in the same tab on the web
(`OpenUrlInPlace::Yes`; Cancellation is in the desk chips and the closing
band). Legal is **not** an in-app Makepad route. Connect opens on
connect.wadhu.com.

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

## The shell around the wasm

Three things live in the HTML, because nothing can run before the wasm has
arrived and link crawlers never run it at all. `overlay-legal-onto-pkg.sh`
puts them in, so `run-web.sh` and deploy show the same thing.

- **Start-up screen** — `shell/boot.{css,html,js}` and `shell/logo.webp`,
  inlined into `index.html` by `scripts/patch-web-shell.sh`. The logo sits
  in a ring that fills with the real wasm download, then the status reads
  "Starting" until the canvas has painted and the screen fades out. It says
  so when the connection is slow and offers Reload and the plain pages when
  the page cannot start.
- **Link preview** — `static/media/link-preview.jpg`, a 1200×630 JPEG of the
  real first screen, named by the Open Graph and Twitter tags on the home
  page and on every legal page. One set of tags serves WhatsApp, X,
  LinkedIn, Facebook, Telegram, Discord, Slack and iMessage. After the
  first screen changes: `run-web.sh` in one terminal, then
  `./sites/wadhu/scripts/make-link-preview.sh http://127.0.0.1:8020/`,
  commit the picture, deploy. Its address carries a hash of the file, so
  apps that cache previews fetch the new one.
- **Addresses that change per build** — deploy gives the wasm and its
  loader `?v=<build id>` and the wasm is served `immutable`, so a returning
  visitor downloads it once per release. `/build-id.txt` says what is live.

- **Fonts cut to the page's own characters** — `scripts/subset-fonts.sh`
  (deploy only; needs `pyftsubset`, skipped without it) rewrites the staged
  copies of the two IBM Plex Sans faces to the characters in `src/*.rs`
  plus plain Latin: 355 KB → 90 KB, the same pixels. A character the page
  starts to use is picked up on the next deploy.

The pictures in `resources/media/` are about 200 px WebP with transparent
corners because they are drawn at 40–72 px on a dark page; keep new ones
that small (`Internal/tools/site-shots/clean-icon.sh` made these from the
square originals). The wasm is 1.3 MB compressed — the Makepad runtime,
already built for size. Two things were tried on it on 2026-11-21 and do
not help: `--split` sends more bytes, not fewer, and Binaryen's `wasm-opt`
makes the file smaller raw but 2–3% larger once compressed, with the same
time to first paint.

## URLs

| Path | What |
| --- | --- |
| `/` | Makepad brand SPA (`wadhu-site`) |
| `/privacy` | Privacy — `docs/legal/PRIVACY.md` (Play / Store / Meta) |
| `/terms` | Terms — `docs/legal/TERMS.md` |
| `/cancellation` | Cancellation / refund — `docs/legal/CANCELLATION.md` |
| `/data-deletion` | Data deletion — Play / Meta instructions |
| `/support` | Support contact |
| `/about` | Company + etymology — `docs/legal/ABOUT.md` |
| `/account` | Register / sign-in (Firebase Auth) |
| `/robots.txt` | Crawl allow + sitemap pointer |
| `/sitemap.xml` | Home + legal + account URLs |
| connect.wadhu.com | Wadhu Connect API / ES UI (separate host) |
