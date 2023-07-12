use std::hash::Hash;

use self::{item::Item, block::Block};

pub mod block;
pub mod item;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Id<T: Eq + Hash + PartialEq> {
    pub namespace: String,
    pub id: u32,
    pub r#type: Type<T>,
}

impl<T: Eq + Hash + PartialEq> Id<T> {
    pub fn new(r#type: Type<T>, namespace: String, id: u32) -> Self {
        Self {
            namespace,
            id,
            r#type,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Type<CustomType> {
    Builtin(BuiltinType),
    Custom(CustomType),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BuiltinType {
    Block,
    Item,
}

pub enum Data<B: Block, I: Item, CustomData> {
    Builtin(BuiltinData<B, I>),
    Custom(CustomData),
}

pub enum BuiltinData<B: Block, I: Item> {
    Block(B),
    Item(I),
}