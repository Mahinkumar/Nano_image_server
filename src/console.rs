use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    routing::{get, Router},
};
use rust_embed::Embed;
use serde_json::json;
use tokio::fs;

#[derive(Embed)]
#[folder = "./console/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

pub fn console_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/assets/{*file}", get(static_handler))
        .route("/api/{req}", get(api))
        .fallback_service(get(not_found))
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

async fn api(Path(req): Path<String>) -> Response<Body> {
    match req.as_str() {
        "list" => Html(get_images().await).into_response(),
        _ => Html("<h1>501</h1><p>Not Implemented</p>").into_response(),
    }
}

async fn get_images() -> String {
    let base_path = PathBuf::from("./images");
    let mut files = Vec::new();
    let mut dirs = vec![base_path.clone()];

    while let Some(dir) = dirs.pop() {
        let mut entries = fs::read_dir(&dir).await.expect("Unable to read dir");

        while let Some(entry) = entries
            .next_entry()
            .await
            .expect("Unable to read dir entry")
        {
            let path = entry.path();

            let relative_path = path
                .strip_prefix(&base_path)
                .expect("Failed to strip prefix")
                .to_string_lossy()
                .into_owned();

            if path.is_dir() {
                dirs.push(path);
            } else {
                files.push(relative_path);
            }
        }
    }

    serde_json::to_string(&json!({
        "images": files
    }))
    .expect("Unable to stringify json")
}
