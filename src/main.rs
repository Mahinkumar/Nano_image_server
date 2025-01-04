pub mod transform;
pub mod filter;

use axum::extract::{Path, Query, Request};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};

use transform::resizer;
use std::net::SocketAddr;
use filter::{blur, brighten, contrast, grayscale};

use serde::Deserialize;
//use std::time::Instant; // For timing functions

#[derive(Deserialize, Debug)]
#[serde(default = "default_param")]
struct ProcessParameters {
    resx: u32,
    resy: u32,
    resfilter: String,
    filter: String,
    f_param: f32,
    transform: String,
    t_param: i8,
}

fn default_param() -> ProcessParameters {
    ProcessParameters {
        resx: 0,
        resy: 0,
        resfilter: "Optimal".to_string(),
        filter: "None".to_string(),
        f_param: 0.0,
        transform: "None".to_string(),
        t_param: 0,
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
    let do_filter: bool = process_params.filter != "None".to_string();
    let do_transform: bool = process_params.transform != "None".to_string();


    match tokio::fs::read(&input_path).await {
        Ok(mut bytes) => {
            if do_resize {
                bytes = resizer(bytes, process_params.resx, process_params.resy, &process_params.resfilter);
            } 
            if do_filter {
                match process_params.filter.to_lowercase().as_str(){
                    "blur" => { bytes =  blur(bytes, process_params.f_param)},
                    "bw" => { bytes = grayscale(bytes)},
                    "brighten" => { bytes = brighten(bytes, process_params.f_param)},
                    "contrast" => { bytes = contrast(bytes, process_params.f_param)}
                    _ => {}
                }

            }
            if do_transform {
                match process_params.transform.to_lowercase().as_str(){
                    //For Transforms
                    _ => {}
                }

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
