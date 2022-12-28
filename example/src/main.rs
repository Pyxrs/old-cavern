use client::config::{MeshingDistance, Gamma};
use server::config::{SimulationDistance, LoadingDistance};

fn main() {
    simple_logger::init().unwrap();

    println!("Hello, world!");
    client::init(client::config::Config(
        MeshingDistance(12),
        Gamma(1.0)
    ));
    server::init(server::config::Config(
        LoadingDistance(12),
        SimulationDistance(14),
    ));
}
