use image::{DynamicImage, ImageReader};
use std::io::Cursor;
use crate::error::Result;

pub fn decoder(image_bytes: Vec<u8>) -> Result<DynamicImage> {
    let decoded = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .map_err(|e| crate::error::ImageServerError::DecodeError(e.to_string()))?
        .decode()?; 
    
    Ok(decoded)
}

pub fn encoder(image: DynamicImage, format: image::ImageFormat) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), format)?; 
    
    Ok(bytes)
}