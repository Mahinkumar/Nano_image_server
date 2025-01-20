use image::{DynamicImage, ImageFormat};

use crate::utils::{decoder, encoder};

pub fn resizer(
    image_bytes: Vec<u8>,
    img_format: ImageFormat,
    x: u32,
    y: u32,
    filter: &str,
) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let final_filter = choose_resize_filter(filter);

    let resized = decoded.resize(x, y, final_filter);

    encoder(resized, img_format)
}

pub fn flip_horizontal(image_bytes: Vec<u8>, img_format: ImageFormat) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let flipped_h = decoded.fliph();

    encoder(flipped_h, img_format)
}

pub fn flip_vertical(image_bytes: Vec<u8>, img_format: ImageFormat) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let flipped_v = decoded.flipv();

    encoder(flipped_v, img_format)
}

pub fn rotate(image_bytes: Vec<u8>, img_format: ImageFormat, transform_param: i32) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let rotated: DynamicImage;
    if transform_param == 1 {
        rotated = decoded.rotate90();
    } else if transform_param == 2 {
        rotated = decoded.rotate180();
    } else if transform_param == 3 {
        rotated = decoded.rotate270();
    } else {
        rotated = decoded
    }

    encoder(rotated, img_format)
}

pub fn hue_rotate(image_bytes: Vec<u8>, img_format: ImageFormat, transform_param: i32) -> Vec<u8> {
    let decoded = decoder(image_bytes);
    let hue_rotated = decoded.huerotate(transform_param);

    encoder(hue_rotated, img_format)
}

fn choose_resize_filter(filter: &str) -> image::imageops::FilterType {
    //For now we choose the Nearest resize filter implicitly.
    match filter {
        "nearest" => return image::imageops::FilterType::Nearest,
        "triangle" => return image::imageops::FilterType::Triangle,
        "catmullrom" => return image::imageops::FilterType::CatmullRom,
        "gaussian" => return image::imageops::FilterType::Gaussian,
        "lanczos" => return image::imageops::FilterType::Lanczos3,
        _ => return image::imageops::FilterType::Nearest,
    }
}
