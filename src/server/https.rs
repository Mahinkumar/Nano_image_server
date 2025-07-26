use axum::Router;
use std::net::SocketAddr;

use axum_server::tls_rustls::RustlsConfig;
use rustls::crypto::aws_lc_rs;
use std::path::PathBuf;
use tokio::net::TcpSocket;

use crate::ADDR;

pub async fn serve_https(app: Router, port: u16, cert_path: PathBuf) {
    let cert_dir = cert_path;

    let config = RustlsConfig::from_pem_file(cert_dir.join("cert.pem"), cert_dir.join("key.pem"))
        .await
        .expect("Unable to setup TLS Config");

    let _ = aws_lc_rs::default_provider().install_default();

    let addr = SocketAddr::from((ADDR, port));
    let socket = TcpSocket::new_v4().unwrap();

    socket.set_send_buffer_size(524_288).unwrap();
    socket.set_recv_buffer_size(524_288).unwrap();
    socket.set_nodelay(true).unwrap();

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
