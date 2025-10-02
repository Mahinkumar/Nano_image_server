use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};

pub struct S3Fifo<K, V> {
    cache: HashMap<K, CacheEntry<V>>,
    
    small: VecDeque<K>,
    main: VecDeque<K>,
    ghost: VecDeque<K>,
    
    small_capacity: usize,
    main_capacity: usize,
    ghost_capacity: usize,
    
    hits: AtomicU64,
    misses: AtomicU64,
}


struct CacheEntry<V> {
    value: V,
    freq: AtomicU8,
    location: QueueLocation,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum QueueLocation {
    Small,
    Main,
    Ghost,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub size: usize,
    pub capacity: usize,
}

impl<K, V> S3Fifo<K, V> 
where 
    K: Hash + Eq + Clone,
{
    pub fn new(capacity: usize) -> Self {
        let small_capacity = capacity / 10;
        let main_capacity = capacity - small_capacity;
        let ghost_capacity = capacity;
        
        Self {
            cache: HashMap::new(),
            small: VecDeque::new(),
            main: VecDeque::new(),
            ghost: VecDeque::new(),
            small_capacity,
            main_capacity,
            ghost_capacity,
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }
    
    pub fn stats(&self) -> CacheStats {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 {
            (hits as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        CacheStats {
            hits,
            misses,
            hit_rate,
            size: self.small.len() + self.main.len(),
            capacity: self.small_capacity + self.main_capacity,
        }
    }
    
    pub fn reset_stats(&self) {
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
    }
    
    
    fn evict_from_small(&mut self) {
        // Once removed, read the frequency counter of the removed entry with the key
        // Read frequency without holding a mutable borrow
        //
        // Evict the entry if the frequency is 0
        //
        // if the frequency is 1 we re add the entry into small FIFO queue
        // Reset the frequency to 0
        // Push the entry again back into small
        //
        // if frequency is greater than 1 promote entry to main queue
        // Check capacity BEFORE getting mutable borrow
        // Mutate entry after evict_from_main is done
        // Push the key to main
        while let Some(key) = self.small.pop_front() {
            let freq = if let Some(entry) = self.cache.get(&key) {
                entry.freq.load(Ordering::Relaxed)
            } else {
                continue;
            };

            if freq == 0 {
                self.move_to_ghost(key);
                return;
            } else if freq == 1 {
                if let Some(entry) = self.cache.get_mut(&key) {
                    entry.freq.store(0, Ordering::Relaxed);
                }
                self.small.push_back(key);
            } else {
                if self.main.len() >= self.main_capacity {
                    self.evict_from_main();
                }
                
                if let Some(entry) = self.cache.get_mut(&key) {
                    entry.freq.store(freq - 1, Ordering::Relaxed);
                    entry.location = QueueLocation::Main;
                }
                self.main.push_back(key);
                return;
            }
        }
    }
    
    fn evict_from_main(&mut self) {
        while let Some(key) = self.main.pop_front() {
            let freq = if let Some(entry) = self.cache.get(&key) {
                entry.freq.load(Ordering::Relaxed)
            } else {
                continue;
            };
            
            if freq == 0 {
                self.move_to_ghost(key);
                return;
            } else {
                // Give it another chance
                if let Some(entry) = self.cache.get_mut(&key) {
                    entry.freq.store(freq - 1, Ordering::Relaxed);
                }
                self.main.push_back(key);
            }
        }
    }
    
    fn move_to_ghost(&mut self, key: K) {
        if let Some(entry) = self.cache.get_mut(&key) {
            entry.location = QueueLocation::Ghost;
            entry.freq.store(0, Ordering::Relaxed);
            
            self.ghost.push_back(key.clone());
            
            if self.ghost.len() > self.ghost_capacity {
                if let Some(old_ghost) = self.ghost.pop_front() {
                    self.cache.remove(&old_ghost);
                }
            }
        }
    }
}

impl<K, V> crate::cache::Cache<K, V> for S3Fifo<K, V>
where
    K: Hash + Eq + Clone,
{
    fn get(&self, key: &K) -> Option<&V> {
        match self.cache.get(key) {
            Some(entry) if entry.location != QueueLocation::Ghost => {
                self.hits.fetch_add(1, Ordering::Relaxed);
                
                let _ = entry.freq.fetch_update(
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                    |freq| if freq < 3 { Some(freq + 1) } else { None }
                );
                Some(&entry.value)
            }
            _ => {
                self.misses.fetch_add(1, Ordering::Relaxed);
                None
            }
        }
    }
    
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(entry) = self.cache.get_mut(&key) {
            if entry.location != QueueLocation::Ghost {
                let old_value = std::mem::replace(&mut entry.value, value);
                entry.freq.store(1, Ordering::Relaxed);
                return Some(old_value);
            }
        }
        
        if let Some(entry) = self.cache.get(&key) {
            if entry.location == QueueLocation::Ghost {
                self.ghost.retain(|k| k != &key);
                self.cache.remove(&key);
                
                if self.main.len() >= self.main_capacity {
                    self.evict_from_main();
                }
                
                self.main.push_back(key.clone());
                self.cache.insert(key, CacheEntry {
                    value,
                    freq: AtomicU8::new(1),
                    location: QueueLocation::Main,
                });
                return None;
            }
        }
        
        if self.small.len() >= self.small_capacity {
            self.evict_from_small();
        }
        
        self.small.push_back(key.clone());
        self.cache.insert(key, CacheEntry {
            value,
            freq: AtomicU8::new(0),
            location: QueueLocation::Small,
        });
        
        None
    }
    
    fn contains(&self, key: &K) -> bool {
        self.cache.get(key)
            .map(|entry| entry.location != QueueLocation::Ghost)
            .unwrap_or(false)
    }
    
    fn len(&self) -> usize {
        self.small.len() + self.main.len()
    }
    
    fn clear(&mut self) {
        self.cache.clear();
        self.small.clear();
        self.main.clear();
        self.ghost.clear();
    }
}