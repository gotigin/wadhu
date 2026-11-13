# Wadhu Template (app)

Package `wadhu-template-app`. The starter every Wadhu app is copied from —
a minimal Makepad 2 app wired the Wadhu way. **Never depend on it**; copy it:

```bash
./scripts/new-app.sh <name> [--id com.example.app] [--label "App Name"]
```

## Run

```bash
cargo template-app-dev                    # live dev: --hot reload, --remote (F12)
cargo template-app-android-dev            # Android: build, install, launch
cargo template-app-windows-dev            # Windows cross package (dev app id)
cargo run -p wadhu-template-app           # plain run
cargo test -p wadhu-template-app          # env/id tests
```

## Layout

| Path | What |
|---|---|
| `src/main.rs` | Makepad 2 shell: `script_mod!` UI + `AppMain` (Android compiles it into the lib) |
| `src/lib.rs` | Non-UI half — everything tests and the shell both call |
| `src/identity.rs` | App id / label / env baked by `build.rs` |
| `build.rs` | Bakes `env/<WADHU_ENV>.env` into the binary |
| `env/development.env`, `env/production.env` | Ids and labels (`<id>.debug` for dev) |
| `scripts/dev.sh` | Live dev (what `cargo template-app-dev` runs) |
| `scripts/package-*.sh`, `windows-*.sh`, `msi.sh`, installers | Per-app packaging (see [`docs/DEV.md`](../../docs/DEV.md) § Packaging) |
| `scripts/generate-icons.sh` | Icons from `assets/app-icon.png` → `resources/` |
| `docs/TEMPLATE_APP.md` | App brief — the app's own docs live in its `docs/` |

Ids: production `com.wadhu.template` · development
`com.wadhu.template.debug`. Conventions: [`docs/DEV.md`](../../docs/DEV.md).
