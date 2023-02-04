use std::thread;

use config::Config;
use input::Input;
use window::Window;
use shared::{extra::info, registry::Registry, Module, InnerModule};
use world::World;

pub mod interface;
pub mod mesher;
pub mod network;
pub mod sound;
pub mod window;
pub mod config;
pub mod input;
mod world;

pub struct Client {
    pub registry: Module<Registry>,
    pub input: Module<Input>,

    pub world: Module<World>,
    pub window: Module<Window>,
}

impl Client {
    fn new(_config: Config) -> Self {
        let registry = Registry::new().to_module();
        let input = Input::new().to_module();

        let world = World::new().to_module();
        let window = Window::new().to_module();

        Self {
            registry,
            input,

            world,
            window,
        }
    }
}

pub fn init<I>(config: Config, init: I) where I: FnOnce(&mut Client) {
    info!("Config: {:?}", config);
    let mut client = Client::new(config);
    init(&mut client);

    let args = client.world.clone();
    thread::spawn(|| {
        World::run(args, ());
    });

    let args = (client.window.clone(), client.input.clone(), client.world.clone());
    Window::run(args.0, (args.1, args.2));
}