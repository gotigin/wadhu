use wadhu_sms_client::{AcceptRequest, Client, ClientError};

#[test]
fn client_keeps_base_url() {
    let c = Client::new("http://127.0.0.1:18765/", "tok");
    assert_eq!(c.base_url(), "http://127.0.0.1:18765");
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
}
