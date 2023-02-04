use std::thread;

use client::{
    config::{Gamma, MeshingDistance},
    input::{InputType, Key},
};
use server::config::{LoadingDistance, SimulationDistance};
use shared::extra::{LevelFilter, debug};
use simple_logger::SimpleLogger;

use crate::registry::register_features;

mod registry;

fn main() {
    // Logging
    SimpleLogger::new()
        .with_module_level("wgpu_hal", LevelFilter::Error)
        .with_module_level("wgpu_core", LevelFilter::Error)
        .with_module_level("naga", LevelFilter::Error)
        .with_module_level("winit", LevelFilter::Error)
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();

    // Server
    thread::spawn(|| { server::init(
        server::config::Config(LoadingDistance(12), SimulationDistance(14)),
        |server| {
            register_features(server.registry.write().unwrap());
        },
    )});

    // Client
    client::init(
        client::config::Config(MeshingDistance(12), Gamma(1.0)),
        |client| {
            debug!("WOW1");
            register_features(client.registry.write().unwrap());

            debug!("WOW2");
            client.input.write().unwrap().add_actions(vec![
                ("exit", vec![InputType::Key(Key::Escape)]),
                ("forward", vec![InputType::Key(Key::W)]),
                ("backward", vec![InputType::Key(Key::S)]),
                ("left", vec![InputType::Key(Key::A)]),
                ("right", vec![InputType::Key(Key::D)]),
            ]);
        },
    );
}
