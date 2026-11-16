# Android / Play lessons (SMS Gateway)

Hard-won rules for shipping a Makepad + Rust Android app to Google Play
without API‑26 boot crashes, black screens, ANRs, or Console rejects. Treat
every row as a **do-not-regress** contract for Wadhu apps.

Related:

- Activity / SMS / IME bridge: `scripts/apply-android-activity-bridge.sh`
- Signing / package helpers: `scripts/android-env.sh`
- UI layout: `.cursor/rules/wadhu-sms-gateway-ui.mdc`
- SMS / carrier / Play permission policy:
  `research_notes/Local SMS gateway solutions/compliance_platform_limits.md`

Brand: **wadhu.com** → application ids `com.wadhu.<app>` / `.debug`. One
website for all Wadhu utilities under that brand.

---

## 1. Lowest Android support (API 26) — boot crash / black SurfaceView

| Failure | Cause | Fix (do not regress) |
|---------|--------|----------------------|
| Instant crash on Android 8.0 | `InputDevice.isEnabled()` is **API 27+** → `NoSuchMethodError` | Guard with `SDK_INT >= 27` (or upstream `SDK_INT < 27 \|\| isEnabled()`). Applied via activity bridge. |
| Black GL / no UI after launch | Inline `WindowInsetsAnimation.Callback` in `ResizingLayout` → old ART **class-verify** fails (`ClassNotFoundException`) and leaves a dead SurfaceView | Keep callback in `ImeInsetsAnimationHelper.java`; attach only via `Class.forName` when `SDK_INT >= 30`. |
| Black screen for many seconds | Cargo plain **`dev`** on phone; or `android-dev` with `debug = true` → ~400 MB `libmakepad.so` | Always `cargo *-android-dev` with workspace profile **`android-dev`**: inherits release speed, `debug = "line-tables-only"`, still `android:debuggable=true` for live-dev. Production packages use **`--release`** (`debuggable=false`). |
| First frame stuck on clear color | First drawable before layout settles | `android_post_layout_redraw` (follow-up `redraw_all`s). |
| False “stuck UI” on some Huawei API 26 devices | `adb exec-out screencap` often **misses** the Makepad SurfaceView; snapshot backdrop looks like a stale real UI | Use system screenshot (`adb shell input keyevent 120`) / `dumpsys activity` — never trust screencap alone on those tablets. |

**SMS Gateway:** `min_sdk_version = 26`, `[profile.android-dev]` set, bridge present. Verify on a real low-API device before Play — not only a new phone.

---

## 2. Soft keyboard (IME) — bounce, retap silent, buried fields

On API 26–29 with Makepad’s `adjustNothing` activity:

1. **▾ hide bounces back / retap silent** — patches `android_ime_dismiss` + `android_ime_retap_show`; Java `SHOW_FORCED` / InsetsController. Never clear key focus from `DidHide` (breaks touch-scroll).
2. **No measured IME height below API 30** — WindowManager does not shrink the app window under `adjustNothing`, so Rust never got `DidShow{height}`. Fix: `MakepadImeProbe` PopupWindow in `adjustResize` that *does* resize → real heights. Stock `KeyboardView` lift only — **no** app-side guessed heights / poll loops (those caused beige gaps and freezes).
3. **Field → field** — skip hide when the next field already requested IME (`android_ime_field_switch`).

**Touch-scroll must stay intact while fixing IME:** `scroll_axis_lock`, `drag_scroll_touch_only`, `scroll_touch_over_buttons`, `text_input_touch_scroll`. Never “simplify” those away.

Manifest: `windowSoftInputMode="adjustNothing|stateUnchanged"`.

---

## 3. UI freezes / ANR (look like “crashes” to reviewers)

| Failure | Cause | Rule |
|---------|--------|------|
| App frozen, high CPU | `flow: Right{wrap:true}` + `width: Fill` child → turtle log spam every frame | Avoid that layout; Fit labels on wrap faces; patch `turtle_wrap_fill_no_spam` if needed. |
| “Not responding” / windowless process | Nested `Mutex` on UI thread | Never call a function that locks the same mutex while a guard is live. |
| Tap feels dead for seconds | Heavy work on UI thread (SQLite, network, image decode, full list redraw) | Workers + wake UI; busy chrome = Progress + Cancel; never block the UI thread. |
| Phone unscrollable | Full-height list drawing every expensive row | Cap the visible window; only on-screen rows draw. |

Apply to outbox / recent lists, relay WS status, and any future heavy UI.

---

## 4. Play Console packaging

Package id: **`com.wadhu.smsgateway`** (dev: `.debug`).

### Signing

Defaults under `$HOME` (no env required when present):

| File | Role |
|------|------|
| `~/smsgateway-release.jks` | Play / production upload key |
| `~/smsgateway-release.jks.password` | one line, mode `600` |
| `~/smsgateway-release.jks.makepad` | alias sidecar (`upload`) |

Slug = product label lowercased with spaces removed (`SMS Gateway` → `smsgateway`). Resolved by `scripts/android-env.sh` (override: `WADHU_ANDROID_KEYSTORE` / `_PASS`).

