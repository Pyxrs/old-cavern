use shared::Module;

pub mod chunk;

struct Mesher {
    //
}

impl Module<(), (), ()> for Mesher {
    #[profiling::function]
    fn new(_: ()) -> ((), Self) {
        todo!()
    }

    #[profiling::function]
    fn run(self, _args: ()) {
        todo!()
    }
}