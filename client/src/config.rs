pub use wgpu::PolygonMode;

pub struct Config {
    pub debug: Debug,
    pub meshing_distance: MeshingDistance,
    pub gamma: Gamma,
}

pub struct Debug(pub PolygonMode);

pub struct MeshingDistance(pub u16);

pub struct Gamma(pub f32);