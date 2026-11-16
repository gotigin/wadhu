# App Review — permission descriptions (Wadhu Connect)

Reviewer / Data handling paste: [`DATA_HANDLING_AND_REVIEWER.md`](DATA_HANDLING_AND_REVIEWER.md).

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

## business_management

We use business_management with Facebook Login for Business / Embedded Signup so
a client business can grant Wadhu Connect access to their Meta Business
portfolio and related assets. After login we read the granted business identity
and asset links needed to finish WhatsApp Cloud API onboarding (associate the
client business with the WABA / phone handoff). We do not use it for ads, Pages
publishing, or consumer Facebook profile features.

This adds value because the person onboards in one guided flow: grant business
access, keep their number, and connect Cloud API to their software without
manual Business Manager wiring.

It is necessary for Tech Provider client-business onboarding: without
business_management Connect cannot identify the granting business portfolio and
complete asset association for WhatsApp Embedded Signup.

## Coexistence

Production goal is coexistence (`featureType=whatsapp_business_app_onboarding`)
for +91 99677 17702. Demo / review uses Meta test number +1 555-156-0957 until
advanced access is approved.
