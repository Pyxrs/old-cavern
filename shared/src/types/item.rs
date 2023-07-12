pub trait Item {
    type Args;

    fn on_use(&mut self, args: Self::Args, target: Target, hand: Hand);
    fn on_update(&mut self, args: Self::Args, location: Location);
}

pub enum Target {
    Air,
    Block,
    Entity,
}

pub enum Hand {
    MainHand,
    OffHand
}

pub enum Location {
    Inventory,
    Hotbar,
    Hand(Hand),
}