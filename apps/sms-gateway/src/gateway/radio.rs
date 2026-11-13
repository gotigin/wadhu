//! Radio backends the gateway uses to place SMS on the mobile network.

/// What a radio can do — advertised on `GET /v1/capabilities`.
#[derive(Clone, Debug)]
pub struct RadioCapabilities {
    pub kind: &'static str,
    pub label: String,
    pub available: bool,
    pub detail: String,
    pub supports_send: bool,
    pub supports_multipart: bool,
}

/// Physical or virtual path that can submit SMS.
pub trait SmsRadio: Send + Sync {
    fn capabilities(&self) -> RadioCapabilities;
    fn send(&self, to: &str, body: &str) -> Result<(), String>;
}

/// Desktop placeholder until USB modem support lands.
pub struct DesktopNoRadio;

impl SmsRadio for DesktopNoRadio {
    fn capabilities(&self) -> RadioCapabilities {
        RadioCapabilities {
            kind: "none",
            label: "No SIM on desktop".into(),
            available: false,
            detail: "Android-first app — run on a phone/tablet with a SIM.".into(),
            supports_send: false,
            supports_multipart: false,
        }
    }

    fn send(&self, _to: &str, _body: &str) -> Result<(), String> {
        Err(self.capabilities().detail)
    }
}

/// Android handset SIM via MakepadActivity JNI.
pub struct AndroidSimRadio;

impl SmsRadio for AndroidSimRadio {
    fn capabilities(&self) -> RadioCapabilities {
        #[cfg(target_os = "android")]
        {
            let detail = unsafe {
                makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_sms_radio_status(
                )
            };
            let available = detail == "ready";
            RadioCapabilities {
                kind: "android_sim",
                label: "Android SIM".into(),
                available,
                detail,
                supports_send: true,
                supports_multipart: true,
            }
        }
        #[cfg(not(target_os = "android"))]
        {
            RadioCapabilities {
                kind: "android_sim",
                label: "Android SIM".into(),
                available: false,
                detail: "Not running on Android".into(),
                supports_send: true,
                supports_multipart: true,
            }
        }
    }

    fn send(&self, to: &str, body: &str) -> Result<(), String> {
        #[cfg(target_os = "android")]
        {
            let caps = self.capabilities();
            if !caps.available {
                return Err(format!("Android SIM not ready: {}", caps.detail));
            }
            let ok = unsafe {
                makepad_widgets::makepad_platform::os::linux::android::android_jni::to_java_send_sms(
                    to.to_string(),
                    body.to_string(),
                )
            };
            if ok {
                Ok(())
            } else {
                Err("SmsManager refused the send (permission or radio error)".into())
            }
        }
        #[cfg(not(target_os = "android"))]
        {
            let _ = (to, body);
            Err("Android SIM radio is only available on Android builds".into())
        }
    }
}

/// Pick the radio for this process.
pub fn default_radio() -> Box<dyn SmsRadio> {
    #[cfg(target_os = "android")]
    {
        Box::new(AndroidSimRadio)
    }
    #[cfg(not(target_os = "android"))]
    {
        Box::new(DesktopNoRadio)
    }
}
