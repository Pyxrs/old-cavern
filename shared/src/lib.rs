use std::sync::mpsc::{Receiver, Sender};

pub use log;
pub mod network {
    pub use uflow::*;
}
pub mod math {
    pub use glam::*;
}

pub mod broadcast;
pub mod direction;
pub mod model;
pub mod packets;
pub mod registry;
pub mod resources;
pub mod types;
pub mod util;

pub trait StaticModule<I, A> {
    fn new(initial: I) -> (A, Self);
}

pub trait Module<I, A, E> {
    fn new(initial: I) -> (A, Self);
    fn run(self, args: E);
}

pub struct Query<R, P> {
    pub request: Sender<R>,
    pub payload: Receiver<P>,

    local_request: Receiver<R>,
    local_payload: Sender<P>,
}

impl<R, P> Query<R, P> {
    #[profiling::function]
    pub fn new(request: (Sender<R>, Receiver<R>), payload: (Sender<P>, Receiver<P>)) -> Self {
        Self {
            request: request.0,
            payload: payload.1,

            local_request: request.1,
            local_payload: payload.0,
        }
    }

    #[profiling::function]
    pub fn update<F>(&mut self, send: F)
    where
        F: Fn(R) -> P,
    {
        for request in self.local_request.try_iter() {
            let _ = self.local_payload.send(send(request));
        }
    }
}

pub trait Ignore {
    fn ignore(self);
}

impl<E> Ignore for Result<(), E> {
    fn ignore(self) {}
}
