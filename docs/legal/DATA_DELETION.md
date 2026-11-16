# Wadhu data deletion

_Last updated: 16 November 2026_

This page is the **data deletion / account deletion** instructions URL for
Wadhu products (Google Play, Microsoft Store, Meta App Dashboard). Privacy
policy: [wadhu.com/privacy](https://wadhu.com/privacy).

## SMS Gateway (`com.wadhu.smsgateway`)

SMS Gateway is **local-first**. Message history, API tokens, and settings
live on the device (and any relay you configured).

**Delete on-device data**

1. Open Android **Settings → Apps → SMS Gateway → Storage → Clear data**
   (wording varies by OEM), **or**
2. Uninstall SMS Gateway — uninstalling removes the app’s private storage
   for that install.

**Desktop / Windows / Linux packages:** uninstall the app; if a data
directory remains under your user profile for `com.wadhu.smsgateway`, delete
that folder. Windows uninstallers may offer to remove local data — choose
remove when you want a full wipe.

**LAN / Bluetooth clients:** revoke or rotate the gateway API token on the
device before wiping so old clients cannot send.

**SMS Relay:** if you used a self-hosted relay, delete its database / volumes
on your server. If you used a Gotigin-hosted relay, email deletion (below)
and include the relay account or API key id.

## Wadhu Connect

Connect stores WhatsApp Business Account linkage and tokens so brand apps
can use Cloud API after Embedded Signup.

**Request deletion**

Email [mearaj@gotigin.com](mailto:mearaj@gotigin.com) with subject:

`Wadhu data deletion`

Include:

- Your business / WABA id or phone number id if known
- The email or Meta Business you used at signup
- Whether you also want webhook logs removed

We will delete or anonymise Connect connection tokens and associated
operational data for that connection within **30 days**, except records we
must keep for legal / abuse prevention (minimal, time-limited).

You should also remove the Gotigin / Wadhu Connect app’s access from your
**Meta Business settings** so new tokens are not issued.

## Brand site / support mail

Email in our support inbox is handled under ordinary mailbox retention.
Ask us to delete a specific support thread in the same deletion email.

## Meta “User data deletion” callback

For Meta App Review, the Data deletion instructions URL is:

**https://wadhu.com/data-deletion**

(this page). Status of a request will be confirmed by email reply.

## Contact

GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED  
[mearaj@gotigin.com](mailto:mearaj@gotigin.com)  
Gosiya Manzil, Darbargadh, Dhoraji, Rajkot, Gujarat 360410, India
