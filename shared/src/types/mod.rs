#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Id {
    pub namespace: String,
    pub id: u32,
    pub r#type: Type,
}

impl Id {
    pub fn new(r#type: Type, namespace: String, id: u32) -> Self {
        Self {
            namespace,
            id,
            r#type,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Block,
    Item,
}