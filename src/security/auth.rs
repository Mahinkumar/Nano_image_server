use super::{Auth, AuthError, JWT_CONFIG};
use async_trait::async_trait;
use axum::http::header;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    nbf: usize,
    sub: String,
}

impl Claims {
    pub fn new(subject: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Claims {
            sub: subject,
            iss: "nano_image_server".to_string(),
            aud: "nano_image_client".to_string(),
            iat: now,
            nbf: now,
            exp: now + JWT_CONFIG.expiration,
        }
    }
}

#[async_trait]
impl Auth for Claims {
    async fn encode(claims: Claims) -> Result<String, AuthError> {
        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(JWT_CONFIG.secret.as_bytes());

        encode(&header, &claims, &key).map_err(|_| AuthError::EncodingError)
    }

    async fn decode(header: header::HeaderValue) -> Result<Claims, AuthError> {
        let token = extract_token(header)?;
        let key = DecodingKey::from_secret(JWT_CONFIG.secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let token_data =
            decode::<Claims>(&token, &key, &validation).map_err(|_| AuthError::DecodingError)?;

        Ok(token_data.claims)
    }

    async fn verify(header: header::HeaderValue) -> Result<(), AuthError> {
        let claims = Self::decode(header).await?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        if claims.exp <= now {
            return Err(AuthError::InvalidTokenError);
        }

        if claims.nbf > now {
            return Err(AuthError::InvalidTokenError);
        }

        if claims.iss != "nano_image_server" {
            return Err(AuthError::InvalidTokenError);
        }

        if claims.aud != "nano_image_client" {
            return Err(AuthError::InvalidTokenError);
        }

        Ok(())
    }
}

// Helper function to extract token from Authorization header
fn extract_token(auth_header: header::HeaderValue) -> Result<String, AuthError> {
    let auth_str = auth_header
        .to_str()
        .map_err(|_| AuthError::InvalidTokenError)?;

    if !auth_str.starts_with("Bearer ") {
        return Err(AuthError::InvalidTokenError);
    }

    Ok(auth_str[7..].to_string()) // Remove "Bearer " prefix
}
