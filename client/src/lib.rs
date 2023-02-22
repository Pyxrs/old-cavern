use std::thread;

use config::Config;
use input::{Input, InputInfo};
use shared::{addons::AddonManager, extra::Receiver, Module, StaticModule};
use window::Window;
use world::World;

pub mod config;
pub mod input;
pub mod interface;
pub mod mesher;
pub mod network;
pub mod sound;
pub mod window;
mod world;

pub struct Client {
    pub config: Config,
    pub addon_manager: AddonManager,
    pub input: Input,
}

pub struct ClientIO {
    pub input_io: Receiver<InputInfo>,
}

pub fn init<I, F, S: 'static>(config: Config, init: I, frame: F)
where
    I: FnOnce(&mut Client, &ClientIO, (&mut World, ())) -> S,
    F: Fn(&mut S, &mut Client, &ClientIO) + 'static,
{
    // Thread local
    let addon_manager = AddonManager::load(config.addons.0.clone());
    let (input_io, input) = Input::new();

    // Threaded
    let (_, mut world) = World::new();

    let mut client = Client {
        config,
        addon_manager,
        input,
    };

    let client_io = ClientIO {
        input_io,
    };

    let state = init(
        &mut client,
        &client_io,
        (
            &mut world,
            ()
        ),
    );

    thread::spawn(|| {
        world.run(());
    });

    // TODO: lua vms inside addon manager on separate thread

    let (_, window) = Window::new();
    window.run((state, client, client_io, frame));
}
