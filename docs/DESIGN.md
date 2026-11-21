# Wadhu design — brand tokens

Scope: **Wadhu's own surfaces only**, meaning Wadhu apps, the template,
crate demos and a future Wadhu site. Reusable crates stay brand-neutral
([`CRATES.md`](CRATES.md) § Independence): a widget crate exposes its
colours as properties and lets the consuming app style them.

Master: [`assets/images/wadhu-brand-logo.png`](../assets/images/wadhu-brand-logo.png)
(W mark, wordmark, "TOOLS • APPS • MORE", "BY GOTIGIN"). Values below were
sampled from it on 2026-11-08 and are the **default Wadhu palette** until
a fuller design system is written. Change them deliberately and update
this file in the same commit.

## Colour

| Token | Hex | Where in the mark | Use |
|---|---|---|---|
| `WadhuNavy` | `#0B1F63` | ground, top-left | Chrome, headers |
| `WadhuIndigo` | `#161252` | ground, centre | Raised surfaces, chips |
| `WadhuNight` | `#0D0B3A` | ground, deep | Page background |
| `WadhuCyan` | `#00C8FC` | W, left stroke | Primary accent, links, status |
| `WadhuBlue` | `#0133AD` | W, left shadow | Pressed / selected states |
| `WadhuViolet` | `#D967FB` | W, middle stroke | Secondary accent |
| `WadhuGreen` | `#0FCA65` | W, right stroke | Success |
| `WadhuLime` | `#ADFB15` | "+" sign | Highlight, call-to-action, tagline |
| `WadhuWhite` | `#FCFBFB` | wordmark | Primary text on dark |
| `WadhuMuted` | `#9FB4D0` | (derived) | Secondary text on dark |

The brand gradient runs **cyan → violet → green/lime**, left to right, as
in the W. Use it for the mark and hero moments, never behind body text.

In Makepad 2 script: `SolidView { draw_bg +: {color: #x0d0b3a} }`,
`Label { draw_text.color: #xadfb15 }`. A plain `View` with `show_bg`
paints nothing on this Makepad rev ([`DEV.md`](DEV.md) § Agent notes).

## Type and voice

- The wordmark is a heavy, rounded geometric sans; the taglines are wide
  tracked caps. Until a typeface is licensed and vendored, Wadhu UIs use
  Makepad's default UI font.
- The name **Wadhu** is Memon for *more*. Tagline: **Tools · Apps · More — by Gotigin**.
  "Wadhu" is always capitalised and never translated. The brand is reusable
  tools, utils, and apps. SMS Gateway, SMS Relay, and Wadhu Connect are
  products under it.

## Always responsive (every Wadhu app **and** Makepad website)

Applies to **SMS Gateway, SMS Relay, Wadhu Connect**, template apps,
demos, and wadhu.com — not “sites only.” Layouts must fit the window
they are in — no cropped columns, no controls past the right edge, no
“designed for one width” grids.

**Responsive ≠ “looks OK on phone, stays phone on desk.”** Fitting a
narrow viewport is half the job. The other half is **graceful growth**:
when the content column has horizontal room, Fit chips and short paired
controls that can sit on one line **must** sit on one line. Empty white
space beside a forever-stacked toolbar on a 1280+ window is a bug — not
“safe responsiveness.” Agents that only verify a phone screenshot and
leave the desk layout as multi-row Fit stacks are wrong.

| Do | Don't |
|---|---|
| Stack / wrap **only** when measured width cannot hold the Fit children on one row | Hard-code `flow: Down` for a Fit chip toolbar because phone once broke |
| `apply_responsive` unlocks to `Right` / side-by-side once wide enough | Ship a phone stack unchanged at every width |
| Check **320–390 and 1280 / 1920** | Ship after phone-only screenshots |

**Exceptions (judge intent):** graceful growth targets chrome / Fit
toolbars that waste horizontal space — not a license to flatten every
stack on a wide window. User-owned columns-per-row / density is one clear
class (do not force more columns; Auto/Fit may still grow when unset).
Other intentional stacks are mostly obvious; still take care — if unsure,
leave it and check owner intent.

Same law as Kumkaj `docs/DESIGN.md` § Always responsive — keep this file
in sync when the rule tightens. Site checks: [`WEBSITES.md`](WEBSITES.md).

## Icons

Default window and taskbar icons are generated from the master into
[`assets/icons/`](../assets/icons/) ([`DEV.md`](DEV.md) § Icons). An app
replaces them with its own mark under `apps/<app>/assets/app-icon.png`.
