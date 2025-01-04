use std::io::Cursor;
use image::{DynamicImage, ImageReader};

pub fn decoder(image_bytes: Vec<u8>)->DynamicImage{
    let decoded = ImageReader::new(Cursor::new(image_bytes))
                .with_guessed_format()
                .expect("Unable to find format")
                .decode()
                .expect("Unable to decode");
    return decoded;
}