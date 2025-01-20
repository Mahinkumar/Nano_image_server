use crate::utils::{decoder, encoder};
use image::ImageFormat;

pub fn blur(image_bytes: Vec<u8>, img_format: ImageFormat, process_param: f32) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let blurred = decoded.fast_blur(process_param);

    encoder(blurred, img_format)
}

pub fn grayscale(image_bytes: Vec<u8>, img_format: ImageFormat) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let grayscaled = decoded.grayscale();

    encoder(grayscaled, img_format)
}

pub fn brighten(image_bytes: Vec<u8>, img_format: ImageFormat, process_param: f32) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let brightened = decoded.brighten((process_param * 51.0) as i32);

    encoder(brightened, img_format)
}

pub fn contrast(image_bytes: Vec<u8>, img_format: ImageFormat, process_param: f32) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let contrasted = decoded.adjust_contrast(process_param);

    encoder(contrasted, img_format)
}
