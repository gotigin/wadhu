# Wadhu websites

| Site | Host | Crate / tree |
| --- | --- | --- |
| Brand | https://wadhu.com | `sites/wadhu` (`wadhu-site`) + Firebase |
| Connect API / ES UI | https://connect.wadhu.com | `apps/wadhu-connect` (Rust) |

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

Pattern (same family as Kumkaj / Khaynjo): Makepad 2 fullpage wasm + thin
static shell. Deploy brand site: `./sites/wadhu/scripts/deploy-firebase.sh`.

Connect is **not** Firebase Hosting — run the Rust binary behind TLS.

Auth / monetization: [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md).
Brand apps integrate Connect via [`apps/wadhu-connect/docs/INTEGRATOR.md`](../apps/wadhu-connect/docs/INTEGRATOR.md).
SMS Gateway store paste: [`apps/sms-gateway/docs/PUBLISH_PLAY_STORE.md`](../apps/sms-gateway/docs/PUBLISH_PLAY_STORE.md).
