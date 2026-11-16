# Allowed usage — App Review status

## Submitted 2026-11-16

Receipt:
[`assets/app-review/Meta_App_Review_Submitted_2026-11-16.pdf`](../assets/app-review/Meta_App_Review_Submitted_2026-11-16.pdf).

| Permission | In this submission |
| --- | --- |
| `whatsapp_business_messaging` | Yes |
| `whatsapp_business_management` | Yes |
| `public_profile` | Yes |
| `pages_show_list` | Yes (already checked; kept) |
| `email` | Yes |
| `business_management` | **No** — deferred |
| `pages_read_engagement` | **No** — deferred |

Why deferred + plan to integrate after approval:
[`APP_REVIEW_ANSWERS.md`](APP_REVIEW_ANSWERS.md) § Deferred.

## After approval (follow-up)

1. Add `business_management` + `pages_read_engagement` only if product needs
   portfolio/Page Graph beyond Embedded Signup session fields.
2. Allowed usage text + videos + Agree; Review → Testing until counters move
   (or Meta Ask-a-question if 200s still show 0).
3. New App Review request for those two (do not block WhatsApp Advanced Access
   on them again).

## Historical notes (WhatsApp testing)

WhatsApp Try-it-out path (for messaging) used Cloud API `+91 75740 67702`,
never the personal WhatsApp Business app number.

Connect can still run Graph probes after ES via
`POST /api/ops/graph-precheck` (`X-Wadhu-Ops-Token`) when a connection token
exists — useful for a future BM/Pages follow-up, not required for the current
WhatsApp-only submission.
