
use std::io::Cursor;
use crate::utils::decoder;


pub fn blur(image_bytes: Vec<u8>,process_param: f32) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let blurred = decoded.fast_blur(process_param);

    let mut bytes: Vec<u8> = Vec::new();
    blurred
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

pub fn grayscale(image_bytes: Vec<u8>) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let grayscaled = decoded.grayscale();

    let mut bytes: Vec<u8> = Vec::new();
    grayscaled
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

pub fn brighten(image_bytes: Vec<u8>,process_param: f32) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let brightened = decoded.brighten(((process_param*51.0)) as i32);

    let mut bytes: Vec<u8> = Vec::new();
    brightened
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

pub fn contrast(image_bytes: Vec<u8>,process_param: f32) -> Vec<u8>{
    let decoded = decoder(image_bytes);
    let contrasted = decoded.adjust_contrast(process_param);

    let mut bytes: Vec<u8> = Vec::new();
    contrasted
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

