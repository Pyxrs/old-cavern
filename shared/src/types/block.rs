use std::fmt::Debug;

use cgmath::Vector3;

use crate::util::{BoundingBox, zero::Zeroable};

pub struct Block {
    pub namespace: String,
    pub id: u32,

    pub on_random_update: Box<dyn Fn(&mut Self, bool) + Send + Sync>,
    pub on_neighbor_update: Box<dyn Fn(&mut Self, bool) + Send + Sync>,
    pub on_place: Box<dyn Fn(&mut Self) + Send + Sync>,
    pub on_destroy: Box<dyn Fn(&mut Self) + Send + Sync>,
    pub on_interact: Box<dyn Fn(&mut Self) + Send + Sync>,
    pub on_collision: Box<dyn Fn(&mut Self) + Send + Sync>,
    pub can_place: Box<dyn Fn(&Self) -> bool + Send + Sync>,
    pub can_interact: Box<dyn Fn(&Self) -> bool + Send + Sync>,

    pub bounding_box: BoundingBox,
    pub pathfinding_state: PathfindingState,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Block").field("namespace", &self.namespace).field("id", &self.id).finish()
    }
}

pub enum PathfindingState {
    Solid,
    Dangerous,
    Empty,
}

pub struct BlockBuilder {
    namespace: String,
    id: u32,

    on_random_update: Option<Box<dyn Fn(&mut Block, /*Visible*/ bool) + Send + Sync>>,
    on_neighbor_update: Option<Box<dyn Fn(&mut Block, /*Visible*/ bool) + Send + Sync>>,
    on_place: Option<Box<dyn Fn(&mut Block) + Send + Sync>>,
    on_destroy: Option<Box<dyn Fn(&mut Block) + Send + Sync>>,
    on_interact: Option<Box<dyn Fn(&mut Block) + Send + Sync>>,
    on_collision: Option<Box<dyn Fn(&mut Block) + Send + Sync>>,
    can_place: Option<Box<dyn Fn(&Block) -> bool + Send + Sync>>,
    can_interact: Option<Box<dyn Fn(&Block) -> bool + Send + Sync>>,

    bounding_box: Option<BoundingBox>,
    pathfinding_state: Option<PathfindingState>,
}

impl BlockBuilder {
    pub fn new(namespace: impl Into<String>, id: u32) -> BlockBuilder {
        BlockBuilder {
            namespace: namespace.into(),
            id,
            on_random_update: None,
            on_neighbor_update: None,
            on_place: None,
            on_destroy: None,
            on_interact: None,
            on_collision: None,
            can_place: None,
            can_interact: None,
            bounding_box: None,
            pathfinding_state: None
        }
    }

    pub fn on_random_update<F>(mut self, function: F) -> BlockBuilder where F: Fn(&mut Block, bool) + Send + Sync + 'static {
        self.on_random_update = Some(Box::new(function));
        self
    }
    pub fn on_neighbor_update<F>(mut self, function: F) -> BlockBuilder where F: Fn(&mut Block, bool) + Send + Sync + 'static {
        self.on_neighbor_update = Some(Box::new(function));
        self
    }
    pub fn on_place<F>(mut self, function: F) -> BlockBuilder where F: Fn(&mut Block) + Send + Sync + 'static {
        self.on_place = Some(Box::new(function));
        self
    }
    pub fn on_destroy<F>(mut self, function: F) -> BlockBuilder where F: Fn(&mut Block) + Send + Sync + 'static {
        self.on_destroy = Some(Box::new(function));
        self
    }
    pub fn on_interact<F>(mut self, function: F) -> BlockBuilder where F: Fn(&mut Block) + Send + Sync + 'static {
        self.on_interact = Some(Box::new(function));
        self
    }
    pub fn on_collision<F>(mut self, function: F) -> BlockBuilder where F: Fn(&mut Block) + Send + Sync + 'static {
        self.on_collision = Some(Box::new(function));
        self
    }
    pub fn can_place<F>(mut self, function: F) -> BlockBuilder where F: Fn(&Block) -> bool + Send + Sync + 'static {
        self.can_place = Some(Box::new(function));
        self
    }
    pub fn can_interact<F>(mut self, function: F) -> BlockBuilder where F: Fn(&Block) -> bool + Send + Sync + 'static {
        self.can_interact = Some(Box::new(function));
        self
    }
    pub fn bounding_box(mut self, bounding_box: BoundingBox) -> BlockBuilder {
        self.bounding_box = Some(bounding_box);
        self
    }
    pub fn pathfinding_state(mut self, pathfinding_state: PathfindingState) -> BlockBuilder {
        self.pathfinding_state = Some(pathfinding_state);
        self
    }

    pub fn build(self) -> Block {
        Block {
            namespace: self.namespace,
            id: self.id,

            on_random_update: self.on_random_update.unwrap_or(Box::new(|_, _| {})),
            on_neighbor_update: self.on_neighbor_update.unwrap_or(Box::new(|_, _| {})),
            on_place: self.on_place.unwrap_or(Box::new(|_| {})),
            on_destroy: self.on_destroy.unwrap_or(Box::new(|_| {})),
            on_interact: self.on_interact.unwrap_or(Box::new(|_| {})),
            on_collision: self.on_collision.unwrap_or(Box::new(|_| {})),
            can_place: self.can_place.unwrap_or(Box::new(|_| false)),
            can_interact: self.can_interact.unwrap_or(Box::new(|_| false)),

            bounding_box: self.bounding_box.unwrap_or(BoundingBox(Vector3::zero(), Vector3::zero())),
            pathfinding_state: self.pathfinding_state.unwrap_or(PathfindingState::Solid),
        }
    }
}