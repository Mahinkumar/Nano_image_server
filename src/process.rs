use crate::utils::{decoder, encoder};
use image::ImageFormat;

pub fn invert(image_bytes: Vec<u8>, img_format: ImageFormat) -> Vec<u8> {
    let mut decoded = decoder(image_bytes);
    decoded.invert();
    encoder(decoded, img_format)
}

pub fn unsharpen(
    image_bytes: Vec<u8>,
    img_format: ImageFormat,
    sigma: f32,
    threshold: f32,
) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let unsharpened = decoded.unsharpen(sigma, threshold as i32);
    encoder(unsharpened, img_format)
}
