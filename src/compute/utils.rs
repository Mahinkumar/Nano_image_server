use image::{DynamicImage, ImageReader};

use std::io::Cursor;

pub fn decoder(image_bytes: Vec<u8>) -> DynamicImage {
    let decoded = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .expect("Unable to find format")
        .decode()
        .expect("Unable to decode");
    return decoded;
}

pub fn encoder(image: DynamicImage, format: image::ImageFormat) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), format)
        .expect("Unable to write");
    bytes
}
