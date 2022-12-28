use config::Config;
use shared::prelude::info;

pub mod config;

pub fn init(config: Config) {
    info!("Config: {:?}", config);
}