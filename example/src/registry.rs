use std::sync::RwLockWriteGuard;

use shared::{registry::Registry, types::block::BlockBuilder};

pub fn register_features(mut registry: RwLockWriteGuard<Registry>) {
    [
        block(0).build(),
        block(1).build(),
    ].into_iter().for_each(|block| { registry.register_block(block); });
}

fn block(id: u32) -> BlockBuilder {
    BlockBuilder::new("example", id)
}