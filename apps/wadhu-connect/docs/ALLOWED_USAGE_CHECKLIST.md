# Allowed usage — what’s left before Next unlocks

## 0) `business_management` — 1 API test call required

Meta tracks this from **Review → Testing** (not random curl alone).

1. On the Allowed usage form: **Save** description + `business_management.mp4` + Agree.
2. Sidebar **Review → Testing** (or the “go to Testing” link on that form).
3. **Generate token** / Graph API Explorer for this app, include permission
   `business_management`.
4. Run one call, e.g.  
   `GET https://graph.facebook.com/v22.0/me/businesses`  
   or  
   `GET https://graph.facebook.com/v22.0/1076913081550690?fields=id,name`
5. Return to Allowed usage — counter can take minutes–**24h** to show `1 of 1`.

Do **not** wait on the form with `0 of 1` forever; save and leave after the call.

## 1) `whatsapp_business_messaging` — only API test calls left

Meta needs a tracked call from **their** Try it out UI (curl alone often doesn’t tick this).

1. Open: https://developers.facebook.com/apps/1606021637403895/use_cases/customize/api-testing-v2/?product_route=whatsapp-business&business_id=1076913081550690&use_case_enum=WHATSAPP_BUSINESS_MESSAGING&selected_tab=api-testing-v2
2. **Generate token**
3. **To** → select `+91 99677 17702`
4. Click **Send message**
5. Return to Allowed usage → messaging **Get started** — wait until it shows `1 of 1` API calls (can take minutes–24h)

## 2) `whatsapp_business_management` — all four items

1. Business description (one line), then **Get started**
2. Paste the long description (see `APP_REVIEW_ANSWERS.md`)
3. Upload: `/home/mearaj/Downloads/whatsapp_business_management.mp4`
4. Tick agree → **Save**
5. Same API-test wait if Meta lists required management calls

## 3) `public_profile`

**Get started** → tick agree → **Save**

Only then **Next** enables.
