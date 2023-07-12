// TODO: Use bbmodel format directly

use std::hash::Hash;
use std::{sync::mpsc::Receiver, thread};

use config::Config;
use input::{Input, InputInfo};
use shared::types::{block, item};
use shared::{registry::Registry, Module, StaticModule};
use std::fmt::Debug;
use window::texture::Texture;
use window::Window;
use world::World;

pub mod config;
pub mod input;
pub mod types;

pub mod interface;
pub mod mesher;
pub mod network;
pub mod sound;
pub mod window;
mod world;

pub struct Client<
    T: Debug + Eq + Hash + PartialEq,
    B: block::Block,
    I: item::Item,
    D,
> {
    pub config: Config,
    pub registry: Registry<T, B, I, D>,
    pub input: Input,
}

pub struct ClientIO {
    pub input_io: Receiver<InputInfo>,
}

//#[profiling::function]
pub fn init<
    State: 'static,
    CustomType: Debug + Eq + Hash + PartialEq + 'static,
    Block: block::Block + 'static,
    Item: item::Item + 'static,
    CustomData: 'static,
    I,
    F,
    R,
    E,
>(
    config: Config,
    init: I,
    frame: F,
    reload: R,
    exit: E,
) where
    I: FnOnce(
        &mut Client<CustomType, Block, Item, CustomData>,
        &ClientIO,
        (&mut World, ()),
    ) -> State,
    F: Fn(&mut State, &mut Client<CustomType, Block, Item, CustomData>, &ClientIO) + 'static,
    R: Fn(
            &mut State,
            &mut Client<CustomType, Block, Item, CustomData>,
            &ClientIO,
        ) -> Resources
        + 'static,
    E: Fn(&mut State, &mut Client<CustomType, Block, Item, CustomData>, &ClientIO) + 'static,
{
    // Thread local
    let registry = Registry::new();
    let (input_io, input) = Input::new(());

    // Threaded
    let (_, mut world) = World::new(());

    let mut client = Client {
        config,
        registry,
        input,
    };

    let client_io = ClientIO { input_io };

    let mut state = init(&mut client, &client_io, (&mut world, ()));

    thread::spawn(|| {
        profiling::register_thread!("World");
        world.run(());
    });

    let resources = reload(&mut state, &mut client, &client_io);

    // TODO: lua vms inside addon manager on separate thread

    let (_, window) = Window::new(());
    window.run::<State, CustomType, Block, Item, CustomData, F, E, R>((state, client, client_io, frame, exit, resources));
}

pub struct Resources {
    pub world_shader: String,
    pub skybox_shader: String,
    pub textures: Vec<((String, String), Texture)>,
}
