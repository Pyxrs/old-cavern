use crossbeam_channel::{Sender, Receiver};

pub mod extra {
    pub use log::*;
    pub use cgmath::*;
    pub use crossbeam_channel::*;
    pub use anyhow::*;
}

pub mod types;
pub mod addons;
pub mod direction;
pub mod packets;
pub mod resources;
pub mod util;

pub trait StaticModule<A> {
    fn new() -> (A, Self);
}

pub trait Module<A, E> {
    fn new() -> (A, Self);
    fn run(self, args: E);
}

pub struct Query<R, P> {
    pub request: Sender<R>,
    pub payload: Receiver<P>,

    local_request: Receiver<R>,
    local_payload: Sender<P>,
}

impl<R, P> Query<R, P> {
    pub fn new(request: (Sender<R>, Receiver<R>), payload: (Sender<P>, Receiver<P>)) -> Self {
        Self {
            request: request.0,
            payload: payload.1,

            local_request: request.1,
            local_payload: payload.0,
        }
    }

    pub fn update<F>(&mut self, send: F) where F: Fn(R) -> P {
        for request in self.local_request.try_iter() {
            let _ = self.local_payload.try_send(send(request));
        }
    }
}

pub trait Ignore {
    fn ignore(self);
}

impl<E> Ignore for Result<(), E> {
    fn ignore(self) {
    }
}