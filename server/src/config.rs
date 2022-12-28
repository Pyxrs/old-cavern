#[derive(Debug)]
pub struct Config(pub LoadingDistance, pub SimulationDistance);

#[derive(Debug)]
pub struct SimulationDistance(pub u16);

#[derive(Debug)]
pub struct LoadingDistance(pub u16);