# Wadhu websites

| Site | Host | Crate / tree |
| --- | --- | --- |
| Brand | https://wadhu.com | `sites/wadhu` (`wadhu-site`) + Firebase |
| Connect API / ES UI | https://connect.wadhu.com | `apps/wadhu-connect` (Rust) |

## wadhu.com — one Hosting tree

Same model as Khaynjo: **one website**, not “static vs wasm.”

```text
public/
  index.html          ← Makepad SPA (scrolling brand page)
  wadhu-site.wasm
  /privacy/…          ← same site, formatted legal HTML
  /terms/ /about/ …
  /js/firebase-init.js
```

Deploy: `./sites/wadhu/scripts/deploy-firebase.sh` (project `wadhu-ac8dc`).
Local: `./sites/wadhu/scripts/run-web.sh`. Details: [`sites/wadhu/README.md`](../sites/wadhu/README.md).

Nav in the SPA opens `/privacy` etc. in a new tab (`OpenUrlInPlace::No`) so
the home canvas stays loaded. Legal is not an in-app Makepad route.

## Legal URLs (Play / Microsoft Store / Meta)

Canonical Markdown: [`legal/`](legal/README.md). Present with
`./sites/wadhu/scripts/present-legal.sh` (also run by deploy).

| URL | Source |
| --- | --- |
| https://wadhu.com/privacy | `docs/legal/PRIVACY.md` |
| https://wadhu.com/terms | `docs/legal/TERMS.md` |
| https://wadhu.com/data-deletion | `docs/legal/DATA_DELETION.md` |
| https://wadhu.com/support | `docs/legal/SUPPORT.md` |
| https://wadhu.com/about | `docs/legal/ABOUT.md` |

Connect embeds the same files and serves the same paths on
connect.wadhu.com. Do not hand-edit generated HTML under `sites/wadhu/static/`.

Connect is **not** Firebase Hosting — run the Rust binary behind TLS.

Auth / monetization: [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md).
Brand apps integrate Connect via [`apps/wadhu-connect/docs/INTEGRATOR.md`](../apps/wadhu-connect/docs/INTEGRATOR.md).
SMS Gateway store paste: [`apps/wadhu-sms-gateway/docs/PUBLISH_PLAY_STORE.md`](../apps/wadhu-sms-gateway/docs/PUBLISH_PLAY_STORE.md).
