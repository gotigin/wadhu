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

We use this permission to send customer-facing WhatsApp messages on behalf of
onboarded Messaging accounts: session replies within the customer care window,
and approved template messages for notifications. Messages are created via
Wadhu Connect / Graph API and delivered to the end user’s WhatsApp client.

## whatsapp_business_management

We use this permission to complete Embedded Signup handoff, read WABA / phone
number IDs, subscribe webhooks, and create/manage WhatsApp message templates for
onboarded accounts.

## Coexistence

Production goal is coexistence (`featureType=whatsapp_business_app_onboarding`)
for +91 99677 17702. Demo / review uses Meta test number +1 555-156-0957 until
advanced access is approved.
