mod cache;
mod fs;

use async_trait::async_trait;

use thiserror::Error;
use tokio::io;

#[async_trait]
pub trait ImageCache {
    async fn init(size: usize) -> Self;
    async fn insert(&self, key: u64, image_stream: Vec<u8>);
    async fn retrieve(&self, key: u64) -> Result<Vec<u8>, CacheError>;
    async fn clear(&self, key: u64) -> Result<Vec<u8>, CacheError>;
}

#[async_trait]
pub trait AsyncFilesystem {
    // async fn mapfs()->Result<Json,FsError>;
    async fn init(path: String) -> Self;
    async fn get_image(&self, name: String) -> Result<Vec<u8>, FsError>;
    async fn save_image(&self, name: String, image_stream: Vec<u8>) -> Result<(), FsError>;
    async fn del_image(&self, name: String) -> Result<(), FsError>;
    fn validate_filename(name: &str)->Result<(),FsError>;
    // async fn setup_directory(&self) -> Result<(), FsError>;
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

    #[error("Read Error for file with name : `{0}`. The file may not exist.")]
    ReadError(String),

    #[error("Write Error for file with name : `{0}`")]
    WriteError(String),

    #[error("Deletion Error for file with name : `{0}`")]
    DeleteError(String),

    #[error("Invalid image name. Name cannot contain '.' and '/' : `{0}`")]
    InvalidName(String),
}
