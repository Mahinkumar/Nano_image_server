// src/server/http.rs
use crate::ADDR;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn serve_http(app: Router, port: u16) {
    let addr = SocketAddr::from((ADDR, port));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("-> Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
