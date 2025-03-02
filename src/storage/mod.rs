mod cache;
mod fs;

use async_trait::async_trait;

use thiserror::Error;
use tokio::io;

#[async_trait]
pub trait ImageCache {
    async fn init(size: usize);
    async fn insert(&self, key: u64, image_stream: Vec<u8>) -> Result<(), CacheError>;
    async fn retrieve(&self, key: u64) -> Result<Vec<u8>, CacheError>;
    async fn clear(&self) -> Result<&Self, CacheError>;
}

#[async_trait]
pub trait AsyncFilesystem {
    // async fn mapfs()->Result<Json,FsError>;
    async fn get_image(name: String) -> Result<Vec<u8>, FsError>;
    async fn save_image(name: String, image_stream: Vec<u8>) -> Result<(), FsError>;
    async fn del_image(name: String) -> Result<(), FsError>;
    async fn setup_directory() -> Result<(), FsError>;
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Cache missing. Maybe the memory limit was hit ?.")]
    CacheNotFound(#[from] io::Error),

    #[error("Unable to insert Image Stream for key `{0}`")]
    InsertionError(u64),

    #[error("Unable to find Image Stream for key `{0}`")]
    KeyNotFound(u64),

    #[error("Unknown Caching error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum FsError {
    
    // Unimplemented

    #[error("Unknown File System error")]
    Unknown,
}
