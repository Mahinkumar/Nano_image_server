use std::path::PathBuf;

use crate::plugin::Plugins;


pub mod logging;
pub mod app;
pub mod handler;
pub mod plugin;

#[derive(Clone)]
pub struct AppState {
    pub base_dir: PathBuf,
    pub plugins: Plugins
}
