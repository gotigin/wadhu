    // Wadhu SMS Gateway: SmsManager send for local SIM radio.
    // Inserted by scripts/apply-android-activity-bridge.sh — do not edit in vendor.
    // Marker: wadhuSmsPieceEnds

    public boolean sendSms(String to, String body) {
        try {
            if (to == null || to.trim().isEmpty()) {
                Log.e(LOG_TAG, "sendSms: empty destination");
                return false;
            }
            if (body == null) {
                body = "";
            }
            if (checkSelfPermission(android.Manifest.permission.SEND_SMS)
                    != android.content.pm.PackageManager.PERMISSION_GRANTED) {
                Log.e(LOG_TAG, "sendSms: SEND_SMS not granted");
                return false;
            }
            android.telephony.SmsManager sms = android.telephony.SmsManager.getDefault();
            java.util.ArrayList<String> parts = sms.divideMessage(body);
            if (parts == null || parts.size() <= 1) {
                sms.sendTextMessage(to.trim(), null, body, null, null);
            } else {
                sms.sendMultipartTextMessage(to.trim(), null, parts, null, null);
            }
            return true;
        } catch (Exception e) {
            Log.e(LOG_TAG, "sendSms failed", e);
            return false;
        }
    }

    public String smsRadioStatus() {
        try {
            if (checkSelfPermission(android.Manifest.permission.SEND_SMS)
                    != android.content.pm.PackageManager.PERMISSION_GRANTED) {
                return "permission_needed";
            }
            android.telephony.TelephonyManager tm =
                    (android.telephony.TelephonyManager) getSystemService(TELEPHONY_SERVICE);
            if (tm == null) {
                return "no_telephony";
            }
            int state = tm.getSimState();
            if (state != android.telephony.TelephonyManager.SIM_STATE_READY) {
                return "sim_not_ready:" + state;
            }
            return "ready";
        } catch (Exception e) {
            Log.e(LOG_TAG, "smsRadioStatus failed", e);
            return "error";
        }
    }
    // wadhuSmsPieceEnds
