# Publish SMS Gateway on the Microsoft Store

Application / package id (desktop): **`com.wadhu.smsgateway`**  
Product version: Cargo `version` in [`../Cargo.toml`](../Cargo.toml).

## Store URLs (required)

| Partner Center field | URL |
| --- | --- |
| Privacy policy URL | https://wadhu.com/privacy |
| Support / contact | https://wadhu.com/support · mearaj@gotigin.com |
| Terms of use (if asked) | https://wadhu.com/terms |
| Website | https://wadhu.com |

Canonical Markdown: [`docs/legal/`](../../../docs/legal/README.md).

## Current packaging in this repo

| Artifact | Script | Notes |
| --- | --- | --- |
| Folder + Setup | `./apps/wadhu-sms-gateway/scripts/windows-prod.sh` | Sideload / USB |
| Per-user MSI | `./apps/wadhu-sms-gateway/scripts/msi.sh` | Sideload; License.rtf ships with package |

**MSIX / Partner Center identity** is not wired yet for SMS Gateway (no
`msix.sh` in this app). Until MSIX exists:

1. Use MSI / folder package for sideload and enterprise
2. When you reserve the Store app, set Identity Name stably (never rename after
   first publish) and add `apps/wadhu-sms-gateway/scripts/msix.sh` following Khaynjo’s
   pattern (`MearajBhagad.*` + Publisher CN from Partner Center)
3. Listing still needs the same privacy / support URLs above before certification

## Partner Center checklist (when MSIX is ready)

1. Reserve app name (e.g. “SMS Gateway” / “Wadhu SMS Gateway”)
2. Copy Package/Identity/Name + Publisher CN into the MSIX packer
3. Upload MSIX
4. Age rating (business / 18+ — not for children)
5. Privacy policy URL → https://wadhu.com/privacy
6. Capabilities review (network, and any SMS-related capabilities if declared)
7. Store listing description aligned with
   [`PLAY_STORE_LISTING.md`](PLAY_STORE_LISTING.md) (adapt wording for Windows)
8. Submit for certification

## Related

- Play: [`PUBLISH_PLAY_STORE.md`](PUBLISH_PLAY_STORE.md)
- Legal: [`docs/legal/README.md`](../../../docs/legal/README.md)
