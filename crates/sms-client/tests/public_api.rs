use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use wadhu_sms_client::framed::{FramedClient, DEFAULT_FRAMED_TCP_PORT};
use wadhu_sms_client::{AcceptRequest, Client, ClientError, Message};
use wadhu_sms_client::discover::DISCOVER_PORT;

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
    assert!(e.user_message().contains("Bad URL"));
    let u = ClientError::Unsupported("sync");
    assert!(u.user_message().contains("Not supported"));
    let io = ClientError::Io("Connection refused (os error 111)".into());
    assert!(io.user_message().contains("Cannot reach gateway"));
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

#[test]
fn framed_tcp_ping_and_send_without_cloud() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind lab");
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept");
        // ping
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).unwrap();
        let n = u32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; n];
        stream.read_exact(&mut body).unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("ping"));
        let reply = br#"{"ok":true,"pong":true}"#;
        let len = (reply.len() as u32).to_be_bytes();
        stream.write_all(&len).unwrap();
        stream.write_all(reply).unwrap();
        // send
        stream.read_exact(&mut len_buf).unwrap();
        let n = u32::from_be_bytes(len_buf) as usize;
        let mut body = vec![0u8; n];
        stream.read_exact(&mut body).unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains("send"));
        assert!(s.contains("lab-tok"));
        let reply = br#"{"ok":true,"message":{"id":"msg-lab","status":"queued","to":"+1","idempotency_key":""}}"#;
        let len = (reply.len() as u32).to_be_bytes();
        stream.write_all(&len).unwrap();
        stream.write_all(reply).unwrap();
    });

    let mut c = FramedClient::connect_tcp("127.0.0.1", port, "lab-tok").expect("connect");
    c.ping().expect("ping");
    let reply = c.send("+1", "hi", None).expect("send");
    assert!(reply.contains("msg-lab"));
    assert_eq!(DEFAULT_FRAMED_TCP_PORT, 18766);
    assert_eq!(DISCOVER_PORT, 18767);
}
