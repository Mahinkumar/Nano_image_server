

use axum::extract::{Path, Query};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use photon_rs::transform::resize;

use photon_rs::native::open_image;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(default = "default_param")]
struct ProcessParameters{
    resx: u32,
    resy: u32
}

fn default_param() -> ProcessParameters{
    ProcessParameters{ resx: 0 , resy: 0}
}

#[tokio::main]
async fn main() {
    // Image processing Test code
    let app = Router::new()
    .route("/image/:image", get(handler));

    let addr = "0.0.0.0";
    let port = 8000;
    println!("Nano Image Server Running...");
    println!("Serving on http://localhost:{port}/image");

    let final_addr = format!("{addr}:{port}");


    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind(final_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Path(image): Path<String>,
    Query(process_params): Query<ProcessParameters>,
) -> impl IntoResponse {
    let input_path = format!("./images/{}", image);

    let img = match open_image(&input_path) {
        Ok(img) => img,
        Err(img) => match img {
            photon_rs::native::Error::ImageError(_image_error) => return (
                [(header::CONTENT_TYPE, "message")],
                axum::body::Body::from("Image Error")
            ),
            photon_rs::native::Error::IoError(_error) => return (
                [(header::CONTENT_TYPE, "message")],
                axum::body::Body::from("File I/O Error")
            ),
        }
    };

    let mut no_resize = false;
    if process_params.resx == 0{ no_resize = true };
    if process_params.resy == 0{ no_resize = true };

    let final_image = if !no_resize {resize(&img, process_params.resx, process_params.resy, photon_rs::transform::SamplingFilter::Nearest)}else{img};
    let jpeg_bytes = final_image.get_bytes_webp();
    
    // Convert Vec<u8> to axum::body::Body
    (
        [(header::CONTENT_TYPE, "image/jpeg")],
        axum::body::Body::from(jpeg_bytes)
    )
}