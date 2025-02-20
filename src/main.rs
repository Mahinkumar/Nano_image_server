
pub mod utils;
pub mod cli;

use axum::extract::{Path, Request};
use axum::http::header;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, ServiceExt};
use cli::Args;
use clap::Parser;

#[cfg(feature = "plugins")]
use plugins::log;

use std::net::SocketAddr;
use tokio::net::TcpSocket;

const ADDR: [u8; 4] = [127, 0, 0, 1];

#[tokio::main]
async fn main() {
    
    let args = Args::parse();

    #[cfg(feature = "plugins")]
    log();

    let app = Router::new()
        .route("/{image}", get(handler));

    serve(app, args.port).await;
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from((ADDR, port));

    let socket = TcpSocket::new_v4().unwrap();

    socket.set_send_buffer_size(524_288).unwrap();
    socket.set_recv_buffer_size(524_288).unwrap();
    socket.set_nodelay(true).unwrap();

    socket.bind(addr).unwrap();
    let listener = socket.listen(2048).unwrap();

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

async fn handler(
    Path(image): Path<String>,
) -> impl IntoResponse {
    // Due to rewrite the functions are incomplete here. 

    return (
        [(header::CONTENT_TYPE, "text".to_owned())],
        axum::body::Body::from(image)
    )
    
}
