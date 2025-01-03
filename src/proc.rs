use std::io::Cursor;
use image::ImageReader;


pub fn blur(image_bytes: Vec<u8>,process_param: f32) -> Vec<u8>{
    let decoded = ImageReader::new(Cursor::new(image_bytes))
                .with_guessed_format()
                .expect("Unable to find format")
                .decode()
                .expect("Unable to decode");
    let blurred = decoded.fast_blur(process_param);

    let mut bytes: Vec<u8> = Vec::new();
    blurred
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}

pub fn grayscale(image_bytes: Vec<u8>) -> Vec<u8>{
    let decoded = ImageReader::new(Cursor::new(image_bytes))
                .with_guessed_format()
                .expect("Unable to find format")
                .decode()
                .expect("Unable to decode");
    let blurred = decoded.grayscale();

    let mut bytes: Vec<u8> = Vec::new();
    blurred
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}