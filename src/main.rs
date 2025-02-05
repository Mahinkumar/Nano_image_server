pub mod cache;
pub mod transform;
pub mod utils;

use axum::extract::{Path, Query, Request, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};

use cache::ImageCache;
use image::ImageFormat;
use tokio::sync::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::net::TcpSocket;
use transform::{resizer, rotate};
use utils::{decoder, encoder};

use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about="Nano Image Server is a tiny, blazingly fast service to serve images with support for basic image operation on fly.", long_about = None)]
pub struct Args {
    /// Defines the port the app is hosted on
    #[arg(long, short, default_value_t = 8000)]
    port: u16,

    /// Base url of the app (needed when self hosting)
    #[arg(long, short)]
    base_url: Option<String>,

    /// Toggle for Caching. Set to false by default and allows caching. 
    #[arg(long, short, default_value_t = false)]
    no_cache: bool,

    /// Limit for memory cache. 1024 MB is default
    #[arg(long, short, default_value_t = 1024)]
    mem_cache_limit: u32,

    /// Limit for storage based caching. 4096 MB is default
    #[arg(long, short, default_value_t = 1024*4)]
    cache_limit: u32,
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

#[derive(Deserialize, Hash, Clone)]
pub struct ImgInfo {
    name: String,
    format: String,
    params: ProcessParameters,
}

#[derive(Clone)]
pub struct AppState {
    args: Args,
}

const ADDR: [u8; 4] = [127, 0, 0, 1];

static CACHE_DB: Lazy<Arc<Mutex<ImageCache>>> = Lazy::new(|| ImageCache::new_cache());
#[tokio::main]
async fn main() {
    
    let args = Args::parse();

    let app_state: AppState = AppState {
        args: args.clone(),
    };
    let app = Router::new()
        .route("/{image}", get(handler))
        .with_state(app_state);

    let base_url = match &args.base_url {
        Some(base) => base,
        None => "localhost",
    };

    println!("Nano Image Server Starting...");
    println!(
        "Serving images on port {} -> http://{}:{}",
        args.port, base_url, args.port
    );

    serve(app, args.port).await;
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from((ADDR, port));

    let socket = TcpSocket::new_v4().unwrap();

    socket.set_send_buffer_size(524_288).unwrap();
    socket.set_recv_buffer_size(524_288).unwrap();
    socket.set_nodelay(true).unwrap();

    socket.bind(addr).unwrap();
    let listener = socket.listen(2048).unwrap();

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

async fn handler(
    Path(image): Path<String>,
    Query(process_params): Query<ProcessParameters>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let parsed_path: Vec<&str> = image.split('.').collect();
    let img_formats = parsed_path[1];

    let meta = ImgInfo {
        name: parsed_path[0].to_owned(),
        format: parsed_path[1].to_owned(),
        params: process_params.clone(),
    };

    let mut img_type = format!("image/{img_formats}");

    let hash = ImageCache::get_hash(&meta);

    if !app_state.args.no_cache {
        match CACHE_DB.lock().await.get(hash).await {
            Some(val) => {
                return (
                    [(header::CONTENT_TYPE, img_type)],
                    axum::body::Body::from(val),
                );
            }
            None => {}
        }
    }

    let input_path = format!("./images/{}", image);
    let do_resize: bool = process_params.resx != 0 || process_params.resy != 0;
    let do_filter: bool = process_params.filter != "None".to_string();
    let do_transform: bool = process_params.transform != "None".to_string();
    let do_process: bool = process_params.process != "None".to_string();
    let do_convert: bool = process_params.to != "None".to_string();

    match tokio::fs::read(&input_path).await {
        Ok(mut bytes) => {
            if do_resize || do_filter || do_transform || do_process || do_convert {
                let mut decoded_img = decoder(bytes);
                if do_resize {
                    decoded_img = resizer(
                        decoded_img,
                        process_params.resx,
                        process_params.resy,
                        &process_params.resfilter,
                    );
                }
                if do_filter {
                    decoded_img = match process_params.filter.to_lowercase().as_str() {
                        "blur" => decoded_img.blur(process_params.f_param as f32),
                        "bw" => decoded_img.grayscale(),
                        "brighten" => decoded_img.brighten(process_params.f_param),
                        "contrast" => decoded_img.adjust_contrast(process_params.f_param as f32),
                        _ => decoded_img,
                    }
                }
                if do_transform {
                    decoded_img = match process_params.transform.to_lowercase().as_str() {
                        "fliph" => decoded_img.fliph(),
                        "flipv" => decoded_img.flipv(),
                        "rotate" => rotate(decoded_img, process_params.t_param),
                        "hue_rotate" => decoded_img.huerotate(process_params.t_param),
                        _ => decoded_img,
                    }
                }
                if do_process {
                    decoded_img = match process_params.process.to_lowercase().as_str() {
                        "invert" => {
                            decoded_img.invert();
                            decoded_img
                        }
                        "unsharpen" => {
                            decoded_img.unsharpen(process_params.p1 as f32, process_params.p2)
                        }
                        _ => decoded_img,
                    }
                }
                let img_format: ImageFormat;
                if do_convert {
                    img_format = ImageFormat::from_extension(&process_params.to)
                        .expect("Unable to parse Image format");
                    img_type = format!("image/{}", process_params.to);
                } else {
                    img_format = ImageFormat::from_extension(img_formats)
                        .expect("Unable to parse Image format");
                }
                bytes = encoder(decoded_img, img_format);
            }
            if !app_state.args.no_cache {
                CACHE_DB.lock().await.insert(bytes.clone(), meta).await
            }
            return (
                [(header::CONTENT_TYPE, img_type)],
                axum::body::Body::from(bytes),
            );
        }
        Err(err) => {
            let message = format!("{err} Error");
            return (
                [(header::CONTENT_TYPE, "message".to_owned())],
                axum::body::Body::from(message),
            );
        }
    }
}
