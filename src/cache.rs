
use std::hash::{DefaultHasher, Hash, Hasher};
use serde::Deserialize;

use crate::ImgInfo;


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