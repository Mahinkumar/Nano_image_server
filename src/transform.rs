use image::DynamicImage;

pub fn resizer(image: DynamicImage, x: u32, y: u32, filter: &str) -> DynamicImage {
    let final_filter = choose_resize_filter(filter, x, y, image.height(), image.width());
    image.resize(x, y, final_filter)
}

pub fn rotate(image: DynamicImage, transform_param: i32) -> DynamicImage {
    let rotated: DynamicImage;
    if transform_param == 90 {
        rotated = image.rotate90();
    } else if transform_param == 180 {
        rotated = image.rotate180();
    } else if transform_param == 270 {
        rotated = image.rotate270();
    } else {
        rotated = image
    }
    rotated
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
