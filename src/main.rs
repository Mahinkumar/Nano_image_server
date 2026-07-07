use axum::Router;

#[cfg(feature = "cache")]
use axum::extract::State;

use axum::routing::get;

#[cfg(feature = "cache")]
use nano_image_server::AppState;
use nano_image_server::args::Args;

#[cfg(not(feature="cache"))]
use nano_image_server::handler::handler;
#[cfg(not(feature = "tls"))]
use nano_image_server::server::http::serve_http;
#[cfg(feature = "tls")]
use nano_image_server::server::https::serve_https;

#[cfg(feature = "cache")]
use nano_image_server::cache::s3fifo::S3Fifo;

#[cfg(feature = "cache")]
use std::sync::Arc;

#[cfg(feature = "cache")]
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    #[cfg(feature = "cache")]
    let app = {
        use nano_image_server::{AppState, handler::{handler, stats_handler}};

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

    #[cfg(not(feature = "tls"))]
    {
        println!("WARNING: TLS disabled. Serving plain HTTP.");
        serve_http(app, args.port).await;
    }

    #[cfg(feature = "tls")]
    {
        // We can safely unwrap because clap ensures cert_path exists if no_tls is false
        let cert_path = args.cert_path.expect("Cert path required for HTTPS");
        serve_https(app, args.port, cert_path).await;
    }
}

