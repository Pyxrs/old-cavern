use shared::extra::{Vector3, Zero};

pub struct Player {
    pub position: Vector3<f32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector3::zero()
        }
    }
}