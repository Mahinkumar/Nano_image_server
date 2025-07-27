#[cfg(feature = "processing")]
pub mod compute;

#[cfg(feature = "cache")]
pub mod cache;

pub mod server;

pub const ADDR: [u8; 4] = [127, 0, 0, 1];
