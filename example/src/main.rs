use std::{thread, vec};

use client::{
    config::{Debug, Gamma, MeshingDistance, PolygonMode},
    input::{InputType, Key}, Resources, declare_block,
};
use server::config::{LoadingDistance, SimulationDistance};
use shared::{log::{LevelFilter, info}, resources, types::item::Item};
use simple_logger::SimpleLogger;

#[profiling::function]
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
    /*thread::spawn(|| {
        profiling::register_thread!("Server");
        server::init(
            server::config::Config {
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
            let textures = vec![]; // TODO: load textures

            Resources {
                world_shader: resources::read_string("example/world_shader.wgsl").unwrap(),
                skybox_shader: resources::read_string("example/skybox_shader.wgsl").unwrap(),
                textures,
            }
        },
        |_state, _client, _client_io| {
            if cfg!(feature = "profile") {
                optick::stop_capture("Profile");
            }
        },
    );*/
}