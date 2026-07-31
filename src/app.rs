use axum::{Router, routing::get};
use crate::{AppState, handler::image_handler};

pub fn app(app_state: AppState) -> Router{
    let router = Router::new()
        .route("/{image_path}", get(image_handler))
        .with_state(app_state);
    router
}