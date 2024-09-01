use hmac::{Hmac, Mac};
use sha3::Sha3_224;

pub type HmacSha3_224 = Hmac<Sha3_224>;

pub fn create_hmac(secret: &[u8]) -> HmacSha3_224 {
    HmacSha3_224::new_from_slice(secret).unwrap()
}
