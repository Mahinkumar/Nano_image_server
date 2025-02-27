use ahash::RandomState;
use quick_cache::sync::Cache;

struct ImageCache {
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
    fn preload(mut self) -> Self {
        self.preloaded = true;
        // Preload from storage to here based on indexes
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

    pub fn set(&self,hash: u64, image_data: Vec<u8>){
        self.image_cache.insert(hash, image_data)
    }

    pub fn get(&self,hash: u64)-> std::option::Option<Vec<u8>>{
        self.image_cache.get(&hash)
    }
}
