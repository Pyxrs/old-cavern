use std::sync::{Arc, RwLock};

use config::Config;
use shared::{extra::info, registry::Registry};

pub mod pathfinding;
pub mod network;
pub mod terrain;
pub mod config;

pub struct Server {
    pub registry: Arc<RwLock<Registry>>,
}

impl Server {
    fn new(config: Config) -> Self {
        Self {
            registry: Arc::new(Registry::new().into()),
        }
    }
}

pub fn init<I>(config: Config, init: I) where
    I: FnOnce(&mut Server),
{
    info!("Config: {:?}", config);
    let mut server = Server::new(config);
    init(&mut server);

}