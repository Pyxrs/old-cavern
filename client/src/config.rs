pub use wgpu::PolygonMode;

pub struct Config {
    pub shader: Shader,
    pub addons: ClientAddons,
    pub debug: Debug,
    pub meshing_distance: MeshingDistance,
    pub gamma: Gamma,
}

pub struct Shader(pub String);

pub struct ClientAddons(pub String);

pub struct Debug(pub PolygonMode);

pub struct MeshingDistance(pub u16);

pub struct Gamma(pub f32);