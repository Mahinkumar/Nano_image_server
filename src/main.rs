pub mod resize;
pub mod proc;

use axum::extract::{Path, Query, Request};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};
use resize::resizer;
use std::net::SocketAddr;
use proc::blur;

use serde::Deserialize;
//use std::time::Instant; // For timing functions

#[derive(Deserialize, Debug)]
#[serde(default = "default_param")]
pub struct ProcessParameters {
    pub resx: u32,
    pub resy: u32,
    pub resfilter: String,
    pub blur: f32,
}

fn default_param() -> ProcessParameters {
    ProcessParameters {
        resx: 0,
        resy: 0,
        resfilter: "Optimal".to_string(),
        blur: 0.0
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
    let do_resize: bool = process_params.resx != 0 || process_params.resy != 0;
    let do_proc: bool = process_params.blur != 0.0;


    match tokio::fs::read(&input_path).await {
        Ok(mut bytes) => {
            if do_resize {
                bytes = resizer(bytes, &process_params);
            } 
            if do_proc {
                bytes = blur(bytes, &process_params);
                //let elapsed = now.elapsed();
                //println!("Elapsed Processed Read: {:.2?}", elapsed);

            }
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
