//! Android runtime permission helpers for SEND_SMS and Classic Bluetooth.

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
    // PackageManager.PERMISSION_GRANTED == 0
    unsafe {
        makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_check_permission(
            permission,
        ) == 0
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
