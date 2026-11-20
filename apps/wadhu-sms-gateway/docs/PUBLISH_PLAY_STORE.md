# Publish SMS Gateway on Google Play

Application id: **`com.wadhu.smsgateway`**  
Product version: Cargo `version` / `version_name` in [`../Cargo.toml`](../Cargo.toml).  
Listing copy: [`PLAY_STORE_LISTING.md`](PLAY_STORE_LISTING.md)  
Data safety: [`DATA_SAFETY.md`](DATA_SAFETY.md)  
Android lessons: [`ANDROID_PLAY_LESSONS.md`](ANDROID_PLAY_LESSONS.md)

## Store URLs (required)

| Console field | URL |
| --- | --- |
| Privacy policy | https://wadhu.com/privacy |
| Terms (if asked) | https://wadhu.com/terms |
| Support URL | https://wadhu.com/support |
| Account / data deletion | https://wadhu.com/data-deletion |

Canonical Markdown: [`docs/legal/`](../../../docs/legal/README.md). Deploy the
site before review so reviewers can open the URLs:

```bash
./sites/wadhu/scripts/present-legal.sh
./sites/wadhu/scripts/deploy-firebase.sh
```

## Prerequisites

- Android SDK; Rust + Makepad Android toolchain ([`docs/DEV.md`](../../../docs/DEV.md))
- Release keystore (default pattern: `$HOME/smsgateway-release.jks` + `.password`)
- Play Console app created for `com.wadhu.smsgateway`
- **SMS policy path decided** (default handler / exception / private) — §5 of
  `ANDROID_PLAY_LESSONS.md`
- Cold-launch smoke on API 26 + a modern device

## Build the Play bundle

```bash
./scripts/check-store-readiness.sh
./apps/wadhu-sms-gateway/scripts/package-android.sh
```

`check-store-readiness.sh` checks brand framing, store URLs, app id, and
that the package script writes Play crash files. It does not build an APK.
Upload the **AAB** plus `native-debug-symbols.zip` and `mapping.txt` from
`dist/sms-gateway-android/` when the release keystore is set.

## Versioning

| Field | Rule |
| --- | --- |
| `version_name` | User-visible SemVer from Cargo metadata |
| `version_code` | Integer starting at `1`; bump by **1** each Play upload; never `auto` |

## Console checklist

1. Store listing (short + full from `PLAY_STORE_LISTING.md`)
2. Graphics (icon, feature graphic, screenshots — real UI)
3. Privacy policy URL → wadhu.com/privacy
4. Data safety → `DATA_SAFETY.md`
5. Content rating questionnaire (18+ / business)
6. Target audience — not children
7. News / COVID / financial — as applicable (usually none)
8. SMS declaration / exception docs if required by Play
9. App access instructions for reviewers (LAN test steps + test token policy —
   do **not** publish long-lived production tokens on the public listing)
10. Submit production or closed testing track

## Related

- Windows / MSI: [`PUBLISH_WINDOWS_STORE.md`](PUBLISH_WINDOWS_STORE.md)
- Brand legal index: [`docs/legal/README.md`](../../../docs/legal/README.md)
