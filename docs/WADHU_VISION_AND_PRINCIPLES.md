# Wadhu — Vision & Principles

> **Brand:** Wadhu — *Tools · Apps · More*.\
> **Trademark:** Gotigin.\
> **Company:** GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED.\
> **What it is:** the Wadhu brand’s home for **reusable code**: libraries, tools
> and apps that other Gotigin brands’ apps need, built so that anyone else
> can use them too. Naming: [`BRAND.md`](BRAND.md).\

> **Document purpose:** the canonical statement of what belongs in Wadhu,
> how its parts relate, and the rules that keep each part independent.
> Other docs link here instead of restating it.\
> **Decision rule:** when another doc, a comment or an old plan conflicts
> with this document, this document wins; when this document changes, the
> later decision wins.

------------------------------------------------------------------------

## 1. What Wadhu is

Gotigin builds products under several brands. **Kumkaj**, for example,
ships concrete business operating system apps (Moklanjo, Khaynjo, Kirano,
Tabiyat), each owning all of its own product code. Those apps, and apps
under future brands, keep needing the same kinds of **generic building
blocks**: printing to receipt printers, LAN discovery, file and image
handling, widgets, protocol clients, small helper programs.

Wadhu is where those building blocks live: **written once, in one place,
on one platform (Rust + Makepad 2), and reusable by any app**, whether it
is a Gotigin app or not.

Wadhu ships three kinds of things:

| Kind | Lives in | What it is | How others use it |
|---|---|---|---|
| **Crate** | `crates/<name>/` | A library: one clear job, brand-neutral | As a Cargo dependency |
| **App** | `apps/<name>/` | A standalone program (tool, service, desktop/mobile app) that other apps or people need | Run it, and talk to it over a boundary: CLI, LAN/HTTP, files, OS intents |
| **Site** | `sites/<name>/` | A Makepad 2 fullpage web/native site (e.g. a future Wadhu brand site) | Visit it |

Supporting pieces, never products: `tools/<app>-dev` (one cargo-alias shim
per app), `templates/` (the starters that `scripts/new-*.sh` copy), root
`scripts/` (tooling that names no app), `patches/makepad/` + `vendor/`
(the shared Makepad 2 platform).

## 2. Who Wadhu serves

1. **Other Gotigin brands' apps.** A Kumkaj app depends on a Wadhu crate
   exactly as it depends on any third-party crate, or runs a Wadhu app
   beside itself.
2. **Everyone else.** Wadhu code is written to be useful outside Gotigin:
   documented, generic, permissively licensed (`MIT OR Apache-2.0`), with
   no assumption about who is calling it.
3. **Wadhu's own apps.** They consume Wadhu crates like any outside app
   would, with no back doors.

## 3. Independence rules

Every part of Wadhu must stay usable on its own. One part changing must
never force a sibling to change. These are the rules
(`scripts/check-independence.sh` enforces 3.1–3.4 on every member):

**3.1 An app never depends on a sibling app.** Not as a crate dependency,
not through shared source files, not through a shared script. If two apps
need the same code, that code is either a crate (§4) or a copy each app
owns.

**3.2 A crate never depends on a sibling crate or on any app.** Each
crate is self-contained, so a consumer can take exactly one crate and
nothing else from Wadhu. When a crate wants another crate's code, it
copies the code (copy-then-diverge). When a crate wants UI, it puts the
UI behind an optional feature of **the same crate** (see
[`CRATES.md`](CRATES.md) § Features), not in a second crate it depends on.

**3.3 Apps and sites may depend on crates.** That is what crates are for.

**3.4 A crate reaches Makepad only through the workspace dependency**
(`makepad-widgets = { workspace = true }`), never through a path into
`vendor/`. That lets a consumer in another workspace point it at its own
Makepad tree ([`DEV.md`](DEV.md) § Consuming Wadhu from another workspace).

