use async_trait::async_trait;
use auth::Claims;
use axum::http::header;
use thiserror::Error;

use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;

mod auth;
mod resilience;

#[async_trait]
pub trait Auth {
    async fn encode(claims: Claims) -> Result<String, AuthError>;
    async fn decode(header: header::HeaderValue) -> Result<Claims, AuthError>;
    async fn verify(header: header::HeaderValue) -> Result<(), AuthError>;
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Unable to Encode")]
    EncodingError,

    #[error("Unable to Decode")]
    DecodingError,

    #[error("Invalid Token")]
    InvalidTokenError,

    #[error("Unknown Security error")]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: usize, // in seconds
    pub issuer: String,
    pub audience: String,
}

pub static JWT_CONFIG: Lazy<JwtConfig> = Lazy::new(|| {
    dotenv().ok(); // Load .env file if it exists

    JwtConfig {
        secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment"),
        expiration: env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "3600".to_string()) // Default 1 hour
            .parse()
            .expect("JWT_EXPIRATION must be a valid number"),
        issuer: env::var("JWT_ISSUER").unwrap_or_else(|_| "nano_image_server".to_string()),
        audience: env::var("JWT_AUDIENCE").unwrap_or_else(|_| "nano_image_client".to_string()),
    }
});
