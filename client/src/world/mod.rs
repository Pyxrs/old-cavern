use shared::{InnerModule, Module};

use self::player::Player;

mod player;

pub struct World {
    pub player: Player,
}

impl World {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }
}

impl InnerModule<()> for World {
    fn run(module: Module<Self>, modules: ()) {
        //
    }
}