use std::collections::HashMap;

use crate::{types::{item::Item, block::Block}, InnerModule};

pub struct Registry {
    blocks: HashMap<String, HashMap<u32, Block>>,
    items: HashMap<String, HashMap<u32, Item>>
}

impl InnerModule<()> for Registry {}

impl Registry {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            items: HashMap::new(),
        }
    }

    pub fn register_block(&mut self, block: Block) {
        let namespace = block.namespace.clone();
        let id = block.id;

        let blocks = self.blocks.entry(namespace).or_insert(HashMap::new());
        blocks.insert(id, block);
    }

    pub fn register_item(&mut self, item: Item) {
        let namespace = item.namespace.clone();
        let id = item.id;

        let items = self.items.entry(namespace).or_insert(HashMap::new());
        items.insert(id, item);
    }

    pub fn get_block(&self, namespace: impl Into<String>, id: u32) -> &Block {
        &self.blocks.get(&namespace.into()).expect("Namespace not found!").get(&id).expect("Block not found!")
    }

    pub fn get_item(&self, namespace: impl Into<String>, id: u32) -> &Item {
        &self.items.get(&namespace.into()).expect("Namespace not found!").get(&id).expect("Item not found!")
    }
}