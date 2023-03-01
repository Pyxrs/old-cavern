use shared::Module;

struct Sound {
    //
}

impl Module<(), (), ()> for Sound {
    #[profiling::function]
    fn new(_: ()) -> ((), Self) {
        todo!()
    }

    #[profiling::function]
    fn run(self, _args: ()) {
        todo!()
    }
}