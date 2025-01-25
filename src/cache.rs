
use std::{hash::{DefaultHasher, Hash, Hasher}, sync::Arc};
use dashmap::{DashMap, DashSet};
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};

pub type Db = Arc<Mutex<DashMap<u64, Vec<u8>>>>;

use crate::ImgInfo;

// const MAX_CACHE_SIZE_BYTES: u64 = 1024 * 1024 * 1024;

// ImageCache{info:imginfo,hash:computed_hash}

#[derive(Clone)]
pub struct ImageCache{
    mcache_count: u32,
    scache_count: u32,
    ttl_mem_db:  Arc<Mutex<DashMap<DateTime<Utc>,u64>>>,
    ttl_storage_db:  Arc<Mutex<DashMap<DateTime<Utc>,u64>>>,
    mem_db: Arc<Mutex<DashMap<u64, Vec<u8>>>>,
    storage_db: Arc<Mutex<DashSet<u64>>>
}

impl ImageCache{
    pub fn new_cache()->ImageCache{
        ImageCache{ mcache_count: 0, scache_count: 0, ttl_mem_db: Arc::new(Mutex::new(DashMap::new())), ttl_storage_db: Arc::new(Mutex::new(DashMap::new())) , mem_db: Arc::new(Mutex::new(DashMap::new())), storage_db:Arc::new(Mutex::new(DashSet::new()))}
    }

    pub async fn insert(&mut self,image_bytes: Vec<u8>,imginfo: ImgInfo){
        self.mcache_count += 1;
        self.scache_count += 1;

        let computed_hash = ImageCache::get_hash(&imginfo);

        let utc: DateTime<Utc> = Utc::now();

        self.ttl_mem_db.lock().await.insert(utc,computed_hash);
        self.ttl_storage_db.lock().await.insert(utc,computed_hash);
        self.mem_db.lock().await.insert(computed_hash, image_bytes);
        self.storage_db.lock().await.insert(computed_hash);

        // Insert into storage function here.
    }

    pub fn get_hash(imginfo: &ImgInfo) -> u64{
        let mut hasher = DefaultHasher::new();
        imginfo.hash(&mut hasher);
        hasher.finish()
    }

    pub async fn get(&mut self,hash: u64) -> Option<Vec<u8>>{
        let mdb = self.mem_db.lock().await;
        if mdb.contains_key(&hash){
            return Some(mdb.get(&hash).expect("Unable to get bytes").to_vec());
        }else {
            let sdb = self.storage_db.lock().await;
            if sdb.contains(&hash){
                return None // We return img from the storage here
            }else {
                return None;
            }
        }
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
