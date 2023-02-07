use std::{thread, vec};

use client::{
    config::{Gamma, MeshingDistance, Resources, Shader, Texture, Debug, PolygonMode},
    input::{InputType, Key},
};
use server::config::{LoadingDistance, SimulationDistance};
use shared::{extra::LevelFilter, resources};
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
        server::config::Config(
            LoadingDistance(12),
            SimulationDistance(14)
        ),
        |server| {
            register_features(server.registry.write().unwrap());
        },
    )});

    // Client
    client::init(
        client::config::Config {
            resources: Resources(
                Shader(resources::load_string("example/resources/shader.wgsl").unwrap()),
                Texture(resources::load_bytes("example/resources/happy-tree.png").unwrap()),
            ),
            debug: Debug(
                PolygonMode::Line,
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
            register_features(client.registry.write().unwrap());
        },
    );
}