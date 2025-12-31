//! Configuration management

pub mod settings;

pub use settings::{AppConfig, load_config, save_config, config_path};
