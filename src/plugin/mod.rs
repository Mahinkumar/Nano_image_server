pub mod error;
pub mod inbuilt;
pub mod registry;

use std::{collections::HashMap, io::Cursor};

use image::{DynamicImage, ImageError, ImageReader};

use crate::plugin::error::ParseError;

#[derive(Clone)]
pub enum PluginOrdering {
    First,
    Any,
    Last
} 

pub type ImagePlugin = fn(DynamicImage, Option<&str>) -> Result<DynamicImage,ParseError>;

#[derive(Clone)]
pub struct Plugins {
    registry: HashMap<String, (ImagePlugin, PluginOrdering)>
}


pub fn decoder(image_bytes: Vec<u8>) -> Result<DynamicImage,ImageError> {
    let decoded = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()?.decode();

    decoded
}

pub fn encoder(image: DynamicImage, format: image::ImageFormat) -> Result<Vec<u8>,ImageError> {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), format)?;

    Ok(bytes)
}


