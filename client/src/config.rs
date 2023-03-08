pub use wgpu::PolygonMode;

pub struct Config {
    pub shaders: Shaders,
    pub addons: ClientAddons,
    pub debug: Debug,
    pub meshing_distance: MeshingDistance,
    pub gamma: Gamma,
}

pub struct Shaders(pub String, pub String);

pub struct ClientAddons(pub String);

pub struct Debug(pub PolygonMode);

pub struct MeshingDistance(pub u16);

pub struct Gamma(pub f32);