pub use wgpu::PolygonMode;

pub struct Config{
    pub resources: Resources,
    pub debug: Debug,
    pub meshing_distance: MeshingDistance,
    pub gamma: Gamma,
}

pub struct Resources(pub Shader, pub Texture);

pub struct Shader(pub String);

pub struct Texture(pub Vec<u8>);

pub struct Debug(pub PolygonMode);

pub struct MeshingDistance(pub u16);

pub struct Gamma(pub f32);