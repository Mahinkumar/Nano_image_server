use async_trait::async_trait;
use quick_cache::sync::Cache;

use super::*;

struct MemoryCache {
    cache: Cache<u64, Vec<u8>>,
}

#[async_trait]
impl ImageCache for MemoryCache {
    async fn init(size: usize) -> Self {
        Self {
            cache: Cache::new(size),
        }
    }

    async fn insert(&self, key: u64, image_stream: Vec<u8>) {
        self.cache.insert(key, image_stream)
    }

    async fn retrieve(&self, key: u64) -> Result<Vec<u8>, CacheError> {
        match self.cache.get(&key) {
            None => Err(CacheError::KeyNotFound(key)),
            Some(val) => Ok(val),
        }
    }

    async fn clear(&self, key: u64) -> Result<Vec<u8>, CacheError> {
        match self.cache.remove(&key) {
            None => Err(CacheError::KeyNotFound(key)),
            Some(val) => Ok(val.1),
        }
    }
}
