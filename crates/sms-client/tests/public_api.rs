use wadhu_sms_client::{AcceptRequest, Client, ClientError, Message};

#[test]
fn client_keeps_base_url() {
    let c = Client::new("http://127.0.0.1:18765/", "tok");
    assert_eq!(c.base_url(), "http://127.0.0.1:18765");
    assert_eq!(c.token(), "tok");
}

#[test]
fn accept_request_is_constructible() {
    let r = AcceptRequest {
        to: "+1".into(),
        body: "x".into(),
        idempotency_key: None,
    };
    assert_eq!(r.to, "+1");
}

#[test]
fn error_is_displayable() {
    let e = ClientError::BadUrl("nope".into());
    assert!(e.to_string().contains("bad url"));
    let u = ClientError::Unsupported("sync");
    assert!(u.to_string().contains("unsupported"));
}

#[test]
fn message_has_optional_error() {
    let m = Message {
        id: "msg-1".into(),
        status: "failed".into(),
        to: "+1".into(),
        error: Some("radio".into()),
    };
    assert_eq!(m.error.as_deref(), Some("radio"));
}
