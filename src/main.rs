pub mod common;
pub mod handlers;
pub mod monitoring;
pub mod security;
pub mod storage;

use axum;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, handlers::router::main_router()).await.unwrap();
}