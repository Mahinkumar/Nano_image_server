

use fast_image_resize::{FilterType, IntoImageView, PixelType, ResizeAlg, ResizeOptions, Resizer, images::Image};
use image::{DynamicImage, RgbImage, RgbaImage};

use crate::plugin::error::ParseError;

pub fn resize(image: DynamicImage, name: Option<&str>) -> Result<DynamicImage, ParseError>{
    match name{
        Some(name) => {
            
            let (x,y) = name.split_once("x").unwrap();

            let width = x.parse().unwrap();
            let height = y.parse().unwrap();
            let mut dst_image = Image::new(
                width,
                height,
                image.pixel_type().unwrap(),
            );

            let mut resizer = Resizer::new();

            resizer.resize(
                &image,
                &mut dst_image,
                &ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Lanczos3)),
            ).unwrap();

            let dst_dynamic: DynamicImage = match dst_image.pixel_type() {
                PixelType::U8x3 => DynamicImage::ImageRgb8(
                    RgbImage::from_raw(width, height, dst_image.buffer().to_vec()).unwrap()
                ),
                PixelType::U8x4 => DynamicImage::ImageRgba8(
                    RgbaImage::from_raw(width, height, dst_image.buffer().to_vec()).unwrap()
                ),
                _ => todo!(),
            };

            Ok(dst_dynamic)
        },
        None => Err(ParseError::MissingParameter("Resolution for resize".to_string())),
    }
}