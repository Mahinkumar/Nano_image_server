#[cfg(feature = "processing")]
pub mod processing;

pub mod transform;

pub mod server;
pub mod utils;

pub const ADDR: [u8; 4] = [127, 0, 0, 1];
