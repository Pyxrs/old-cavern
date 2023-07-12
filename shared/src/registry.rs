use std::hash::Hash;
use std::collections::HashMap;

use crate::types::{block::Block, item::Item, Data, Id};
use std::fmt::Debug;

pub struct Registry<
    CustomType: Debug + Eq + Hash + PartialEq,
    B: Block,
    I: Item,
    CustomData,
> {
    entries: HashMap<Id<CustomType>, Data<B, I, CustomData>>,
}

impl<T: Debug + Eq + Hash + PartialEq, B: Block, I: Item, D> Registry<T, B, I, D> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: Id<T>, entry: Data<B, I, D>) {
        self.entries.insert(id, entry);
    }

    pub fn register_entries(&mut self, entries: Vec<(Id<T>, Data<B, I, D>)>) {
        for (id, entry) in entries {
            self.entries.insert(id, entry);
        }
    }

    pub fn get(&self, id: &Id<T>) -> &Data<B, I, D> {
        &self
            .entries
            .get(id)
            .expect(&format!("Entry {:?} not found!", id))
    }
}
