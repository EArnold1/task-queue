use std::collections::HashMap;

pub fn create_email_payload(to: &str, body: &str, subject: &str) -> HashMap<String, String> {
    let mut payload = HashMap::new();

    payload.insert("to".into(), to.into());
    payload.insert("body".into(), body.into());
    payload.insert("subject".into(), subject.into());

    payload
}

pub fn create_notification_payload(
    msg: &str,
    title: &str,
    fcm_token: Option<&str>,
) -> HashMap<String, String> {
    let mut payload = HashMap::new();

    payload.insert("msg".into(), msg.into());
    payload.insert("title".into(), title.into());

    if let Some(fcm_token) = fcm_token {
        payload.insert("fcm_token".into(), fcm_token.into());
    }

    payload
}
