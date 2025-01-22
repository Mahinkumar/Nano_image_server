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
    let final_filter = choose_resize_filter(filter, x, y, decoded.height(), decoded.width());

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
    if transform_param == 90 {
        rotated = decoded.rotate90();
    } else if transform_param == 180 {
        rotated = decoded.rotate180();
    } else if transform_param == 270 {
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

fn choose_resize_filter(
    filter: &str,
    x: u32,
    y: u32,
    x_original: u32,
    y_original: u32,
) -> image::imageops::FilterType {
    match filter {
        "nearest" => return image::imageops::FilterType::Nearest,
        "triangle" => return image::imageops::FilterType::Triangle,
        "catmullrom" => return image::imageops::FilterType::CatmullRom,
        "gaussian" => return image::imageops::FilterType::Gaussian,
        "lanczos" => return image::imageops::FilterType::Lanczos3,
        "optimal" => {
            let scale_x = x as f32 / x_original as f32;
            let scale_y = y as f32 / y_original as f32;
            let scale = scale_x.min(scale_y);
            let total_pixels = x_original * y_original;

            if scale >= 1.0 {
                if total_pixels > 4_000_000 {
                    image::imageops::FilterType::Triangle
                } else if scale > 2.0 {
                    image::imageops::FilterType::Lanczos3
                } else {
                    image::imageops::FilterType::CatmullRom
                }
            } else {
                if scale < 0.3 {
                    image::imageops::FilterType::Lanczos3
                } else if total_pixels < 1_000_000 {
                    image::imageops::FilterType::CatmullRom
                } else {
                    image::imageops::FilterType::Triangle
                }
            }
        }
        _ => image::imageops::FilterType::Nearest,
    }
}
