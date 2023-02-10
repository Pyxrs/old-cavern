use std::sync::RwLockWriteGuard;

use client::ClientRegistry;
use server::ServerRegistry;
use shared::{registry::types::{ClientRegistryType, ServerRegistryType}, types::block::BlockBuilder};

pub fn register_client_features(mut registry: RwLockWriteGuard<ClientRegistry>) {
    [
        block(0).build(),
        block(1).build(),
        block(2).build(),
    ].into_iter().for_each(|block| registry.register(ClientRegistryType::Block(block)));
}

pub fn register_server_features(mut registry: RwLockWriteGuard<ServerRegistry>) {
    [
        block(0).build(),
        block(1).build(),
        block(2).build(),
    ].into_iter().for_each(|block| registry.register(ServerRegistryType::Block(block)));
}

fn block(id: u32) -> BlockBuilder {
    BlockBuilder::new("example", id)
}