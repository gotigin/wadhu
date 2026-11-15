# wadhu.com — Makepad 2 brand site

Package `wadhu-site`. Legal HTML under `static/` deploys with Firebase even
before wasm is published.

## Run

```bash
./sites/wadhu/scripts/run-desktop.sh   # native
./sites/wadhu/scripts/run-web.sh       # wasm → :8020
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
| `/privacy` | Privacy policy |
| `/terms` | Terms |
| `/about` | Etymology + company |
| connect.wadhu.com | Wadhu Connect API / ES UI (separate host) |
