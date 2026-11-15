# App Review — permission descriptions

Reviewer / Data handling paste lives only in `DATA_HANDLING_AND_REVIEWER.md`.

Privacy policy: https://khaynjo.com/privacy  
Business contact: mearaj@gotigin.com  
Target coexistence number: +91 99677 17702

## App / service description

Gotigin Connect is the WhatsApp Business Platform connector for GOTIGIN SOFTWARE &
HARDWARE PRIVATE LIMITED. It onboards businesses onto Cloud API through Embedded
Signup, including coexistence so a number already on the WhatsApp Business app
can keep the mobile app while also using Cloud API. It sends and receives
WhatsApp messages, manages message templates, and processes webhooks.

## whatsapp_business_messaging

We use this permission to send customer-facing WhatsApp messages on behalf of
onboarded Messaging accounts: session replies within the customer care window,
and approved template messages for notifications. Messages are created in
Gotigin Connect / Graph API and delivered to the end user’s WhatsApp client.

## whatsapp_business_management

We use this permission to complete Embedded Signup handoff, read WABA / phone
number IDs, subscribe webhooks, and create/manage WhatsApp message templates for
onboarded accounts.

## Coexistence

Production goal is coexistence (`featureType=whatsapp_business_app_onboarding`)
for +91 99677 17702. Demo / review uses Meta test number +1 555-156-0957 until
advanced access is approved.
