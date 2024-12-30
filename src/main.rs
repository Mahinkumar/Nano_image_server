

use axum::extract::{Path, Query, Request};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};
use std::io::Cursor;
use std::net::SocketAddr;
use image::ImageReader;

use serde::Deserialize;
//use std::time::Instant; // For timing functions

#[derive(Deserialize,Debug)]
#[serde(default = "default_param")]
struct ProcessParameters{
    resx: u32,
    resy: u32
}

fn default_param() -> ProcessParameters{
    ProcessParameters{ resx: 0 , resy: 0}
}

const ADDR: [u8; 4] = [127, 0, 0, 1];
const PORT_HOST: u16 = 8000;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/image/:image", get(handler));

    println!("Nano Image Server Running...");
    println!("Serving on http://localhost:{PORT_HOST}/image");

    tokio::spawn(async move { serve(app, PORT_HOST).await })
        .await
        .expect("Unable to Spawn Threads")
}

async fn serve(app:Router, port: u16) {
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
    
    if no_resize {
        match tokio::fs::read(&input_path).await {
            Ok(bytes) => {
                //let elapsed = now.elapsed();
                //println!("Elapsed direct Read: {:.2?}", elapsed);
                return (
                    [(header::CONTENT_TYPE, "image/jpeg")],
                    axum::body::Body::from(bytes)
                )
            }
            Err(_) =>{
                //let elapsed = now.elapsed();
                //println!("Elapsed Error Read: {:.2?}", elapsed);
                return (
                    [(header::CONTENT_TYPE, "message")],
                    axum::body::Body::from("File I/O Error")
                )
            } 
        }

        
    } else {
        match ImageReader::open(input_path) {
            Ok(img) => {
                let decoded = img.decode().expect("Unable to decode");
                let resized = decoded.resize(
                    process_params.resx,
                    process_params.resy,
                    image::imageops::FilterType::Nearest
                );
                
                let mut bytes: Vec<u8> = Vec::new();
                resized
                    .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
                    .expect("Unable to write");

                //let elapsed = now.elapsed();
                //println!("Elapsed Processed Read: {:.2?}", elapsed);
                
                (
                    [(header::CONTENT_TYPE, "image/jpeg")],
                    axum::body::Body::from(bytes)
                )
            }
            Err(_) => {
                //let elapsed = now.elapsed();
                //println!("Elapsed Processed Error Read: {:.2?}", elapsed);
                return (
                [(header::CONTENT_TYPE, "message")],
                axum::body::Body::from("File I/O Error")
            )}
        }
    }
    
}