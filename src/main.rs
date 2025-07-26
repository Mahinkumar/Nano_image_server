use axum::extract::Path;

#[cfg(feature = "processing")]
use nano_image_server::processing::ProcessParameters;
#[cfg(feature = "processing")]
use nano_image_server::processing::{image_processing, need_compute};
use nano_image_server::server::https::serve_https;

#[cfg(feature = "processing")]
use axum::extract::Query;

use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use clap::Parser;

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about="Nano Image Server is a tiny, blazingly fast service to serve images with support for basic image operation on fly.", long_about = None)]
pub struct Args {
    /// Defines the port the app is hosted on
    #[arg(long, short, default_value_t = 8000)]
    port: u16,

    /// Base url
    #[arg(
        long,
        short,
        help = "Base url where application is hosted, Default is localhost"
    )]
    base_url: Option<String>,

    /// TLS certificate path. Necessary to run the application
    #[arg(
        long,
        short,
        value_name = "TLS_CERT_PATH",
        help = "TLS certificate path (required)",
        long_help = "Path to TLS certificate file (PEM format). \n\nYou can generate one with:\n $ openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365\n\nPlace the key.pem and cert.pem inside a folder and use the folder's path as argument for this flag"
    )]
    cert_path: PathBuf,
}

#[derive(Deserialize, Hash, Clone)]
pub struct ImgInfo {
    name: String,
    format: String,
    #[cfg(feature = "processing")]
    params: ProcessParameters,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let app = Router::new().route("/{image}", get(handler));

    let base_url = match &args.base_url {
        Some(base) => base,
        None => "localhost",
    };

    println!("Nano Image Server Starting...");
    println!(
        "Serving images on port {} with url -> https://{}:{}",
        args.port, base_url, args.port
    );
    println!(
        "Test image with url -> https://{}:{}/image.jpg",
        base_url, args.port
    );

    serve_https(app, args.port, args.cert_path).await;
}

async fn handler(
    Path(image): Path<String>,
    #[cfg(feature = "processing")] Query(process_params): Query<ProcessParameters>,
) -> impl IntoResponse {
    let parsed_path: Vec<&str> = image.split('.').collect();
    let input_path = format!("./images/{}", image);
    let img_formats = parsed_path[1];
    let img_type = format!("image/{img_formats}");

    match tokio::fs::read(&input_path).await {
        Ok(bytes) => {
            #[cfg(feature = "processing")]
            if need_compute(&process_params) {
                return (
                    [(header::CONTENT_TYPE, img_type)],
                    axum::body::Body::from(image_processing(process_params, bytes, parsed_path)),
                );
            }

            return (
                [(header::CONTENT_TYPE, img_type)],
                axum::body::Body::from(bytes),
            );
        }
        Err(_err) => {
            return (
                [(header::CONTENT_TYPE, "message".to_owned())],
                axum::body::Body::from("Unable to process request"),
            );
        }
    }
}
