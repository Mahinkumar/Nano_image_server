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