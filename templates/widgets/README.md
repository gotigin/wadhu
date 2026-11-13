# wadhu-template-widgets

Starter for a reusable **Makepad 2 widget** crate. Copy it:

```bash
./scripts/new-crate.sh <name> --widgets
```

Ships one example widget, `TemplatePill` (a rounded label chip), and a
runnable demo:

```bash
cargo run -p wadhu-template-widgets --example demo
```

## Use from an app

```toml
[dependencies]
wadhu-template-widgets = { path = "../../crates/template-widgets" }
```

```rust
fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
    crate::makepad_widgets::script_mod(vm);
    wadhu_template_widgets::script_mod(vm); // after makepad_widgets, before the app
    self::script_mod(vm)
}
```

In the app's `script_mod!` add `use mod.widgets.*`, then use
`TemplatePill{}`; from Rust, `use wadhu_template_widgets::TemplatePillWidgetRefExt`
and call `ui.template_pill(cx, ids!(pill)).set_text(cx, "…")`.

From **another workspace**, also add the one-line `[patch.crates-io]` for
`makepad-widgets` — [`docs/DEV.md`](../../docs/DEV.md) § Consuming Wadhu
from another workspace. Rules: [`docs/CRATES.md`](../../docs/CRATES.md).
