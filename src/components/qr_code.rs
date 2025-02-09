use std::io::Cursor;

use base64::{Engine, prelude::BASE64_STANDARD};
use image::Luma;
use leptos::prelude::*;
use leptos::*;

#[component]
pub fn QRCode(url: String) -> impl IntoView {
    let code = qrcode::QrCode::new(url.as_bytes()).expect("Should not fail..?");

    let rendered_image = code.render::<Luma<u8>>().build();

    let mut image_byte_buffer = Cursor::new(Vec::<u8>::new());
    rendered_image
        .write_to(&mut image_byte_buffer, image::ImageFormat::Png)
        .unwrap_or_else(|err| {
            logging::error!("{err:?}");
        });

    let base64_string = BASE64_STANDARD.encode(image_byte_buffer.get_ref());

    view! {
        <img
            src={format!("data:image/png;base64, {}", base64_string) }
            alt={format!("{} as QR Code", url)}
        />
    }
}
