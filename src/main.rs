pub mod console;
pub mod filter;
pub mod process;
pub mod transform;
pub mod utils;
pub mod cache;

use axum::extract::{Path, Query, Request};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};

use cache::{cleanup_cache_if_needed, ImageCache};
use console::console_router;
use filter::{blur, brighten, contrast, grayscale};
use image::ImageFormat;
use process::{invert, unsharpen};
use std::hash::Hash;
use std::net::SocketAddr;
use transform::{flip_horizontal, flip_vertical, hue_rotate, resizer, rotate};

use serde::Deserialize;
//use std::time::Instant; // For timing functions
use clap::Parser;

/// Simple CLI application with console flag
#[derive(Parser, Debug)]
#[command(author, version, about="Nano Image Server is a tiny, blazingly fast service to serve images with support for image operation on fly.", long_about = None)]
struct Args {
    #[arg(long, short, default_value_t = false)]
    enable_dashboard: bool,

    #[arg(long, short, default_value_t = 8000)]
    port: u16,

    #[arg(long, short)]
    base_url: Option<String>,

    #[arg(long, short, default_value_t = 8001)]
    dashboard_port: u16,
}

#[derive(Deserialize, Debug, Hash, Clone)]
#[serde(default = "default_param")]

struct ProcessParameters {
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
    }
}

#[derive(Deserialize, Hash)]
pub struct ImgInfo {
    name: String,
    format: String,
    params: ProcessParameters,
}

const ADDR: [u8; 4] = [127, 0, 0, 1];

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let app = Router::new().route("/{image}", get(handler));

    let base_url = match args.base_url {
        Some(base) => base,
        None => "localhost".to_string(),
    };

    println!("Nano Image Server Starting...");
    println!(
        "Serving images on port {} -> http://{}:{}",
        args.port, base_url, args.port
    );

    if args.enable_dashboard {
        println!(
            "Serving dashboard on port {}  -> http://{}:{}",
            base_url, args.dashboard_port, args.dashboard_port
        );
        tokio::join!(
            serve(app, args.port),
            serve(console_router(), args.dashboard_port)
        );
    } else {
        serve(app, args.port).await;
    }
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from((ADDR, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

async fn handler(
    Path(image): Path<String>,
    Query(process_params): Query<ProcessParameters>,
) -> impl IntoResponse {

    
    let parsed_path: Vec<&str> = image.split('.').collect();
    let img_formats = parsed_path[1];

    let meta = ImgInfo {
        name: parsed_path[0].to_owned(),
        format: parsed_path[1].to_owned(),
        params: process_params.clone(),
    };

    let img_format =
        ImageFormat::from_extension(img_formats).expect("Unable to parse Image format");

    let cache = ImageCache::new(meta);
    let computed_hash = cache.get_hash();

    let cache_path = format!("./cache/{}", computed_hash);
    if tokio::fs::try_exists(&cache_path)
        .await
        .expect("Unable to check")
    {
        match tokio::fs::read(cache_path).await {
            Ok(bytes) => {
                return (
                    [(header::CONTENT_TYPE, "image/jpeg")],
                    axum::body::Body::from(bytes),
                );
            }
            Err(err) => {
                let message = format!("{err} Error");
                return (
                    [(header::CONTENT_TYPE, "message")],
                    axum::body::Body::from(message),
                );
            }
        }
    } else {
        cleanup_cache_if_needed("./cache").await;
        let input_path = format!("./images/{}", image);
        let do_resize: bool = process_params.resx != 0 || process_params.resy != 0;
        let do_filter: bool = process_params.filter != "None".to_string();
        let do_transform: bool = process_params.transform != "None".to_string();
        let do_process: bool = process_params.process != "None".to_string();

        match tokio::fs::read(&input_path).await {
            Ok(mut bytes) => {
                if do_resize {
                    bytes = resizer(
                        bytes,
                        img_format,
                        process_params.resx,
                        process_params.resy,
                        &process_params.resfilter,
                    );
                }
                if do_filter {
                    match process_params.filter.to_lowercase().as_str() {
                        "blur" => bytes = blur(bytes, img_format, process_params.f_param as f32),
                        "bw" => bytes = grayscale(bytes, img_format),
                        "brighten" => {
                            bytes = brighten(bytes, img_format, process_params.f_param as f32)
                        }
                        "contrast" => {
                            bytes = contrast(bytes, img_format, process_params.f_param as f32)
                        }
                        _ => {}
                    }
                }
                if do_transform {
                    match process_params.transform.to_lowercase().as_str() {
                        "fliph" => bytes = flip_horizontal(bytes, img_format),
                        "flipv" => bytes = flip_vertical(bytes, img_format),
                        "rotate" => bytes = rotate(bytes, img_format, process_params.t_param),
                        "hue_rotate" => {
                            bytes = hue_rotate(bytes, img_format, process_params.t_param)
                        }
                        _ => {}
                    }
                }
                if do_process {
                    match process_params.process.to_lowercase().as_str() {
                        "invert" => bytes = invert(bytes, img_format),
                        "unsharpen" => {
                            bytes =
                                unsharpen(bytes, img_format, process_params.p1, process_params.p2)
                        }
                        _ => {}
                    }
                }

                let write_path = format!("./cache/{}", &computed_hash);
                tokio::fs::write(write_path, &bytes)
                    .await
                    .expect("UNable to write");
                return (
                    [(header::CONTENT_TYPE, "image/jpeg")],
                    axum::body::Body::from(bytes),
                );
            }
            Err(err) => {
                let message = format!("{err} Error");
                return (
                    [(header::CONTENT_TYPE, "message")],
                    axum::body::Body::from(message),
                );
            }
        }
    }
}
