use base64::{Engine, prelude::BASE64_STANDARD};
use image::Luma;
use leptos::prelude::*;
use leptos::*;

#[component]
pub fn QRCode(url: String) -> impl IntoView {
    let code = qrcode::QrCode::new(url.as_bytes()).expect("Should not fail..?");

    let rendered_image = code.render::<Luma<u8>>().build();

    let mut image_byte_buffer = ByteBuffer::new();
    rendered_image
        .write_to(&mut image_byte_buffer, image::ImageFormat::Png)
        .unwrap_or_else(|err| {
            logging::error!("{err:?}");
        });

    view! {
        <img
            src={format!("data:image/png;base64, {}", image_byte_buffer.into_base64_string()) }
            alt={format!("{} as QR Code", url)}
        />
    }
}

struct ByteBuffer {
    buf: Vec<u8>,
    pos: usize,
}

impl ByteBuffer {
    fn new() -> Self {
        Self {
            buf: vec![],
            pos: 0,
        }
    }

    // fn new_with_capacity(capacity: usize) -> Self {
    //     Self {
    //         buf: Vec::<u8>::with_capacity(capacity),
    //         pos: 0,
    //     }
    // }

    fn into_base64_string(self) -> String {
        let mut buffer = String::with_capacity(4 * self.buf.len() / 3);
        BASE64_STANDARD.encode_string(self.buf, &mut buffer);
        buffer
    }
}

impl std::io::Write for ByteBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.pos + buf.len() > self.buf.len() {
            self.buf.resize(self.pos + buf.len(), 0);
        }

        self.buf[self.pos..self.pos + buf.len()].copy_from_slice(buf);
        self.pos += buf.len();

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::io::Seek for ByteBuffer {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let pos = match pos {
            std::io::SeekFrom::Start(pos) => pos as i64,
            std::io::SeekFrom::End(pos) => self.buf.len() as i64 + pos,
            std::io::SeekFrom::Current(pos) => pos,
        };

        if pos < 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid seek position",
            ));
        }

        Ok(pos as u64)
    }
}
