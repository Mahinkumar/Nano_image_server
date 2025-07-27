pub mod s3fifo;

pub trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn contains(&self, key: &K) -> bool;
    fn len(&self) -> usize;
    fn clear(&mut self);
}
