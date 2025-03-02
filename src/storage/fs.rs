use thiserror::Error;
use tokio::io;
pub trait ImageCache {
    async fn insert(&self, key: u64, image_stream:Vec<u8>)->Result<(),CacheError>;
    async fn retrieve(&self, key:u64)->Result<Vec<u8>,CacheError>;
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Data store disconnected")]
    Disconnect(#[from] io::Error),

    #[error("Unable to insert Image Stream for key `{0}`")]
    InsertionError(u64),

    #[error("Unable to find Image Stream for key `{0}`")]
    KeyNotFound(u64),


    #[error("unknown Caching error")]
    Unknown,
}