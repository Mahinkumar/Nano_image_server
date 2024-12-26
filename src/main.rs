

use axum::extract::{Path, Query};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use photon_rs::transform::resize;
use tower_http::services::ServeDir;

use photon_rs::channels::alter_blue_channel;
use photon_rs::native::open_image;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct ProcessParameters{
    resx: u32,
    resy: u32
}

#[tokio::main]
async fn main() {
    // Image processing Test code
    
    
    let app = Router::new()
    .route_service("/images", ServeDir::new("images"))
    .route("/processed/:image", get(handler));


    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Path(image): Path<String>,
    Query(process_params): Query<ProcessParameters>,
) -> impl IntoResponse {
    println!("{:?}", process_params);
    
    let input_path = format!("./images/{}", image);
    let mut img = open_image(&input_path)
        .expect("File should open");
    
    resize(&img, process_params.resx, process_params.resy, photon_rs::transform::SamplingFilter::Nearest);
    
    let jpeg_bytes = img.get_bytes_jpeg(255);
    
    // Convert Vec<u8> to axum::body::Body
    (
        [(header::CONTENT_TYPE, "image/jpeg")],
        axum::body::Body::from(jpeg_bytes)
    )
}