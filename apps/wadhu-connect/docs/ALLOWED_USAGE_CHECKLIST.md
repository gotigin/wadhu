# Allowed usage — what’s left before Next unlocks

From your screenshot (messaging video done):

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
