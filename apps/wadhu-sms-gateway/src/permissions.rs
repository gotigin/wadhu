//! Android runtime permission helpers for SEND_SMS and Classic Bluetooth.

/// MakepadActivity.checkPermission status codes (not PackageManager ints).
pub const STATUS_NOT_DETERMINED: i32 = 0;
pub const STATUS_GRANTED: i32 = 1;
pub const STATUS_DENIED_CAN_RETRY: i32 = 2;

#[cfg(target_os = "android")]
const SEND_SMS: &str = "android.permission.SEND_SMS";

#[cfg(target_os = "android")]
const BT_CONNECT: &str = "android.permission.BLUETOOTH_CONNECT";

#[cfg(target_os = "android")]
const BT_ADVERTISE: &str = "android.permission.BLUETOOTH_ADVERTISE";

#[cfg(target_os = "android")]
const REQ_SMS: i32 = 71001;

#[cfg(target_os = "android")]
fn granted(permission: &str) -> bool {
    // See STATUS_* above. Treating NOT_DETERMINED (0) as granted left Do Next
    // stuck on "Allow SMS" after a real grant (Pixel_7 emulator, 2026-11-22).
    unsafe {
        makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_check_permission(
            permission,
        ) == STATUS_GRANTED
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makepad_permission_status_codes_are_documented() {
        // Regression lock: PackageManager.PERMISSION_GRANTED is 0; Makepad's
        // granted status is 1. Do not "simplify" these back to PM ints.
        assert_eq!(STATUS_NOT_DETERMINED, 0);
        assert_eq!(STATUS_GRANTED, 1);
        assert_eq!(STATUS_DENIED_CAN_RETRY, 2);
        assert_ne!(STATUS_GRANTED, STATUS_NOT_DETERMINED);
    }
}

/// Whether SEND_SMS is granted (false on non-Android).
pub fn sms_permission_granted() -> bool {
    #[cfg(target_os = "android")]
    {
        granted(SEND_SMS)
    }
    #[cfg(not(target_os = "android"))]
    {
        false
    }
}

/// Whether Classic BT server permissions are granted (API 31+ CONNECT + ADVERTISE).
pub fn bluetooth_permission_granted() -> bool {
    #[cfg(target_os = "android")]
    {
        granted(BT_CONNECT) && granted(BT_ADVERTISE)
    }
    #[cfg(not(target_os = "android"))]
    {
        true
    }
}

/// Request SEND_SMS. Bluetooth CONNECT/ADVERTISE/SCAN are batch-requested from
/// Java `btRfcommStart` when the RFCOMM accept loop starts or retries.
pub fn request_sms_permission() {
    #[cfg(target_os = "android")]
    {
        unsafe {
            makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_request_permission(
                SEND_SMS, REQ_SMS,
            );
        }
    }
}

/// Alias used by Bluetooth-focused UI copy.
pub fn request_gateway_permissions() {
    request_sms_permission();
}
