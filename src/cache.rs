use chrono::{DateTime, Utc};
use dashmap::{DashMap, DashSet};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<DashMap<u64, Vec<u8>>>>;

use crate::ImgInfo;

// const MAX_CACHE_SIZE_BYTES: u64 = 1024 * 1024 * 1024;

// ImageCache{info:imginfo,hash:computed_hash}

#[derive(Clone)]
pub struct ImageCache {
    ttl_mem_db: Arc<Mutex<DashMap<DateTime<Utc>, u64>>>,
    ttl_storage_db: Arc<Mutex<DashMap<DateTime<Utc>, u64>>>,
    mem_db: Arc<Mutex<DashMap<u64, Vec<u8>>>>,
    storage_db: Arc<Mutex<DashSet<u64>>>,
}

impl ImageCache {
    pub fn new_cache() -> ImageCache {
        ImageCache {
            ttl_mem_db: Arc::new(Mutex::new(DashMap::new())),
            ttl_storage_db: Arc::new(Mutex::new(DashMap::new())),
            mem_db: Arc::new(Mutex::new(DashMap::new())),
            storage_db: Arc::new(Mutex::new(DashSet::new())),
        }
    }

    pub async fn insert(&mut self, image_bytes: Vec<u8>, imginfo: ImgInfo) {
        let computed_hash = ImageCache::get_hash(&imginfo);

        let utc: DateTime<Utc> = Utc::now();

        self.ttl_mem_db.lock().await.insert(utc, computed_hash);
        self.ttl_storage_db.lock().await.insert(utc, computed_hash);
        self.mem_db
            .lock()
            .await
            .insert(computed_hash, image_bytes.clone());
        self.storage_db.lock().await.insert(computed_hash);

        ImageCache::cache_in_storage(&computed_hash, &image_bytes).await
    }

    pub fn get_hash(imginfo: &ImgInfo) -> u64 {
        let mut hasher = DefaultHasher::new();
        imginfo.hash(&mut hasher);
        hasher.finish()
    }

    pub async fn get(&mut self, hash: u64) -> Option<Vec<u8>> {
        let mdb = self.mem_db.lock().await;
        if mdb.contains_key(&hash) {
            println!("Found in Tier 1 cache (Memory)");
            return Some(mdb.get(&hash).expect("Unable to get bytes").to_vec());
        } else {
            let sdb = self.storage_db.lock().await;
            let read_path = format!("./cache/{}", hash.to_string());
            if sdb.contains(&hash) {
                println!(
                    "Found in Tier 2 cache db (Disc) -> Transferring to Tier 1 cache (Memory)"
                );
                let read_bytes = tokio::fs::read(read_path)
                    .await
                    .expect("Unable to read cache");
                mdb.insert(hash, read_bytes.clone());
                Some(read_bytes)
            } else if tokio::fs::try_exists(&read_path)
                .await
                .expect("Unable to check")
            {
                println!("Found in Tier 2 cache (Disc) -> Transferring to Tier 1 cache (Memory)");
                let read_bytes = tokio::fs::read(read_path)
                    .await
                    .expect("Unable to read bytes");
                mdb.insert(hash, read_bytes.clone());
                Some(read_bytes)
            } else {
                return None;
            }
        }
    }

    pub async fn cache_in_storage(hash: &u64, image_bytes: &Vec<u8>) {
        let write_path = format!("./cache/{}", hash.to_string());
        tokio::fs::write(write_path, &image_bytes)
            .await
            .expect("Unable to write");
    }
}

// pub async fn try_cleanup_cache(cache_dir: &str){
//     let mut total_size = 0;
//     let mut files = Vec::new();

//     let mut entries = fs::read_dir(cache_dir).await.expect("Unable to read dir");
//     while let Some(entry) = entries.next_entry().await.expect("Unable to get entry") {
//         let metadata = entry.metadata().await.expect("Unable to get metadata");
//         total_size += metadata.len();
//         files.push((entry.path(), metadata.modified().expect("Unable to get modified info")));
//     }

//     if total_size > MAX_CACHE_SIZE_BYTES {
//         files.sort_by_key(|&(_, modified)| modified);
//         for (path, _) in files {
//             if fs::try_exists(&path).await.expect("Unable to access path"){
//                 total_size -= Path::new(&path).metadata().expect("Unable to compute total size").len();
//                 fs::remove_file(&path).await.expect("Unable to remove file");
//             }
//             if total_size <= MAX_CACHE_SIZE_BYTES {
//                 break;
//             }
//         }
//     }
// }
