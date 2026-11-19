# Gotigin product apps: configure clients, do not hardcode them

> **Agents working in Wadhu or across Gotigin brands:** product apps
> (especially under **Kumkaj** — Moklanjo, Khaynjo, Kirano, Tabiyat, and
> future trade apps) implement **features for many client setups**, but
> the app must **not** become concrete to one client. Templates,
> Settings, and rights **display or hide** capability as the owner
> desires. Cursor rule: `.cursor/rules/configure-not-hardcode-clients.mdc`.
>
> Full Kumkaj text (desk-rush examples, template layers, checklist):
> `kumkaj-workspace/docs/CLIENT_TEMPLATES_AND_SETUP.md`.

## Why this lives in Wadhu too

Wadhu holds **generic** tools and crates. Agents here still touch product
thinking when:

- scaffolding apps/crates that later power brand products
- documenting cross-brand patterns
- reviewing “client wants X” requests that might wrongly land as a
  hardcoded fork in a consumer app

Most Gotigin **business** apps will keep adding features that **some**
clients need. The durable shape is:

```text
✅ One trade product · many shop faces via config / templates / rights
❌ One binary (or hard delete) per client layout
```

## Two “concrete” meanings

| | Meaning |
|--|---------|
| **Concrete product** (Kumkaj) | A courier OS vs a food-floor OS — distinct apps, no shared product core. |
| **Not concrete-to-client** (this doc) | Inside one product, do not bake Client A’s screen as the only face. |

Example: a restaurant with counter beside kitchen may hide Kitchen / KDS
chrome via template or Settings; another restaurant keeps full Floor +
Kitchen. Same Khaynjo app.

## Wadhu’s own boundary

Wadhu still does **not** own restaurant KOT rules, courier AWB flows, or
client menu packs — those stay in the brand app ([§4](WADHU_VISION_AND_PRINCIPLES.md)).
When a Wadhu tool *supports* templates or feature flags generically, keep
them **options with safe defaults**, never a single consumer’s business
layout hardcoded into the crate.
