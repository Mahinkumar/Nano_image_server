use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use tokio::fs;

use crate::AppState;

pub async fn image_handler(
    State(state): State<AppState>,
    Path(image): Path<String>,
) -> Response<Body> {
    let image_path = state.base_dir.join(&image);

    let canonical_path = fs::canonicalize(&image_path).await;
    
    match canonical_path {
        Ok(canonical_path) => {
            if !canonical_path.starts_with(&state.base_dir) {
                return StatusCode::BAD_REQUEST.into_response();
            }

            let Some(extension) = canonical_path.extension().and_then(|ext| ext.to_str()) else {
                return StatusCode::BAD_REQUEST.into_response();
            };

            let img_type = match extension {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "webp" => "image/webp",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                _ => return StatusCode::NOT_IMPLEMENTED.into_response(),
            };

            let Ok(bytes) = fs::read(&canonical_path).await else {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            };

            (StatusCode::OK, [(header::CONTENT_TYPE, img_type)], bytes).into_response()
        }
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn processing_handler(State(state): State<AppState>, Path(image): Path<String>) -> Response<Body> {
    tracing::info!(image);
    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
}
