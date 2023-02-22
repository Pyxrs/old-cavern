use config::Config;
use shared::addons::AddonManager;

pub mod pathfinding;
pub mod network;
pub mod terrain;
pub mod config;

pub struct Server {
    pub config: Config,
    pub addon_manager: AddonManager,
}

pub struct ServerIO {
}

pub fn init<I, F, S>(config: Config, init: I, frame: F)
where
    I: FnOnce(&mut Server, &ServerIO, ()) -> S,
    F: Fn(&mut S, &mut Server, &ServerIO),
{
    let addon_manager = AddonManager::load(config.addons.0.clone());

    let mut server = Server {
        config,
        addon_manager
    };

    let server_io = ServerIO {
    };

    let mut state = init(&mut server, &server_io, ());

    loop {
        frame(&mut state, &mut server, &server_io)
    }
}