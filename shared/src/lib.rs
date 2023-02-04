use std::sync::{Arc, RwLock};

pub mod extra {
    pub use log::*;
    pub use cgmath::*;
    pub use crossbeam_channel::*;
    pub use anyhow::*;
}

pub mod registry;
pub mod types;
pub mod direction;
pub mod util;
pub mod packets;
pub mod resources;

pub type Module<T> = Arc<RwLock<T>>;

pub trait InnerModule<M> {
    fn run(module: Module<Self>, other_modules: M) {}

    fn to_module(self) -> Module<Self> where Self: Sized {
        Arc::new(RwLock::new(self))
    }
}