use thiserror::Error;


#[derive(Error, Debug)]
pub enum ImageServerError {
    /// Image file not found at specified path
    #[error("image not found: {path}")]
    NotFound { path: String },

    /// Failed to read file from disk
    #[error("failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    /// Image format not recognized or supported
    #[error("unsupported or invalid image format")]
    InvalidFormat,

    /// Image decoding failed
    #[cfg(feature = "processing")]
    #[error("failed to decode image: {0}")]
    DecodeError(String),

    /// Image encoding failed
    #[cfg(feature = "processing")]
    #[error("failed to encode image: {0}")]
    EncodeError(String),

    /// Image processing operation failed
    #[cfg(feature = "processing")]
    #[error("image processing failed: {0}")]
    ProcessingError(String),

    /// Invalid processing parameters
    #[cfg(feature = "processing")]
    #[error("invalid processing parameters: {0}")]
    InvalidParameters(String),

    /// Cache operation failed
    #[cfg(feature = "cache")]
    #[error("cache error: {0}")]
    CacheError(String),

    /// TLS configuration error
    #[error("TLS configuration error: {0}")]
    TlsError(String),

    /// Generic internal server error
    #[error("internal server error: {0}")]
    Internal(String),
}

/// Type alias for Results using ImageServerError
pub type Result<T> = std::result::Result<T, ImageServerError>;

impl ImageServerError {
    /// Convert error to appropriate HTTP status code
    pub fn status_code(&self) -> u16 {
        match self {
            ImageServerError::NotFound { .. } => 404,
            ImageServerError::InvalidFormat => 400,
            
            #[cfg(feature = "processing")]
            ImageServerError::DecodeError(_) => 400,
            
            #[cfg(feature = "processing")]
            ImageServerError::InvalidParameters(_) => 400,
            
            #[cfg(feature = "processing")]
            ImageServerError::EncodeError(_) => 500,
            
            #[cfg(feature = "processing")]
            ImageServerError::ProcessingError(_) => 500,
            
            #[cfg(feature = "cache")]
            ImageServerError::CacheError(_) => 500,
            
            ImageServerError::IoError(_) => 500,
            ImageServerError::TlsError(_) => 500,
            ImageServerError::Internal(_) => 500,
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            ImageServerError::NotFound { .. } => "Image not found".to_string(),
            ImageServerError::InvalidFormat => "Invalid or unsupported image format".to_string(),
            
            #[cfg(feature = "processing")]
            ImageServerError::DecodeError(_) => "Failed to decode image".to_string(),
            
            #[cfg(feature = "processing")]
            ImageServerError::InvalidParameters(_) => "Invalid image processing parameters".to_string(),
            
            _ => "Internal server error".to_string(),
        }
    }
}

#[cfg(feature = "processing")]
impl From<image::ImageError> for ImageServerError {
    fn from(err: image::ImageError) -> Self {
        match err {
            image::ImageError::Decoding(_) => ImageServerError::DecodeError(err.to_string()),
            image::ImageError::Encoding(_) => ImageServerError::EncodeError(err.to_string()),
            image::ImageError::Unsupported(_) => ImageServerError::InvalidFormat,
            _ => ImageServerError::ProcessingError(err.to_string()),
        }
    }
}