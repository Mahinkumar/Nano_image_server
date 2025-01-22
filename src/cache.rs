
use std::{hash::{DefaultHasher, Hash, Hasher}, path::Path};
use serde::Deserialize;
use tokio::fs;

use crate::ImgInfo;
const MAX_CACHE_SIZE_BYTES: u64 = 1024 * 1024 * 1024;

#[derive(Deserialize,Hash)]
pub struct ImageCache{
    info: ImgInfo,
    hash: u64
}


impl ImageCache{
    pub fn new(imginfo: ImgInfo)->ImageCache{
        let mut hasher = DefaultHasher::new();
        imginfo.hash(&mut hasher);
        let computed_hash = hasher.finish();
        ImageCache{info:imginfo , hash:computed_hash}
    }

    pub fn get_hash(&self)->u64{
        self.hash
    }
}



pub async fn try_cleanup_cache(cache_dir: &str){
    let mut total_size = 0;
    let mut files = Vec::new();
    
    let mut entries = fs::read_dir(cache_dir).await.expect("Unable to read dir");
    while let Some(entry) = entries.next_entry().await.expect("Unable to get entry") {
        let metadata = entry.metadata().await.expect("Unable to get metadata");
        total_size += metadata.len();
        files.push((entry.path(), metadata.modified().expect("Unable to get modified info")));
    }

    if total_size > MAX_CACHE_SIZE_BYTES {
        files.sort_by_key(|&(_, modified)| modified);
        for (path, _) in files {
            if fs::try_exists(&path).await.expect("Unable to access path"){
                total_size -= Path::new(&path).metadata().expect("Unable to compute total size").len();
                fs::remove_file(&path).await.expect("Unable to remove file");
            }
            if total_size <= MAX_CACHE_SIZE_BYTES {
                break;
            }
        }
    }
}
