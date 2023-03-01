use std::{thread, vec};

use client::{
    config::{ClientAddons, Debug, Gamma, MeshingDistance, PolygonMode, Shader},
    input::{InputType, Key},
};
use server::config::{LoadingDistance, SimulationDistance, ServerAddons};
use shared::{extra::{LevelFilter, info}, resources};
use simple_logger::SimpleLogger;

fn main() {
    // Run `cargo run --features=profile` to profile using optick
    if cfg!(feature = "profile") {
        optick::start_capture();
        info!("Profiling started");
    }

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
        profiling::register_thread!("Server");
        server::init(
            server::config::Config {
                addons: ServerAddons("example/addons/".to_string()),
                loading_distance: LoadingDistance(12),
                simulation_distance: SimulationDistance(14),
            },
            |_server, _server_io, _modules| {
            },
            |_state, _server, _server_io| {
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
        |client, _client_io, _modules| {
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
        |_state, _client, _client_io| {
        },
        |_state, _client, _client_io| {
            if cfg!(feature = "profile") {
                optick::stop_capture("Profile");
            }
        },
    );
}