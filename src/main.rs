pub mod cli;
pub mod resilience;
pub mod cache;
pub mod fs;

use axum::extract::{Path, Request};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};
use cache::ImageCache;
use clap::Parser;
use cli::Args;

#[cfg(feature = "plugins")]
use plugins::log;
use resilience::health;

use std::net::SocketAddr;
use tokio::net::TcpSocket;


/// Loopback addr (Localhost)
const ADDR: [u8; 4] = [127, 0, 0, 1];



#[tokio::main]
async fn main() {
    // Basic arguments are parsed here based on cli.rs file
    // Check cli.rs for more information about the arguments
    let args = Args::parse();

    // Runs only if the plugins feature is enabled
    // Includes a suite of basic image processing methods
    // check the lib.rs in /plugins/src directory for more info
    #[cfg(feature = "plugins")]
    log();

    let cache = ImageCache::init_cache(22);
    cache.preload();

    // Basic router with image request handler and a health check endpoint
    // Todo: Include a security and rate limiting middleware before v0.6.0
    let app = Router::new()
        .route("/{image}", get(handler))
        .route("/health", get(health))
        .route("/status", get(health));

    // Serve the app
    serve(app, args.port).await;
}

/// Serves the router in specified port.
/// Anything related to sockets and serving must be implemented here
/// Includes socket configurations including SO_SNDBUF, SO_RCVBUF and TCP_NODELAY.
/// The socket backlog is also defined here.
async fn serve(app: Router, port: u16) {
    // Address and socket definitions
    let addr = SocketAddr::from((ADDR, port));
    let socket = TcpSocket::new_v4().unwrap();

    //socket configurations
    socket.set_send_buffer_size(524_288).unwrap();
    socket.set_recv_buffer_size(524_288).unwrap();
    socket.set_nodelay(true).unwrap();
    socket.bind(addr).unwrap();

    
    /*
    Socket backlog configuration.

    Defines the maximum number of pending connections that are
    queued by the operating system at any moment*/
    let listener = socket.listen(2048).unwrap();

    // Serve the app service to specified listener
    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

/// Standard handler for the Image Server
async fn handler(Path(image): Path<String>) -> impl IntoResponse {
    // Due to rewrite the functions are incomplete here.

    return (
        [(header::CONTENT_TYPE, "text".to_owned())],
        axum::body::Body::from(image),
    );
}
