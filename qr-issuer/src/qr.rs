use std::io::Cursor;

use base64::prelude::*;
use image::{ImageFormat, Luma};
use qrcode::QrCode;

pub(crate) fn create_qr_image_html(token: &str) -> String {
    let mut qr_image = Cursor::new(Vec::new());
    QrCode::new(&token)
        .unwrap()
        .render::<Luma<u8>>()
        .build()
        .write_to(&mut qr_image, ImageFormat::Png)
        .unwrap();

    let qr_b64 = BASE64_STANDARD.encode(qr_image.get_ref());
    format!("<img src=\"data:image/png;base64,{}\">", qr_b64)
}
