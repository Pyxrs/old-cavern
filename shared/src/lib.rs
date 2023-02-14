use std::sync::{Arc, RwLock};

pub mod extra {
    pub use log::*;
    pub use cgmath::*;
    pub use crossbeam_channel::*;
    pub use anyhow::*;
}

pub mod types;
pub mod addons;
pub mod direction;
pub mod util;
pub mod packets;
pub mod resources;

pub type Module<T> = Arc<RwLock<T>>;

pub trait InnerModule<M> {
    fn run(_module: Module<Self>, _other_modules: M) {}

    fn to_module(self) -> Module<Self> where Self: Sized {
        Arc::new(RwLock::new(self))
    }
}