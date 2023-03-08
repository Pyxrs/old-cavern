use shared::math::Vec3;

pub struct Player {
    pub position: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO
        }
    }
}