- Back up the `.jks` + `.password` outside the machine forever. Lost key = **no more updates** for that package id.
- Debug-signed APKs are for sideload only — **not** for Play.
- Never uninstall a shop device to “fix signing” if that wipes data (outbox / API key / relay pairing).

### Versioning

- New application id → `version_code = 1`.
- Each Play upload → bump **`+1`** manually in Cargo `[package.metadata.makepad.android]`.
- **Never** `--version-code=auto` / date timestamps (burns Play’s ceiling and trips Console warnings).
- `version_name` holds marketing semver; do not encode majors into `version_code`.
- `targetSdkVersion` must meet current Play policy (confirm in cargo-makepad `sdk.rs` at upload time; historically ≥ 36). `min_sdk_version` stays **26** for reach.

### Crash assets (upload every AAB)

| File | Why |
|------|-----|
| `native-debug-symbols.zip` | Almost all logic is Rust in `libmakepad.so` — Play vitals need symbols |
| `mapping.txt` | Identity Java map (Makepad uses D8, not R8) — clears “no deobfuscation file” warning |

Archive **AAB + zip + mapping** per `version_code`. Local: `ndk-stack -sym …/arm64-v8a`.

### Pre-upload gate

Do not ship a Play cut without:

- Cold launch on API 26 (or similar low API) **and** a modern device
- SMS permission grant + one successful queue→send
- Logcat clean of FATAL / ANR; process must show a real UI (pid-alive-without-UI fails)

### Manifest hygiene

- Do **not** declare broad media / sensitive permissions you do not need.
- Mark optional hardware `required="false"` when the app runs without it.
- `largeHeap="true"` is fine if GL/assets need it.
- `SEND_SMS` is sensitive — see §5.

---

## 5. SMS-specific Play risk (beyond Makepad)

Play’s SMS/Call Log policy is stricter than “it compiles”:

- Declaring `SEND_SMS` / `READ_SMS` / `RECEIVE_SMS` for a Play app generally requires being the **default SMS handler** (or an approved exception: automation / enterprise CRM, etc.).
- Default SMS apps must **actually** provide SMS UI and a privacy policy; ask to become default **before** requesting SMS permissions.
- Sideload / private shop distribution avoids some Play SMS rules but hits Play Protect / OEM blockers differently.
- Product decision before first Play listing: **default-SMS path**, **Play exception declaration**, or **private APK only**. Do not upload an AAB that only has `SEND_SMS` with no handler story.

Details: `research_notes/Local SMS gateway solutions/compliance_platform_limits.md`.

Privacy / listing URLs for the brand live under **https://wadhu.com** (shared site for all Wadhu utilities).

---

## 6. Day-to-day Android checklist

```bash
# Live-dev (debuggable, android-dev profile) — pin device when several are plugged in
export ANDROID_SERIAL=<serial>
adb -s "$ANDROID_SERIAL" get-state
cargo wadhu-sms-gateway-android-dev

# After launch
adb -s "$ANDROID_SERIAL" logcat -s Makepad:* RustStdoutStderr:* AndroidRuntime:E ActivityManager:E
# Grant SMS; confirm LAN listening; queue a test SMS

# Production-shaped package (when ready)
./apps/wadhu-sms-gateway/scripts/package-android.sh
# Expect: signed AAB (+ crash symbol files when package script writes them) under dist/
```

Bridge must **not** abort the pipeline on optional upstream patch drift
(`apply-android-activity-bridge.sh` soft-skips and continues). Hard-fail only when
MakepadActivity / SMS bridge is truly missing.

**cargo-makepad rev must match `vendor/makepad`.** `scripts/android-env.sh`
pins `MAKEPAD_REV` to the same commit as `scripts/setup-makepad.sh` and
materializes `vendor/makepad-full` (complete workspace, including
`examples/`). Pointing at a newer cargo git checkout (e.g. one that requires
a `.makepad.font-assets.v1` ELF section the app tree does not emit) breaks
`android run` after a successful compile.

---

## 7. Checklist vs current SMS Gateway

| Rule | Status / action |
|------|-----------------|
| API 26 `isEnabled` / IME helper reflection | Bridge + upstream-safe forms; keep minSdk 26 |
| `android-dev` profile (not plain `dev`) | Present in workspace `Cargo.toml` |
| `~/smsgateway-release.jks` (+ `.password`, `.makepad`) | Created; auto-resolved by `android-env.sh` |
| `version_code` discipline | Cargo `version_code = 1` — bump +1 per upload; no `auto` |
| Native symbols + mapping on package | Ensure `package-android.sh` writes Play crash files before first upload |
| No UI-thread I/O / freezes | Keep gateway drain / relay off the UI thread; busy chrome if long |
| Layout / Inset / wrap rules | `.cursor/rules/wadhu-sms-gateway-ui.mdc` |
| Device launch smoke | Real phone + ideally API 26 tablet before any listing |
| SMS Play policy | **Open product decision** — default SMS / exception / private APK |

Play cut docs (listing, Data safety, upload):
[`PUBLISH_PLAY_STORE.md`](PUBLISH_PLAY_STORE.md),
[`PLAY_STORE_LISTING.md`](PLAY_STORE_LISTING.md),
[`DATA_SAFETY.md`](DATA_SAFETY.md). Legal URLs live under
[`docs/legal/`](../../../docs/legal/README.md).
