pub use wgpu::PolygonMode;

pub struct Config{
    pub resources: Resources,
    pub debug: Debug,
    pub meshing_distance: MeshingDistance,
    pub gamma: Gamma,
}

pub struct Resources(pub Shaders, pub Textures);

pub struct Shaders(pub String);

pub struct Textures(pub String);

pub struct Debug(pub PolygonMode);

pub struct MeshingDistance(pub u16);

pub struct Gamma(pub f32);