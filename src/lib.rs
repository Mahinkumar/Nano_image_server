#[cfg(feature = "processing")]
pub mod processing;

#[cfg(feature = "processing")]
pub mod transform;

pub mod server;

#[cfg(feature = "processing")]
pub mod utils;

pub const ADDR: [u8; 4] = [127, 0, 0, 1];
