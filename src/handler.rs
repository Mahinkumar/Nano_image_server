use axum::extract::Path;

#[cfg(feature = "cache")]
use axum::extract::State;

use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};

use tokio::fs;

#[cfg(feature = "cache")]
use crate::AppState;
#[cfg(feature = "cache")]
use crate::cache::Cache;
use crate::error::{ImageServerError, Result};


// Core image processing logic with caching
#[cfg(feature = "cache")]
pub async fn handle_image_request_cached(
    state: AppState,
    image: String,
) -> Result<(String, Vec<u8>)> {
    let cache_key = image.clone();

    {
        let cache = state.cache.read().await;
        if let Some(cached_bytes) = cache.get(&cache_key) {
            let extension = std::path::Path::new(&image)
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|s| s.to_lowercase())
                .unwrap_or_default();

            let img_type = match extension.as_str() {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "webp" => "image/webp",
                "gif" => "image/gif",
                "svg" => "image/svg+xml",
                _ => "application/octet-stream",
            };

            return Ok((img_type.to_string(), cached_bytes.clone()));
        }
    }

    let (content_type, bytes) = handle_image_request(image.clone()).await?;

    {
        let mut cache = state.cache.write().await;
        cache.insert(cache_key, bytes.clone());
    }

    Ok((content_type, bytes))
}

pub async fn handle_image_request(image: String) -> Result<(String, Vec<u8>)> {
    let base_dir = fs::canonicalize("./images/")
        .await
        .map_err(|e| ImageServerError::Internal(format!("Base dir config error: {}", e)))?;

    if image.contains("..") || image.starts_with('/') || image.contains('\\') {
        return Err(ImageServerError::InvalidFormat);
    }

    let image_path = base_dir.join(&image);

    let canonical_path =
        fs::canonicalize(&image_path)
            .await
            .map_err(|_| ImageServerError::NotFound {
                path: image.clone(),
            })?;

    if !canonical_path.starts_with(&base_dir) {
        return Err(ImageServerError::InvalidFormat);
    }

    let extension = canonical_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or(ImageServerError::InvalidFormat)?
        .to_lowercase();

    let img_type = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        _ => return Err(ImageServerError::InvalidFormat),
    };

    let bytes = fs::read(&canonical_path).await?;

    Ok((img_type.to_string(), bytes))
}


#[cfg(not(feature = "cache"))]
pub async fn handler(Path(image): Path<String>) -> Response {
    let result = handle_image_request(image).await;

    match result {
        Ok((content_type, body)) => {
            (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], body).into_response()
        }
        Err(err) => {
            let status_code = StatusCode::from_u16(err.status_code())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let message = err.user_message();

            eprintln!("Error serving image: {}", err);

            (status_code, message).into_response()
        }
    }
}

#[cfg(feature = "cache")]
pub async fn handler(State(state): State<AppState>, Path(image): Path<String>) -> Response {
    let result = handle_image_request_cached(state, image).await;


    match result {
        Ok((content_type, body)) => {
            (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], body).into_response()
        }
        Err(err) => {
            let status_code = StatusCode::from_u16(err.status_code())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let message = err.user_message();

            eprintln!("Error serving image: {}", err);

            (status_code, message).into_response()
        }
    }
}

/// Stats endpoint to monitor cache performance
#[cfg(feature = "cache")]
pub async fn stats_handler(State(state): State<AppState>) -> String {
    let cache = state.cache.read().await;
    let stats = cache.stats();

    format!(
        "Cache Statistics\n\
         ================\n\
         Hits: {}\n\
         Misses: {}\n\
         Hit Rate: {:.2}%\n\
         Current Size: {}/{}\n",
        stats.hits, stats.misses, stats.hit_rate, stats.size, stats.capacity
    )
}

