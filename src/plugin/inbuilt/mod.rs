use image::DynamicImage;

use crate::plugin::error::ParseError;



pub fn resize(image: DynamicImage, name: Option<&str>) -> Result<DynamicImage, ParseError>{
    match name{
        Some(name) => {
            let (x,y) = name.split_once("x").unwrap();
            let img = image.resize(x.parse().unwrap(), y.parse().unwrap(), image::imageops::FilterType::Lanczos3);
            Ok(img)
        },
        None => Err(ParseError::MissingParameter("Resolution for resize".to_string())),
    }
}