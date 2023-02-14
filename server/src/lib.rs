use config::Config;
use shared::{Module, addons::AddonManager, InnerModule};

pub mod pathfinding;
pub mod network;
pub mod terrain;
pub mod config;

pub struct Server {
    pub config: Module<Config>,
    pub addon_manager: Module<AddonManager>,
}

impl Server {
    fn new(config: Config) -> Self {
        Self {
            config: Module::new(config.into()),
            addon_manager: AddonManager::new().to_module(),
        }
    }
}

pub fn init<I>(config: Config, init: I) where
    I: FnOnce(&mut Server),
{
    let mut server = Server::new(config);
    init(&mut server);
}