#[cfg(feature = "processing")]
use image::ImageFormat;
use serde::Deserialize;

#[cfg(feature = "processing")]
use crate::compute::transform::{resizer, rotate};
use crate::compute::utils::{decoder, encoder};

#[derive(Deserialize, Debug, Hash, Clone)]
#[serde(default = "default_param")]

pub struct ProcessParameters {
    resx: u32,
    resy: u32,
    resfilter: String,
    filter: String,
    f_param: i32,
    transform: String,
    t_param: i32,
    process: String,
    p1: i32,
    p2: i32,
    to: String,
}

fn default_param() -> ProcessParameters {
    ProcessParameters {
        resx: 0,
        resy: 0,
        resfilter: "Optimal".to_string(),
        filter: "None".to_string(),
        f_param: 0,
        transform: "None".to_string(),
        t_param: 0,
        process: "None".to_string(),
        p1: 0,
        p2: 0,
        to: "None".to_string(),
    }
}

pub fn need_compute(process_params: &ProcessParameters) -> bool {
    process_params.resx != 0
        || process_params.resy != 0
        || process_params.filter != "None".to_string()
        || process_params.transform != "None".to_string()
        || process_params.process != "None".to_string()
        || process_params.to != "None".to_string()
}

pub fn image_processing(
    process_params: ProcessParameters,
    bytes: Vec<u8>,
    parsed_path: Vec<&str>,
) -> Vec<u8> {
    let mut decoded_img = decoder(bytes);

    let img_formats = parsed_path[1];

    if process_params.resx != 0 || process_params.resy != 0 {
        decoded_img = resizer(
            decoded_img,
            process_params.resx,
            process_params.resy,
            &process_params.resfilter,
        );
    }
    if process_params.filter != "None".to_string() {
        decoded_img = match process_params.filter.to_lowercase().as_str() {
            "blur" => decoded_img.blur(process_params.f_param as f32),
            "bw" => decoded_img.grayscale(),
            "brighten" => decoded_img.brighten(process_params.f_param),
            "contrast" => decoded_img.adjust_contrast(process_params.f_param as f32),
            _ => decoded_img,
        }
    }
    if process_params.transform != "None".to_string() {
        decoded_img = match process_params.transform.to_lowercase().as_str() {
            "fliph" => decoded_img.fliph(),
            "flipv" => decoded_img.flipv(),
            "rotate" => rotate(decoded_img, process_params.t_param),
            "hue_rotate" => decoded_img.huerotate(process_params.t_param),
            _ => decoded_img,
        }
    }
    if process_params.process != "None".to_string() {
        decoded_img = match process_params.process.to_lowercase().as_str() {
            "invert" => {
                decoded_img.invert();
                decoded_img
            }
            "unsharpen" => decoded_img.unsharpen(process_params.p1 as f32, process_params.p2),
            _ => decoded_img,
        }
    }
    let img_format: ImageFormat;
    if process_params.to != "None".to_string() {
        img_format =
            ImageFormat::from_extension(&process_params.to).expect("Unable to parse Image format");
    } else {
        img_format =
            ImageFormat::from_extension(img_formats).expect("Unable to parse Image format");
    }
    encoder(decoded_img, img_format)
}
