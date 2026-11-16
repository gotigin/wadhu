# Wadhu legal — one Markdown file per document

Naming: [`../BRAND.md`](../BRAND.md) (Wadhu=brand, Gotigin=trademark,
company=GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED).

Edit **only** the `.md` files in this folder. Sites and apps **present** them
as formatted HTML (Khaynjo pattern). Do not hand-edit generated HTML under
`sites/wadhu/static/`.

| File | Public URL | Used for |
| --- | --- | --- |
| [`PRIVACY.md`](PRIVACY.md) | https://wadhu.com/privacy | Play, Microsoft Store, Meta privacy URL |
| [`TERMS.md`](TERMS.md) | https://wadhu.com/terms | Store / site terms |
| [`DATA_DELETION.md`](DATA_DELETION.md) | https://wadhu.com/data-deletion | Play account deletion / Meta data deletion instructions |
| [`SUPPORT.md`](SUPPORT.md) | https://wadhu.com/support | Store support URL |
| [`ABOUT.md`](ABOUT.md) | https://wadhu.com/about | Company / brand |

Regenerate site HTML:

```bash
./sites/wadhu/scripts/present-legal.sh
```

Connect embeds the same Markdown (`apps/wadhu-connect` → `include_str!` of
these files) and serves `/privacy`, `/terms`, `/data-deletion`, `/support`,
`/about`.

SMS Gateway store paste docs:
[`apps/sms-gateway/docs/PLAY_STORE_LISTING.md`](../../apps/sms-gateway/docs/PLAY_STORE_LISTING.md),
[`DATA_SAFETY.md`](../../apps/sms-gateway/docs/DATA_SAFETY.md),
[`PUBLISH_PLAY_STORE.md`](../../apps/sms-gateway/docs/PUBLISH_PLAY_STORE.md),
[`PUBLISH_WINDOWS_STORE.md`](../../apps/sms-gateway/docs/PUBLISH_WINDOWS_STORE.md).
