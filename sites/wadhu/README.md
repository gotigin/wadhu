# wadhu.com — Makepad 2 brand site

Package `wadhu-site`. One **scrolling** page (`ScrollYView`), not a slide deck —
wheel / touch scroll through hero → products → Connect → about → close.

Text selection is **page-level** (`selection_root` + `select_from_gaps` on
`page`) via the workspace Makepad `text_selection` patch — drag across
blocks, Ctrl/Cmd+A, Copy, right-click / long-press menu. Do not re-add
select per label.

Legal pages come from [`docs/legal/`](../../docs/legal/README.md) (privacy,
terms, data-deletion, support, about). Connect embeds the same files. Present
with `./sites/wadhu/scripts/present-legal.sh` (also run by deploy). Do not
hand-edit `static/privacy/` (or other generated legal HTML).

## Run

```bash
./sites/wadhu/scripts/run-desktop.sh   # native
./sites/wadhu/scripts/run-web.sh       # Makepad wasm + legal routes → :8020
# Builds wasm, presents docs/legal, overlays /privacy etc. WITHOUT replacing
# the Makepad index.html (that bug made the home look like a squeezed legal page).
```

## Deploy

```bash
./sites/wadhu/scripts/deploy-firebase.sh           # static shell + legal
./sites/wadhu/scripts/deploy-firebase.sh --with-wasm
```

Firebase project id is in `.firebaserc` (change to your real project). Add
custom domain **wadhu.com**.

## URLs

| Path | What |
| --- | --- |
| `/` | Brand story (static shell and/or Makepad) |
| `/privacy` | Privacy — `docs/legal/PRIVACY.md` (Play / Store / Meta) |
| `/terms` | Terms — `docs/legal/TERMS.md` |
| `/data-deletion` | Data deletion — Play / Meta instructions |
| `/support` | Support contact |
| `/about` | Company + etymology — `docs/legal/ABOUT.md` |
| connect.wadhu.com | Wadhu Connect API / ES UI (separate host) |
