# Wadhu websites

| Site | Host | Crate / tree |
| --- | --- | --- |
| Brand | https://wadhu.com | `sites/wadhu` (`wadhu-site`) + Firebase |
| Connect API / ES UI | https://connect.wadhu.com | `apps/wadhu-connect` (Rust) |

Legal: `/privacy` `/terms` `/about` under `sites/wadhu/static/`.

Pattern (same family as Kumkaj): Makepad 2 fullpage wasm + thin static shell.
Deploy brand site: `./sites/wadhu/scripts/deploy-firebase.sh`.

Connect is **not** Firebase Hosting — run the Rust binary behind TLS.

Auth / monetization: [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md).
Brand apps integrate Connect via [`apps/wadhu-connect/docs/INTEGRATOR.md`](../apps/wadhu-connect/docs/INTEGRATOR.md).
