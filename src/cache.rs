use ahash::RandomState;
use quick_cache::sync::Cache;
use std::fs;
use std::path::Path;

pub struct ImageCache {
    image_cache: Cache<u64, Vec<u8>>,
    preloaded: bool,
    size: usize,
}

impl ImageCache {
    /// Initializes image cache
    /// Cache has key type of u64
    /// Cache has value type of Vec<u8>
    pub fn init_cache(size: usize) -> Self {
        Self {
            image_cache: Cache::new(size),
            preloaded: false,
            size,
        }
    }

    /// Preload from the cache store upto capacity
    /// Loaded based on LRU policy from secondary storage
    /// Recommended to run once on startup of the server
    pub fn preload(mut self) -> Self {
        self.preloaded = true;

        let image_dir = Path::new("./images");

        if !image_dir.exists() || !image_dir.is_dir() {
            println!("Invalid path provided!");
            println!("Returning Immediately");
            return self;
        }

        // Get all files from the directory
        let entries: Vec<_> = fs::read_dir(image_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .collect();

        // Sort by modified time (most recent first)
        // entries.sort_by(|a, b| {
        //     let a_time = a
        //         .metadata()
        //         .and_then(|m| m.modified())
        //         .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
        //     let b_time = b
        //         .metadata()
        //         .and_then(|m| m.modified())
        //         .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
        //     b_time.cmp(&a_time)
        // });

        // Take only the max size allowed entries
        let recent_entries = entries.into_iter().take(self.size);
        let mut total_entries = 0;

        // Read and cache each file
        for entry in recent_entries {
            let path = entry.path();
            if let Some(filename) = path.file_name() {
                if let Some(name_str) = filename.to_str() {
                    let hash = Self::hash(name_str.to_string());
                    total_entries += 1;
                    match fs::read(&path) {
                        Ok(image_data) => {
                            self.set(hash, image_data);
                        }
                        Err(e) => {
                            eprintln!("Failed to read image file {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
        println!("Loaded {total_entries} image(s) into memory");
        self
    }

    /// Hashing with names. Two files cannot have same names hence this is the most
    /// straight forward method. Randome state with the same seed produces identical hashes
    /// in case of a same computer, hash from ahash will vary system to system
    pub fn hash(name: String) -> u64 {
        let hash_builder = RandomState::with_seed(42);
        let hash = hash_builder.hash_one(name);
        hash
    }

    pub fn set(&self, hash: u64, image_data: Vec<u8>) {
        self.image_cache.insert(hash, image_data)
    }

    pub fn get(&self, hash: u64) -> std::option::Option<Vec<u8>> {
        self.image_cache.get(&hash)
    }

    pub fn get_size(&self) -> u64 {
        self.image_cache.capacity()
    }
}
