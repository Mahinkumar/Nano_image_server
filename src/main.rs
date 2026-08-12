use std::env;

use nano_image_server::{AppState, app::app, logging::init_logging, plugin::Plugins};
use tokio::{fs, signal};

#[tokio::main]
async fn main() {
    init_logging();

    tracing::info!("Starting Image Server");

    println!(
        " \n 
     
    ,________,
    |        |     NANO IMAGE SERVER
    | /\\   @ |     VERSION 0.8.0-BETA
    |/__\\____|



    https://github.com/mahinkumar/nano_image_server
        "
    );

    let port = env::var("NANO_IMG_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);

    let path = env::var("NANO_IMG_PATH")
        .ok()
        .unwrap_or("./images/".to_string());

    let base_dir = fs::canonicalize(path)
        .await
        .expect("Unable to parse base_dir");

    let plugins =  Plugins::init_registry();


    let app_state = AppState { base_dir, plugins };

    let app = app(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    let shutdown_signal = async {
        let ctrl_c = signal::ctrl_c();
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        tokio::select! {
            _ = ctrl_c => tracing::info!("Shutdown signal received (SIGINT). \nExiting process safely. "),
            _ = terminate => tracing::info!("Shutdown signal received (SIGTERM). \nExiting process safely."),
        }
    };

    let url = format!("http://localhost:{port}");
    tracing::info!(url = %url, "Server started :");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .unwrap();
}
