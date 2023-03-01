use shared::Module;

struct Network {
    //
}

impl Module<(), (), ()> for Network {
    #[profiling::function]
    fn new(_: ()) -> ((), Self) {
        todo!()
    }

    #[profiling::function]
    fn run(self, _args: ()) {
        todo!()
    }
}