#[derive(Debug)]
pub struct Config(pub MeshingDistance, pub Gamma);

#[derive(Debug)]
pub struct MeshingDistance(pub u16);

#[derive(Debug)]
pub struct Gamma(pub f32);