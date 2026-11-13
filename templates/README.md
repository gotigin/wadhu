# templates/

Starters copied by the scaffolding scripts. They are workspace members, so
they keep compiling and their tests keep running, but **nothing may depend
on them**: copy, never link.

| Template | Package | Copied by |
|---|---|---|
| [`app/`](app/README.md) | `wadhu-template-app` | `./scripts/new-app.sh <name>` |
| [`crate/`](crate/README.md) | `wadhu-template-crate` | `./scripts/new-crate.sh <name>` |
| [`widgets/`](widgets/README.md) | `wadhu-template-widgets` | `./scripts/new-crate.sh <name> --widgets` |

When you improve a template, the change reaches only apps and crates
created afterwards. Existing copies own their code and diverge freely.
