pub mod prelude {
    pub use log::*;
    pub use cgmath::*;
}

pub trait Module<R, F> {
    /// Create the module's senders and receivers
    /// Return the receivers for other modules to use
    fn new() -> R;

    /// Consumes other modules' receivers
    /// Usually blocks thread
    fn run(connections: F) -> Self;
}