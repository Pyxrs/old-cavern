use std::{thread, vec};

use client::{
    config::{Gamma, MeshingDistance, Resources, Shaders, Textures, Debug, PolygonMode},
    input::{InputType, Key},
};
use registry::register_server_features;
use server::config::{LoadingDistance, SimulationDistance};
use shared::{extra::LevelFilter, resources};
use simple_logger::SimpleLogger;

use crate::registry::register_client_features;

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
        server::config::Config(
            LoadingDistance(12),
            SimulationDistance(14)
        ),
        |server| {
            register_server_features(server.registry.write().unwrap());
        },
    )});

    // Client
    client::init(
        client::config::Config {
            resources: Resources(
                Shaders(resources::read_string("example/resources/shader.wgsl").unwrap()),
                Textures("example/resources/textures/".to_string()),
            ),
            debug: Debug(
                PolygonMode::Fill,
            ),
            meshing_distance: MeshingDistance(12),
            gamma: Gamma(1.0),
        },
        vec![
            ("exit", vec![InputType::Key(Key::Escape)]),
            ("forward", vec![InputType::Key(Key::W)]),
            ("backward", vec![InputType::Key(Key::S)]),
            ("left", vec![InputType::Key(Key::A)]),
            ("right", vec![InputType::Key(Key::D)]),
            ("up", vec![InputType::Key(Key::Space)]),
            ("down", vec![InputType::Key(Key::LShift)]),
        ],
        |client| {
            register_client_features(client.registry.write().unwrap());
        },
    );
}