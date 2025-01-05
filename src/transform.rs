use std::io::Cursor;
use image::DynamicImage;

use crate::utils::decoder;

pub fn resizer(image_bytes: Vec<u8>,x: u32 ,y: u32 ,filter: &str) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let final_filter = choose_resize_filter(filter);

    let resized = decoded.resize(x, y, final_filter);

    let mut bytes: Vec<u8> = Vec::new();
    resized
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");

    bytes
}


pub fn flip_horizontal(image_bytes: Vec<u8>) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let flipped_h = decoded.fliph();

    let mut bytes: Vec<u8> = Vec::new();
    flipped_h
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

pub fn flip_vertical(image_bytes: Vec<u8>) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let flipped_v = decoded.flipv();

    let mut bytes: Vec<u8> = Vec::new();
    flipped_v
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

pub fn rotate(image_bytes: Vec<u8>,transform_param: i32) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let rotated: DynamicImage;
    if transform_param == 1 {
        rotated = decoded.rotate90();
    }else if transform_param == 2 {
        rotated = decoded.rotate180();
    }else if transform_param == 3 {
        rotated = decoded.rotate270();
    }else{
        rotated = decoded
    }
    
    let mut bytes: Vec<u8> = Vec::new();
    rotated
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

pub fn hue_rotate(image_bytes: Vec<u8>,transform_param: i32) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let hue_rotated = decoded.huerotate(transform_param);

    let mut bytes: Vec<u8> = Vec::new();
    hue_rotated
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
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
