use shared::Module;

use self::player::Player;

mod player;

pub struct World {
    pub player: Player,
}

impl Module<(), ()> for World {
    fn new() -> ((), Self) {
        ((), Self {
            player: Player::new(),
        })
    }

    fn run(self, _args: ()) {
    }
}