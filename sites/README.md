# sites/

Makepad 2 fullpage sites (wasm in the browser, the same crate native for
previews), following the pattern Kumkaj uses for its product sites
(`kumkaj-workspace/sites/`). A site may depend on Wadhu crates, never on an
app or another site.

| Site | Package | Domain |
| --- | --- | --- |
| [`wadhu/`](wadhu/README.md) | `wadhu-site` | https://wadhu.com |

Deploy: `./sites/wadhu/scripts/deploy-firebase.sh`. Connect API is separate:
https://connect.wadhu.com (`apps/wadhu-connect`).
