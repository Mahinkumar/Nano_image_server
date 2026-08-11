use std::path::PathBuf;


pub mod logging;
pub mod app;
pub mod handler;
pub mod plugin;

#[derive(Clone)]
pub struct AppState {
    pub base_dir: PathBuf
}
