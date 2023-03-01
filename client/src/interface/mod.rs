use shared::Module;

struct Interface {
    //
}

impl Module<(), (), ()> for Interface {
    #[profiling::function]
    fn new(_: ()) -> ((), Self) {
        todo!()
    }

    #[profiling::function]
    fn run(self, _args: ()) {
        todo!()
    }
}