//! Android runtime permission helpers for SEND_SMS.

#[cfg(target_os = "android")]
const SEND_SMS: &str = "android.permission.SEND_SMS";

#[cfg(target_os = "android")]
const REQ_SMS: i32 = 71001;

/// Whether SEND_SMS is granted (always true on non-Android for UI simplicity).
pub fn sms_permission_granted() -> bool {
    #[cfg(target_os = "android")]
    {
        // PackageManager.PERMISSION_GRANTED == 0
        unsafe {
            makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_check_permission(
                SEND_SMS,
            ) == 0
        }
    }
    #[cfg(not(target_os = "android"))]
    {
        false
    }
}

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
