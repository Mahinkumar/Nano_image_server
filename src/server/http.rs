// src/server/http.rs
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::ADDR; // Assuming this is [127, 0, 0, 1]

pub async fn serve_http(app: Router, port: u16) {
    let addr = SocketAddr::from((ADDR, port));
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("-> Listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}