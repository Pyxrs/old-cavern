use shared::Module;

use self::player::Player;

mod player;

pub struct World {
    pub player: Player,
}

impl Module<(), (), ()> for World {
    #[profiling::function]
    fn new(_: ()) -> ((), Self) {
        ((), Self {
            player: Player::new(),
        })
    }

    #[profiling::function]
    fn run(self, _args: ()) {
    }
}