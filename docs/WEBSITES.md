# Wadhu websites

| Site | Host | Crate / tree |
| --- | --- | --- |
| Brand | https://wadhu.com | `sites/wadhu` (`wadhu-site`) + Firebase |
| Connect API / ES UI | https://connect.wadhu.com | `apps/wadhu-connect` (Rust) |

## Languages + countries (hard rule)

Same Gotigin product floor as Kumkaj: every Wadhu **app and website**
supports the full language catalogue (**115+ languages**) and **199
countries**. wadhu.com must translate marketing home + account chrome
(legal bodies stay English). No preference-only language picker — packs
under `/i18n/<code>.tsv`, `lang-nav.js` writes `#l=`, wasm + `site-i18n.js`
repaint. See `.cursor/rules/languages-countries-floor.mdc`.

## Responsive (graceful growth)

Same law as [`DESIGN.md`](DESIGN.md) § Always responsive: phone-safe
stacking must unlock on wide viewports. Fit chip / short CTA toolbars that
fit on one line at 1280 / 1920 **must** be one line — forever-`Down` with
empty horizontal space is a bug. Exceptions exist (user-owned density and
other intentional stacks — mostly obvious; still take care; if unsure
leave alone). Verify narrow **and** wide when changing `apply_responsive`
or toolbar DSL. **Android Chrome Desktop site / zoom:** layout must clamp
to phone chrome width (`web_mobile_layout_width` + shell
`mobile-viewport-guard.js`). Gate: `./scripts/check-web-mobile-layout.sh`.

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

The SPA is one scroll: hero (*Wadhu means more*) → what the brand is → reuse → products (SMS Gateway, SMS Relay, Wadhu Connect) → names → close. Desktop nav opens `/privacy`, `/terms`, `/cancellation`, `/about`, and `/account` in the same tab on the web (`OpenUrlInPlace::Yes`); phone keeps four short tabs (Privacy · Terms · About · Account) and Cancellation lives in the closing band. Connect stays a separate host. Legal is not an in-app Makepad route.

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
| https://wadhu.com/cancellation | `docs/legal/CANCELLATION.md` |

Also deployed from `sites/wadhu/static/`: `robots.txt`, `sitemap.xml`
(home + legal + account). Safe response headers live in
`sites/wadhu/firebase.json` (`X-Content-Type-Options`, `Referrer-Policy`,
`Permissions-Policy`, `X-Frame-Options`). A Content-Security-Policy is
**not** set yet — the wasm shell needs inline boot script and
`'wasm-unsafe-eval'`; prove a CSP in the Hosting emulator before adding
one. The wasm shell carries a crawlable `<noscript>` summary and JSON-LD
`Organization` (Wadhu brand, `parentOrganization` Gotigin).

Connect embeds the same legal Markdown and serves the same paths on
connect.wadhu.com. Do not hand-edit generated HTML under `sites/wadhu/static/`.

Connect is **not** Firebase Hosting — run the Rust binary behind TLS.

Auth / monetization: [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md).
Brand apps integrate Connect via [`apps/wadhu-connect/docs/INTEGRATOR.md`](../apps/wadhu-connect/docs/INTEGRATOR.md).
SMS Gateway store paste: [`apps/wadhu-sms-gateway/docs/PUBLISH_PLAY_STORE.md`](../apps/wadhu-sms-gateway/docs/PUBLISH_PLAY_STORE.md).
