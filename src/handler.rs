use std::collections::VecDeque;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use image::DynamicImage;
use tokio::fs;

use crate::{AppState, plugin::{ImagePlugin, decoder, encoder}};

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

pub async fn processing_handler(State(state): State<AppState>, Path(image_and_plugins): Path<String>) -> Response<Body> {
    
    let params: Vec<&str> = image_and_plugins.split("/").collect();

    let image = params[0];

    let mut process_stack: VecDeque<(ImagePlugin,&str)> = VecDeque::new();

    for i in 1..params.len(){
        let (name, args) =  params[i].split_once(":").unwrap();

        let plugin_data = state.plugins.get_plugin(name.to_string()).unwrap();
        match plugin_data.1{
            crate::plugin::PluginOrdering::First =>  process_stack.push_front((plugin_data.0,args)),
            crate::plugin::PluginOrdering::Any => process_stack.push_front((plugin_data.0,args)),
            crate::plugin::PluginOrdering::Last => process_stack.push_back((plugin_data.0,args)),
        }
       
    }
    
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

            let mut image: DynamicImage = decoder(bytes.clone()).unwrap();

            for i in process_stack{
                image = i.0(image.clone(), Some(i.1)).unwrap();
            }

            let bytes = encoder(image, image::ImageFormat::from_extension(extension).unwrap()).unwrap();

            (StatusCode::OK, [(header::CONTENT_TYPE, img_type)], bytes).into_response()
        }
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    }
}