**3.5 Tooling is per app.** Each app has its own `tools/<app>-dev` shim,
its own `scripts/`, its own `env/`. Root `scripts/` hold only tooling that
names no app and fails without being told which app. Fork a root script
per app rather than let one app's tooling change break another.

**3.6 Docs follow scope.** Wadhu-wide ideas live once in root `docs/`;
each app or crate documents only itself and links to the root
([`README.md`](README.md) § Documentation system).

```text
✅ apps/print-bridge → crates/escpos           (app uses a crate)
✅ Kumkaj's apps/khaynjo → wadhu crates/escpos  (another brand uses a crate)
❌ apps/print-bridge → apps/label-maker        (app → sibling app)
❌ crates/escpos-widgets → crates/escpos       (crate → sibling crate: use a feature instead)
❌ crates/escpos → path = "../../vendor/makepad/draw"   (crate bypassing the workspace dep)
```

## 4. What belongs in Wadhu (and what does not)

**Belongs:** code that is **generic** and **brand-neutral**. It would make
sense in a crate on crates.io, and more than one app (or an app nobody at
Gotigin has written yet) could reasonably use it. Examples: device and
protocol clients, file formats, LAN plumbing, image and text utilities,
reusable Makepad 2 widgets, helper programs other apps launch.

**Does not belong:** any brand's **product or business logic**. A
restaurant's KOT rules, a courier's AWB flow, Kumkaj's books engine: those
stay in the product app that owns them. Kumkaj's doctrine ("no Kumkaj
core; each app owns its product code") is fully compatible with Wadhu,
because a Wadhu crate is consumed like any other third-party library, never
as a home for one brand's shared product code. If a proposed crate only
makes sense for one product, it belongs in that product.

**Test before adding:** *Could an unrelated developer use this, from its
README alone, without learning a Gotigin product?* If not, it is not Wadhu
code yet.

## 5. Stability: reuse without ripple

Reuse only works if a change made for one consumer does not surprise the
others:

- **Each crate is versioned on its own** ([SemVer](https://semver.org/)),
  with a `CHANGELOG.md`. A breaking change bumps the major version (the
  minor while `0.x`) and is listed in the changelog. Consumers move when
  they choose to.
- **A consumer's need never becomes a silent behaviour change.** It comes
  in as new API, a new option with the old default, or a new major version.
- **Apps expose stable boundaries** (CLI flags, file formats, network
  protocols) and version them as deliberately as crate APIs.

## 6. Platform

- **Rust**, edition 2024, one Cargo workspace.
- **Makepad 2 only** (`script_mod!` / `AppMain`; never Makepad 1
  `live_design!` docs or APIs), vendored at `vendor/makepad` from a pinned
  upstream commit plus `patches/makepad/`, **the same pin and patch set as
  the other Gotigin workspaces**. A Wadhu crate is then built against the
  same Makepad wherever it is used ([`DEV.md`](DEV.md) § Makepad 2).
- Plain-logic crates stay **Makepad-free**, so non-UI consumers (servers,
  CLIs, tests) never pull in a GUI toolkit.

## 7. Gotigin product apps (not Wadhu’s job — still for agents)

Brand products (e.g. Kumkaj’s Moklanjo / Khaynjo) implement features **for
many client setups**, but must **not** become hardcoded to one client.
Templates, Settings, and rights configure what each shop sees. Wadhu does
not own those packs; agents in this workspace must still not push
one-client hardcoding into consumers or into generic crates. Short note:
[`GOTIGIN_CONFIGURE_NOT_HARDCODE.md`](GOTIGIN_CONFIGURE_NOT_HARDCODE.md).
Cursor: `.cursor/rules/configure-not-hardcode-clients.mdc`.

## 8. The central idea

> **Write it once, generically, in Wadhu. Every app, at Gotigin or
> elsewhere, can take exactly the piece it needs, and no piece ever
> drags a sibling along with it.**
