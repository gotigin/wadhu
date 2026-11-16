# What Meta App Review videos must show

Meta wants a **screen recording (screencast)** of your product using each
permission — not a logo animation or a slide deck. Aim for 30–90 seconds each.

## 1) `whatsapp_business_messaging`

Show **sending a WhatsApp message end-to-end**:

1. Open **Wadhu Connect** (https://connect.wadhu.com or local `cargo wadhu-connect-dev`)
   and/or Meta **API Setup → Step 1 Try it out** (or Graph API Explorer).
2. Show the test number and allowlisted recipient (`+91 99677 17702`).
3. Click **Send message** (or run the Graph `POST …/messages` call).
4. Show success (`accepted` / message id) **and ideally** the message arriving in
   WhatsApp on the phone (or WhatsApp Web).

That proves the permission is used to deliver customer messages.

## 2) `whatsapp_business_management`

Show **managing WhatsApp business assets**:

1. Open WhatsApp Manager → Message templates **or** Graph API creating a template.
2. Create / open a template (e.g. `wadhu_connect_order_update`) start to finish.
3. Optionally show WABA / phone ID / webhook subscription screens.

That proves the permission is used to manage WABA, phones, templates, webhooks.

## What we already prepared

Rebuilt demos (live Connect UI + Graph panels) in `assets/app-review/`:

- `business_management.mp4` — ES / Login for Business grant → why Business Manager API
- `whatsapp_business_messaging.mp4` — onboard UI → send UI → Graph send panel
- `whatsapp_business_management.mp4` — ES UI → WABA/phone IDs → templates/webhooks

Uses **Wadhu Connect** branding and production IDs (`+91 75740 67702`,
WABA `1897764578101148`). Upload from `apps/wadhu-connect/assets/app-review/`
only. Prefer a real phone + browser recording later if Meta asks.

## Upload where

App Review → submission → **Allowed usage** → each permission’s **Upload file**
→ Save. Agent browser cannot attach files; upload from your normal browser.
