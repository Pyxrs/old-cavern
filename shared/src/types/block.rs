pub trait Block {
    type Args;

    fn on_random_update(&mut self, args: Self::Args, visible: bool);
    fn on_neighbor_update(&mut self, args: Self::Args, visible: bool);
    fn on_place(&mut self, args: Self::Args);
    fn on_destroy(&mut self, args: Self::Args);
    fn on_interact(&mut self, args: Self::Args);
    fn on_collision(&mut self, args: Self::Args);
    fn can_place(&self, args: Self::Args) -> bool;
    fn can_interact(&self, args: Self::Args) -> bool;
    fn can_pathfind(&self, args: Self::Args) -> PathfindingState;
}

pub enum PathfindingState {
    Solid,
    Dangerous,
    Empty,
}
