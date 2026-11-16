# App Review — permission descriptions (Wadhu Connect)

Reviewer / Data handling paste: [`DATA_HANDLING_AND_REVIEWER.md`](DATA_HANDLING_AND_REVIEWER.md).

Submission receipt:
[`assets/app-review/Meta_App_Review_Submitted_2026-11-16.pdf`](../assets/app-review/Meta_App_Review_Submitted_2026-11-16.pdf).

| | |
| --- | --- |
| Meta app display name | **Wadhu Connect** |
| Privacy policy | https://wadhu.com/privacy |
| Data deletion | https://wadhu.com/data-deletion |
| Business contact | mearaj@gotigin.com |
| Company | GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED |
| Brand | Wadhu |
| Target coexistence number | +91 99677 17702 |

## App / service description

Wadhu Connect is the WhatsApp Business Platform Tech Provider connector under
the Wadhu brand of GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED. It onboards
businesses onto Cloud API through Embedded Signup, including coexistence so a
number already on the WhatsApp Business app can keep the mobile app while also
using Cloud API. It sends and receives WhatsApp messages, manages message
templates, and processes webhooks.

## whatsapp_business_messaging

We use whatsapp_business_messaging to send and receive customer-facing WhatsApp
messages for businesses onboarded to Cloud API through Wadhu Connect. After
Embedded Signup, Connect calls Graph API to deliver session replies inside the
24-hour customer care window and approved template messages for notifications
(order updates, alerts). Inbound messages arrive on our webhook
(https://connect.wadhu.com/webhook) so the business’s software can respond.

This adds value because the business keeps their WhatsApp number while their
own apps (or Wadhu Connect’s test UI) can message customers reliably over Cloud
API instead of manual phone-only chat.

It is necessary: without this permission Wadhu Connect cannot send or receive
WhatsApp messages for onboarded Messaging accounts, which is the core product
function of this Tech Provider bridge.

## whatsapp_business_management

We use whatsapp_business_management to complete WhatsApp Cloud API onboarding
and operate Messaging accounts connected through Wadhu Connect: read WABA and
phone number IDs after Embedded Signup, subscribe the app to webhooks
(https://connect.wadhu.com/webhook), and create or manage WhatsApp message
templates for the onboarded business.

This adds value by letting the business finish setup once and then manage
templates and account wiring from their software / Connect, without repeating
manual steps in WhatsApp Manager for every change.

It is necessary: without this permission Connect cannot finish handoff, keep
webhooks subscribed, or manage templates for onboarded accounts — required for
a Tech Provider WhatsApp bridge.

## pages_show_list (submitted)

Kept in the 2026-11-16 submission because Meta Already Allowed usage / Testing
had progressed for this permission, and Login for Business may expose managed
Page list metadata during portfolio grant.

Wadhu Connect’s product is WhatsApp Tech Provider onboarding via Embedded
Signup. We do not build a consumer Pages directory, Page picker, or Page
publishing UI. Any Page list access is incidental to Login / portfolio grant,
not a standalone Pages product.

## Deferred — not in this App Review submission

### Why `business_management` was omitted

Meta documents `business_management` as **optional** on the WhatsApp use case
(only needed if the app reads the business portfolio programmatically via the
Business Manager API). Wadhu Connect’s required Tech Provider Advanced Access
is `whatsapp_business_messaging` + `whatsapp_business_management` (+
`public_profile`). Embedded Signup already returns `business_id` / WABA / phone
IDs in the session payload; runtime onboarding uses WhatsApp permissions, not
`GET /me/businesses`.

Review → Testing also stayed at **0 of 1** for `business_management` despite
valid Graph **200** responses (known Meta attribution flake for BM/Pages vs
WhatsApp product tests). Shipping unused permissions risks rejection; we
removed it so this review covers only what the product uses today.

### Why `pages_read_engagement` was omitted

`pages_read_engagement` is for reading Page posts / engagement (or CTWA-style
ads products). Connect does not display Page posts, insights, or Page
moderation. It was only on the app because Meta attaches it as a dependency
when `business_management` / the Pages use case is present. Without a Pages
product and without BM in this submission, it is not needed.

### After this review passes

Once WhatsApp Advanced Access is approved, we will **try to integrate**
`business_management` and `pages_read_engagement` in a later App Review
(Allowed usage + Testing + product wiring), if portfolio/Page Graph access
proves useful beyond Embedded Signup’s session fields. Until then they stay
out of production scope and out of this submission.

## Coexistence

Production goal is coexistence (`featureType=whatsapp_business_app_onboarding`).
Own-business Cloud API number: +91 75740 67702. Personal +91 99677 17702 is not
the Cloud API sender. Meta test numbers may appear in dashboard for review.
Default Connect launch is Cloud API ES; coexistence is `?coexistence=1` after
Meta clears partner biz-app onboarding (#3441107).
