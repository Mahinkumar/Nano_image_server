use axum::{routing::get, Router};

pub fn main_router() -> Router {
    Router::new()
        .route("/{wildcard}", get(|| async { "Nano Image Server | Images will be rendered here |" }))
        .nest("/secure/", secure_router())
        .nest("/api/", api_router())
}


fn secure_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Secure Images here" }))
        .route("/test",get(|| async { "Authorization test here" }))
}

fn api_router()-> Router{
    Router::new()
        .route("/health",get(|| async { "Health check here" }))
}
