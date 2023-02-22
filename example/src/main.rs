use std::{thread, vec};

use client::{
    config::{ClientAddons, Debug, Gamma, MeshingDistance, PolygonMode, Shader},
    input::{InputType, Key},
};
use server::config::{LoadingDistance, SimulationDistance, ServerAddons};
use shared::{extra::LevelFilter, resources};
use simple_logger::SimpleLogger;

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
    thread::spawn(|| {
        server::init(
            server::config::Config {
                addons: ServerAddons("example/addons/".to_string()),
                loading_distance: LoadingDistance(12),
                simulation_distance: SimulationDistance(14),
            },
            |server, server_io, modules| {
            },
            |state, server, server_io| {
            },
        )
    });

    client::init(
        client::config::Config {
            shader: Shader(resources::read_string("example/shader.wgsl").unwrap()),
            addons: ClientAddons("example/addons/".to_string()),
            debug: Debug(PolygonMode::Fill),
            meshing_distance: MeshingDistance(12),
            gamma: Gamma(1.0),
        },
        |client, client_io, modules| {
            client.input.add_actions(vec![
                ("exit", vec![InputType::Key(Key::Escape)]),
                ("forward", vec![InputType::Key(Key::W)]),
                ("backward", vec![InputType::Key(Key::S)]),
                ("left", vec![InputType::Key(Key::A)]),
                ("right", vec![InputType::Key(Key::D)]),
                ("up", vec![InputType::Key(Key::Space)]),
                ("down", vec![InputType::Key(Key::LShift)]),
            ]);
        },
        |state, client, client_io| {
        },
    );
}