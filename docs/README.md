# Wadhu docs

Wadhu, by Gotigin (*Tools · Apps · More*), is Gotigin's home for reusable
code: crates, apps and tools that other Gotigin brands' apps need, and that
anyone else can use. Rust + Makepad 2, one Cargo workspace, and **every
part independent of its siblings**.

| Doc | What |
|---|---|
| [`WADHU_VISION_AND_PRINCIPLES.md`](WADHU_VISION_AND_PRINCIPLES.md) | **Canonical doctrine:** what Wadhu is, who it serves, the independence rules (§3), what belongs here (§4), stability (§5) |
| [`DEV.md`](DEV.md) | Developer how-to: layout, setup, commands, new app/crate, env and ids, icons, packaging, Makepad 2 (patches, agent notes), consuming Wadhu from another workspace, screenshots |
| [`CRATES.md`](CRATES.md) | Rules for reusable crates: shape, independence, features, API, versioning, MSRV, tests, publishing |
| [`DESIGN.md`](DESIGN.md) | Wadhu brand tokens (colours sampled from the logo), type, voice, icons |
| [`LOCAL_SMS.md`](LOCAL_SMS.md) | Local SMS: **gateway (provider) vs client** roles, API/status contract; gateway app is [`apps/sms-gateway/`](../apps/sms-gateway/README.md) |
| [`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md) | Post-BT alignment: monetize relay/platform only; IdP/orgs staged after LAN+BT (not a same-day build) |
| [`WEBSITES.md`](WEBSITES.md) | wadhu.com (Makepad + Firebase) and connect.wadhu.com (Rust Connect) |
| [`BRAND.md`](BRAND.md) | **Names:** Wadhu=brand, Gotigin=trademark, company=legal entity, products under Wadhu |
| [`GOTIGIN_COMPANY_IDENTITY.md`](GOTIGIN_COMPANY_IDENTITY.md) | Company / Meta form values (CIN, portfolio, Wadhu Connect app ids) |
| [`legal/`](legal/README.md) | **Store / Meta legal:** privacy, terms, data deletion, support, about (one MD each → wadhu.com) |
| [`WARNINGS.md`](WARNINGS.md) | **Strict:** fix compile warnings on all platforms; never suppress |
| [`plans/BT_THEN_IDENTITY.md`](plans/BT_THEN_IDENTITY.md) | Readable plan: finish Classic RFCOMM first, then identity/monetization direction |
| [`TRANSPORTS.md`](TRANSPORTS.md) | LAN / Bluetooth / relay: user-facing behaviour + gateway security controls |

## Documentation system (read before adding or moving a doc)

Docs are organised by **scope**, with one canonical home per idea:

| Scope | Lives in | Contains |
|---|---|---|
| **Wadhu-wide** | root `docs/` | Ideas, rules and how-tos that apply to every app and crate |
| **One app** | `apps/<app>/README.md`, `apps/<app>/docs/` | That app's brief (`docs/<APP>.md`), state, protocols, decisions |
| **One crate** | `crates/<crate>/README.md`, `CHANGELOG.md`, rustdoc | What it does, how to use it, what changed |

**Placement rules**

1. An idea that applies to more than one app or crate, or would apply to
   a future one, goes in root `docs/`, even if one app discovered it.
2. An app or crate doc states only its own facts and **links** to the root
   doc. Never restate root text: duplicated docs drift. Code may be
   duplicated (copy-then-diverge); docs may not.
3. Create a new root doc only for a large Wadhu-wide topic with no home in
   the four above, and add it to this index in the same change.
4. When a root rule changes, update the root doc once, then each affected
   app or crate, one at a time.

## Elsewhere

- Workspace overview and quick start: [`../README.md`](../README.md)
- Agent rules: [`../CLAUDE.md`](../CLAUDE.md), [`../.cursor/rules/`](../.cursor/rules/)
- Templates: [`../templates/`](../templates/README.md)
