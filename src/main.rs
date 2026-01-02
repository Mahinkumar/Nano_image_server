use axum::Router;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;

#[cfg(feature = "processing")]
use axum::extract::Query;
#[cfg(feature = "processing")]
use nano_image_server::compute::processing::{ProcessParameters, image_processing, need_compute};

use nano_image_server::error::{ImageServerError, Result};
use nano_image_server::server::http::serve_http;
use nano_image_server::server::https::serve_https;

#[cfg(feature = "cache")]
use nano_image_server::cache::{Cache, s3fifo::S3Fifo};

use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

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

    #[arg(long, default_value_t = false)]
    no_tls: bool,

    /// TLS certificate path. Necessary to run the application
    #[arg(
        long,
        short,
        required_unless_present = "no_tls",
        value_name = "TLS_CERT_PATH",
        help = "TLS certificate path (required)",
        long_help = "Path to TLS certificate file (PEM format). \n\nYou can generate one with:\n $ openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365\n\nPlace the key.pem and cert.pem inside a folder and use the folder's path as argument for this flag"
    )]
    cert_path: Option<PathBuf>,

    /// Cache capacity (number of images to cache)
    #[cfg(feature = "cache")]
    #[arg(long, default_value_t = 100)]
    cache_capacity: usize,
}

#[cfg(feature = "cache")]
#[derive(Clone)]
struct AppState {
    cache: Arc<RwLock<S3Fifo<String, Vec<u8>>>>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    #[cfg(feature = "cache")]
    let app = {
        let cache = Arc::new(RwLock::new(S3Fifo::new(args.cache_capacity)));
        let state = AppState { cache };
        Router::new()
            .route("/{image}", get(handler))
            .route("/_stats", get(stats_handler))
            .with_state(state)
    };

    #[cfg(not(feature = "cache"))]
    let app = Router::new().route("/{image}", get(handler));

    let base_url = match &args.base_url {
        Some(base) => base,
        None => "localhost",
    };

    println!("Nano Image Server Starting...");
    println!(
        "Serving images on port {} with url -> {}:{}",
        args.port, base_url, args.port
    );

    #[cfg(feature = "cache")]
    println!("Cache enabled with capacity: {}", args.cache_capacity);

    if args.no_tls {
        println!("WARNING: TLS disabled. Serving plain HTTP.");
        serve_http(app, args.port).await;
    } else {
        // We can safely unwrap because clap ensures cert_path exists if no_tls is false
        let cert_path = args.cert_path.expect("Cert path required for HTTPS");
        serve_https(app, args.port, cert_path).await;
    }
}

#[cfg(feature = "cache")]
async fn handler(
    State(state): State<AppState>,
    Path(image): Path<String>,
    #[cfg(feature = "processing")] Query(process_params): Query<ProcessParameters>,
) -> Response {
    #[cfg(feature = "processing")]
    let result = handle_image_request_cached(state, image, Some(process_params)).await;

    #[cfg(not(feature = "processing"))]
    let result = handle_image_request_cached(state, image, None).await;

    match result {
        Ok((content_type, body)) => {
            (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], body).into_response()
        }
        Err(err) => {
            let status_code = StatusCode::from_u16(err.status_code())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let message = err.user_message();

            eprintln!("Error serving image: {}", err);

            (status_code, message).into_response()
        }
    }
}

/// Stats endpoint to monitor cache performance
#[cfg(feature = "cache")]
async fn stats_handler(State(state): State<AppState>) -> String {
    let cache = state.cache.read().await;
    let stats = cache.stats();
    
    format!(
        "Cache Statistics\n\
         ================\n\
         Hits: {}\n\
         Misses: {}\n\
         Hit Rate: {:.2}%\n\
         Current Size: {}/{}\n",
        stats.hits,
        stats.misses,
        stats.hit_rate,
        stats.size,
        stats.capacity
    )
}


#[cfg(not(feature = "cache"))]
async fn handler(
    Path(image): Path<String>,
    #[cfg(feature = "processing")] Query(process_params): Query<ProcessParameters>,
) -> Response {
    #[cfg(feature = "processing")]
    let result = handle_image_request(image, Some(process_params)).await;

    #[cfg(not(feature = "processing"))]
    let result = handle_image_request(image, None).await;

    match result {
        Ok((content_type, body)) => {
            (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], body).into_response()
        }
        Err(err) => {
            let status_code = StatusCode::from_u16(err.status_code())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let message = err.user_message();

            eprintln!("Error serving image: {}", err);

            (status_code, message).into_response()
        }
    }
}

// Core image processing logic with caching
#[cfg(feature = "cache")]
async fn handle_image_request_cached(
    state: AppState,
    image: String,
    #[cfg(feature = "processing")] process_params: Option<ProcessParameters>,
    #[cfg(not(feature = "processing"))] _process_params: Option<()>,
) -> Result<(String, Vec<u8>)> {
    #[cfg(feature = "processing")]
    let cache_key = if let Some(ref params) = process_params {
        format!("{:?}:{:?}", image, params)
    } else {
        image.clone()
    };

    #[cfg(not(feature = "processing"))]
    let cache_key = image.clone();

    {
        let cache = state.cache.read().await;
        if let Some(cached_bytes) = cache.get(&cache_key) {
            let parsed_path: Vec<&str> = image.split('.').collect();
            if parsed_path.len() == 2 {
                let img_type = format!("image/{}", parsed_path[1]);
                return Ok((img_type, cached_bytes.clone()));
            }
        }
    }

    #[cfg(feature = "processing")]
    let (content_type, bytes) = handle_image_request(image.clone(), process_params).await?;

    #[cfg(not(feature = "processing"))]
    let (content_type, bytes) = handle_image_request(image.clone(), None).await?;

    {
        let mut cache = state.cache.write().await;
        cache.insert(cache_key, bytes.clone());
    }

    Ok((content_type, bytes))
}

async fn handle_image_request(
    image: String,
    #[cfg(feature = "processing")] process_params: Option<ProcessParameters>,
    #[cfg(not(feature = "processing"))] _process_params: Option<()>,
) -> Result<(String, Vec<u8>)> {
    // Parse image path and format
    let parsed_path: Vec<&str> = image.split('.').collect();

    if parsed_path.len() != 2 {
        return Err(ImageServerError::InvalidFormat);
    }

    let img_formats = parsed_path[1];
    let img_type = format!("image/{}", img_formats);
    let input_path = format!("./images/{}", image);

    // Check if file exists and read it
    let bytes = tokio::fs::read(&input_path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            ImageServerError::NotFound { path: input_path }
        } else {
            ImageServerError::from(e)
        }
    })?;

    // Process image if feature is enabled and params require it
    #[cfg(feature = "processing")]
    if let Some(params) = process_params {
        if need_compute(&params) {
            let processed = image_processing(params, bytes, parsed_path)?;
            return Ok((img_type, processed));
        }
    }

    // Return raw bytes if no processing needed
    Ok((img_type, bytes))
}
