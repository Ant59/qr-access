use hmac::Mac;
use qr_common::{create_hmac, get_secret_from_env};
use std::time::Duration;

pub(crate) fn new(user: &str, lifetime: Duration) -> String {
    let time = std::time::SystemTime::now();
    let expiry = time + lifetime;
    let payload = format!(
        "{}:{}",
        user,
        expiry
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let payload_bytes = payload.as_bytes();

    let secret = get_secret_from_env();
    let mut mac = create_hmac(&secret);
    mac.update(payload_bytes);
    let hmac_bytes = mac.finalize().into_bytes();
    let hmac_string = base85::encode(&hmac_bytes);

    format!("{}:{}", hmac_string, payload)
}
