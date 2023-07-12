// TODO: Rewrite to be like client

use config::Config;
use shared::registry::Registry;

pub mod pathfinding;
pub mod terrain;
pub mod config;
pub mod network;

pub struct Server {
    pub config: Config,
    //pub registry: Registry<D>,
}

pub struct ServerIO {
}

#[profiling::function]
pub fn init<I, F, S>(config: Config, init: I, frame: F)
where
    I: FnOnce(&mut Server, &ServerIO, ()) -> S,
    F: Fn(&mut S, &mut Server, &ServerIO),
{
    //let registry = Registry::new();

    let mut server = Server {
        config,
        //registry,
    };

    let server_io = ServerIO {
    };

    let mut state = init(&mut server, &server_io, ());

    loop {
        tick(&frame, &mut state, &mut server, &server_io);
    }
}

fn tick<F, S>(frame: &F, state: &mut S, server: &mut Server, server_io: &ServerIO) where F: Fn(&mut S, &mut Server, &ServerIO) {
    frame(state, server, server_io);
}