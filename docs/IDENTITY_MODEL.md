# Identity model — user, org, grouping

Canonical definitions so these words stay distinct. Monetization timing:
[`IDENTITY_AND_MONETIZATION_ALIGNMENT.md`](IDENTITY_AND_MONETIZATION_ALIGNMENT.md).
Naming (Trademark → Company → Brand → Product): [`BRAND.md`](BRAND.md).

------------------------------------------------------------------------

## User (person)

A **user** is a person who registers on wadhu.com.

- Login today: Firebase Auth (adapter only). See [`FIREBASE_AUTH_SETUP.md`](FIREBASE_AUTH_SETUP.md).
- Wadhu owns a stable **`user_id`** in our DB (when built). **Firebase UID is
  not** the billing foreign key.
- **Same person across apps** (wadhu.com, Connect, Moklanjo, Khaynjo, …) means
  the same Wadhu `user_id` — not an org.

Registration creates a **user**. It does not require an org in the UI.

------------------------------------------------------------------------

## Org (Wadhu platform workspace)

An **org** is a Wadhu cloud workspace used only where isolation is required:

1. **Same cloud product bought more than once** under one login (e.g. two
   Connect WABAs / two hosted-relay subscriptions).
2. **More than one person** on the same paid cloud thing (owner + staff).
3. **Scoped API keys / entitlements** that must not be “forever personal” or
   baked into brand APKs.
4. **B2B billing** (invoice / Stripe customer for that business instance).

Solo user with one of each cloud thing: **no org UX required**. A hidden
default org may exist later for billing hooks; it must not confuse signup.

Do **not** use org for: cross-app identity, LAN/BT SMS, or in-product
compare/report of shop locations.

------------------------------------------------------------------------

## Grouping (ideology — usually in brand products)

**Grouping** = same kinds of things can be clustered so they are
**comparable** and **presentable** (side-by-side, rollups, reports).

- It is a **presentation / compare lens**, not the identity of the thing.
- Prefer **dynamic** membership (rules, tags, named sets resolved at read
  time) over a mandatory `group_id` field on every business row.
- In Kumkaj apps this is how an owner views several **separate product
  instances** (e.g. five Khaynjo outlets) without merging their books.
  Prefer UI words like **group** / **outlet group** — **never** call that
  a Wadhu **org**.

Wadhu docs do not invent ProductsGroup / AppsGroup. Brand products own
their grouping UX; see Kumkaj multi-branch doctrine (separate instances +
compare, not one org inside one set of books).

------------------------------------------------------------------------

## Quick map

| Need | Use |
| --- | --- |
| Register / sign-in | **User** |
| Same person in another app | **User** (`user_id`) |
| Two paid Connect numbers | **Org** (or org-scoped entitlement) each |
| Staff on one Connect | **Org** members |
| Compare three restaurant branches | **Grouping** in the product |
| LAN / Bluetooth SMS | Neither (local keys) |

```text
Person (user_id)
 ├── optional Org A — cloud purchase / WABA / keys / members
 ├── optional Org B — second cloud purchase …
 └── (in brand apps) Grouping views over same-type instances — not Wadhu orgs
```

------------------------------------------------------------------------

## Build order (cloud track)

Do not wait on Meta App Review or Apple Sign-In for this spine.

1. Platform API + Postgres **users** (+ identity links).
2. Wire `/account` → Wadhu `user_id`.
3. Connect / site WhatsApp in **test mode**, scoped when a second purchase or
   teammate appears (**org** only then — or default org if implementation
   needs a row).
4. Payments per org / entitlement.
5. Hosted relay entitlements (LAN/BT stay free forever).

SMS OpenAPI stability for brand LAN/BT is a **parallel** track; it is not
a prerequisite for registration or Connect UX.
