use axum::extract::{Path, Query, Request};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};
use image::ImageReader;
use std::io::Cursor;
use std::net::SocketAddr;

use serde::Deserialize;
//use std::time::Instant; // For timing functions

#[derive(Deserialize, Debug)]
#[serde(default = "default_param")]
struct ProcessParameters {
    resx: u32,
    resy: u32,
    resfilter: String,
}

fn default_param() -> ProcessParameters {
    ProcessParameters {
        resx: 0,
        resy: 0,
        resfilter: "Optimal".to_string(),
    }
}

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT_HOST: u16 = 8000;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/image/{image}", get(handler));

    println!("Nano Image Server Running...");
    println!("Serving on http://localhost:{PORT_HOST}/image");

    tokio::spawn(async move { serve(app, PORT_HOST).await })
        .await
        .expect("Unable to Spawn Threads")
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
    //let now = Instant::now();

    let input_path = format!("./images/{}", image);
    let no_resize = process_params.resx == 0 || process_params.resy == 0;

    match tokio::fs::read(&input_path).await {
        Ok(bytes) => {
            if no_resize {
                return (
                    [(header::CONTENT_TYPE, "image/jpeg")],
                    axum::body::Body::from(bytes),
                );
            }
            let decoded = ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()
                .expect("Unable to find format")
                .decode()
                .expect("Unable to decode");
            let filter = choose_resize_filter(&process_params.resfilter);
            //println!("{:?}",filter);
            let resized = decoded.resize(process_params.resx, process_params.resy, filter);

            let mut bytes: Vec<u8> = Vec::new();
            resized
                .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
                .expect("Unable to write");

            //let elapsed = now.elapsed();
            //println!("Elapsed Processed Read: {:.2?}", elapsed);

            return (
                [(header::CONTENT_TYPE, "image/jpeg")],
                axum::body::Body::from(bytes),
            );
        }
        Err(_) => {
            //let elapsed = now.elapsed();
            //println!("Elapsed Error Read: {:.2?}", elapsed);
            return (
                [(header::CONTENT_TYPE, "message")],
                axum::body::Body::from("File I/O Error"),
            );
        }
    }
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
