use axum::{http::header, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Health {
    load: f32,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Healthy,
    Underload,
}

pub async fn health() -> impl IntoResponse {
    // Placeholder load
    // Implement safe and reliable health check
    let load = 3.0;

    // Determine status based on load
    let status = if load > 5.0 {
        Status::Underload
    } else {
        Status::Healthy
    };

    return (
        [(header::CONTENT_TYPE, "application/json".to_owned())],
        Json(Health { load, status }),
    );
}
