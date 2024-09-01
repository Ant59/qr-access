use hmac::Mac;
use qr_common::{create_hmac, get_secret_from_env};

pub(crate) fn new(user: &str, expiry: &u64) -> String {
    let payload = format!("{}:{}", user, expiry);
    let payload_bytes = payload.as_bytes();

    let secret = get_secret_from_env();
    let mut mac = create_hmac(&secret);
    mac.update(payload_bytes);
    let hmac_bytes = mac.finalize().into_bytes();
    let hmac_string = base85::encode(&hmac_bytes);

    format!("{}:{}", hmac_string, payload)
}
