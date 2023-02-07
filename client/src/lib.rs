use std::thread;

use config::Config;
use input::{Input, InputType};
use window::Window;
use shared::{registry::Registry, Module, InnerModule};
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
    pub config: Module<Config>,

    pub registry: Module<Registry>,
    pub input: Module<Input>,

    pub world: Module<World>,
    pub window: Module<Window>,
}

impl Client {
    fn new(config: Config) -> Self {
        let config = Module::new(config.into());

        let registry = Registry::new().to_module();
        let input = Input::new().to_module();

        let world = World::new().to_module();
        let window = Window::new().to_module();

        Self {
            config,

            registry,
            input,

            world,
            window,
        }
    }
}

pub fn init<I>(config: Config, input: Vec<(impl Into<String>, Vec<InputType>)>, init: I) where I: FnOnce(&mut Client) {
    let mut client = Client::new(config);
    init(&mut client);

    client.input.write().unwrap().add_actions(input);

    let args = client.world.clone();
    thread::spawn(|| {
        World::run(args, ());
    });

    let args = (client.window.clone(), client.config.clone(), client.input.clone(), client.world.clone());
    Window::run(args.0, (args.1, args.2, args.3));
}