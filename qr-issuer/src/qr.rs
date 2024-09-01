use std::io::Cursor;

use image::{ImageFormat, Luma};
use qrcode::QrCode;

pub(crate) fn create_qr_png(token: &str) -> Vec<u8> {
    let mut qr_image = Cursor::new(Vec::new());
    QrCode::new(&token)
        .unwrap()
        .render::<Luma<u8>>()
        .build()
        .write_to(&mut qr_image, ImageFormat::Png)
        .unwrap();
    qr_image.into_inner()
}
