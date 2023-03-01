use shared::Module;

pub mod quad;

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