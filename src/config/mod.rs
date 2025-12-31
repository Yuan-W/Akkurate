//! Configuration management

pub mod settings;

pub use settings::{config_path, load_config, save_config, AppConfig};
