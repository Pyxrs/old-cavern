use std::sync::{Arc, RwLock};

use config::Config;
use shared::{extra::info, registry::{Registry, types::ServerRegistryType}, Module};

pub type ServerRegistry = Registry<ServerRegistryType>;

pub mod pathfinding;
pub mod network;
pub mod terrain;
pub mod config;

pub struct Server {
    pub registry: Module<ServerRegistry>,
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