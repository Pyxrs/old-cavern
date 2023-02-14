pub struct Config {
    pub addons: ServerAddons,
    pub loading_distance: LoadingDistance,
    pub simulation_distance: SimulationDistance,
}

pub struct ServerAddons(pub String);

pub struct SimulationDistance(pub u16);

pub struct LoadingDistance(pub u16);