use chrono::{DateTime, Utc};
use dashmap::{DashMap, DashSet};
use std::{hash::{DefaultHasher, Hash, Hasher}, sync::Arc};
use tokio::sync::Mutex;
use crate::ImgInfo;

#[derive(Clone)]
pub struct ImageCache {
    ttl_mem_db: DashMap<DateTime<Utc>, u64>,
    ttl_storage_db: DashMap<DateTime<Utc>, u64>,
    mem_db: DashMap<u64, Vec<u8>>,
    storage_db: DashSet<u64>,
}

impl ImageCache {
    pub fn new_cache() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            ttl_mem_db: DashMap::new(),
            ttl_storage_db: DashMap::new(),
            mem_db: DashMap::new(),
            storage_db: DashSet::new(),
        }))
    }

    pub async fn insert(&mut self, image_bytes: Vec<u8>, imginfo: ImgInfo) {
        let computed_hash = ImageCache::get_hash(&imginfo);

        let utc: DateTime<Utc> = Utc::now();

        self.ttl_mem_db.insert(utc, computed_hash);
        self.ttl_storage_db.insert(utc, computed_hash);
        self.mem_db.insert(computed_hash, image_bytes.clone());
        self.storage_db.insert(computed_hash);

        ImageCache::cache_in_storage(&computed_hash, &image_bytes).await
    }

    pub fn get_hash(imginfo: &ImgInfo) -> u64 {
        let mut hasher = DefaultHasher::new();
        imginfo.hash(&mut hasher);
        hasher.finish()
    }

    pub async fn get(&mut self, hash: u64) -> Option<Vec<u8>> {
        let mdb = &self.mem_db;
        if mdb.contains_key(&hash) {
            // println!("Found in Tier 1 cache (Memory)");
            let byte_val = mdb.get(&hash).expect("Unable to get bytes").to_vec();
            return Some(byte_val);
            
        } else {
            let sdb = &self.storage_db;
            let read_path = format!("./cache/{}", hash.to_string());
            if sdb.contains(&hash) {
                // println!("Found in Tier 2 cache db (Disc) -> Transferring to Tier 1 cache (Memory)");
                let read_bytes = tokio::fs::read(read_path)
                    .await
                    .expect("Unable to read cache");
                mdb.insert(hash, read_bytes.clone());
                Some(read_bytes)
            } else if tokio::fs::try_exists(&read_path)
                .await
                .expect("Unable to check")
            {
                // println!("Found in Tier 2 cache (Disc) -> Transferring to Tier 1 cache (Memory)");
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
