use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {

    // build our application with a single route
    let app = Router::new().route("/images", get(|| async { "Hello from Nano Image Server!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    
}