/*use std::fmt::Debug;

pub struct Item {
    pub namespace: String,
    pub id: u32,

    pub on_use: Box<dyn 
fn(&mut Self, Target, Hand) + Send + Sync>,
    pub on_update: Box<dyn 
fn(&mut Self, Location) + Send + Sync>,

    pub description: String,
}

impl Debug for Item {
    
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Item").field("namespace", &self.namespace).field("id", &self.id).finish()
    }
}

pub enum Target {
    Air,
    Block,
    Entity,
}

pub enum Location {
    Inventory,
    Hotbar,
    Hand(Hand),
}

pub enum Hand {
    MainHand,
    OffHand
}

pub struct ItemBuilder {
    namespace: String,
    id: u32,

    on_use: Option<Box<dyn 
fn(&mut Item, Target, Hand) + Send + Sync + 'static>>,
    on_update: Option<Box<dyn 
fn(&mut Item, Location) + Send + Sync + 'static>>,

    description: Option<String>,
}

impl ItemBuilder {
    pub 
fn new(namespace: impl Into<String>, id: u32) -> ItemBuilder {
        Self {
            namespace: namespace.into(),
            id,
            on_use: None,
            on_update: None,
            description: None,
        }
    }

    pub 
fn on_use<F>(mut self, function: F) -> Self where F: 
fn(&mut Item, Target, Hand) + Send + Sync + 'static {
        self.on_use = Some(Box::new(function));
        self
    }
    
    pub 
fn on_update<F>(mut self, function: F) -> Self where F: 
fn(&mut Item, Location) + Send + Sync + 'static {
        self.on_update = Some(Box::new(function));
        self
    }

    pub 
fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    pub 
fn build(self) -> Item {
        Item {
            namespace: self.namespace,
            id: self.id,

            on_use: self.on_use.unwrap_or(Box::new(|_, _, _| {})),
            on_update: self.on_update.unwrap_or(Box::new(|_, _| {})),

            description: self.description.unwrap_or_default(),
        }
    }
}*/