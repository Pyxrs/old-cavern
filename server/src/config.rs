pub struct Config {
    pub loading_distance: LoadingDistance,
    pub simulation_distance: SimulationDistance,
}

pub struct SimulationDistance(pub u16);

pub struct LoadingDistance(pub u16);