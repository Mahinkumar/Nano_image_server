use std::io::Cursor;
use image::ImageReader;

pub fn resizer(image_bytes: Vec<u8>,x: u32 ,y: u32 ,filter: &str) -> Vec<u8>{
    let decoded = ImageReader::new(Cursor::new(image_bytes))
                .with_guessed_format()
                .expect("Unable to find format")
                .decode()
                .expect("Unable to decode");
    let final_filter = choose_resize_filter(filter);
    //println!("{:?}",filter);
    let resized = decoded.resize(x, y, final_filter);

    let mut bytes: Vec<u8> = Vec::new();
    resized
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
