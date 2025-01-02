use std::io::Cursor;

use image::ImageReader;
use crate::ProcessParameters;


pub fn blur(image_bytes: Vec<u8>,process_params: &ProcessParameters) -> Vec<u8>{
    let decoded = ImageReader::new(Cursor::new(image_bytes))
                .with_guessed_format()
                .expect("Unable to find format")
                .decode()
                .expect("Unable to decode");
    //println!("{:?}",filter);
    let blurred = decoded.fast_blur(process_params.blur);

    let mut bytes: Vec<u8> = Vec::new();
    blurred
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
        .expect("Unable to write");
    bytes
}