# Wadhu data deletion

_Last updated: 20 November 2026_

Wadhu (*more* in Memon — *Tools · Apps · More*) is the brand. This page
says how to delete your data, or your account, for each product under it.
Privacy: [wadhu.com/privacy](https://wadhu.com/privacy) ·
Cancellation: [wadhu.com/cancellation](https://wadhu.com/cancellation) ·
Support: [wadhu.com/support](https://wadhu.com/support).

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

Email [support@gotigin.com](mailto:support@gotigin.com) with subject:

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

## wadhu.com account

If you registered on [wadhu.com/account](https://wadhu.com/account), email
[support@gotigin.com](mailto:support@gotigin.com) with subject
`Wadhu data deletion` from the address on the account (or name the phone
number you signed in with). We delete the account and its sign-in record
within **30 days**.

## Brand site / support mail

Email in our support inbox is handled under ordinary mailbox retention.
Ask us to delete a specific support thread in the same deletion email.

We confirm every deletion request by email reply when it is done.

## Contact

GOTIGIN SOFTWARE & HARDWARE PRIVATE LIMITED  
[support@gotigin.com](mailto:support@gotigin.com)  
Gosiya Manzil, Darbargadh, Dhoraji, Rajkot, Gujarat 360410, India
