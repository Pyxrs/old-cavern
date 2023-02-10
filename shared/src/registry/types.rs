use crate::types::{block::Block, item::Item};

#[derive(Debug)]
pub enum ServerRegistryType {
    Block(Block),
    Item(Item),
}

#[derive(Debug)]
pub enum ClientRegistryType {
    Block(Block),
    Item(Item),
}

pub trait RegistryType {
    fn get_id(&self) -> (&String, u32);
}

impl RegistryType for ServerRegistryType {
    fn get_id(&self) -> (&String, u32) {
        match self {
            Self::Block(b) => (&b.namespace, b.id),
            Self::Item(i) => (&i.namespace, i.id),
        }
    }
}
impl RegistryType for ClientRegistryType {
    fn get_id(&self) -> (&String, u32) {
        match self {
            Self::Block(b) => (&b.namespace, b.id),
            Self::Item(i) => (&i.namespace, i.id),
        }
    }
}