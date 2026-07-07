#[cfg(feature = "cache")]
use std::sync::Arc;

#[cfg(feature = "cache")]
use tokio::sync::RwLock;

#[cfg(feature = "cache")]
use crate::cache::s3fifo::S3Fifo;

pub mod error;

#[cfg(feature = "cache")]
pub mod cache;

pub mod args;

pub mod server;

pub mod handler;
// pub mod plugin;

pub const ADDR: [u8; 4] = [127, 0, 0, 1];


#[cfg(feature = "cache")]
#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<RwLock<S3Fifo<String, Vec<u8>>>>,
}