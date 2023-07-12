use shared::types::block;

#[macro_export]
macro_rules! declare_block {
    ($struct_name:ident, { $($body:tt)* }) => {
        struct $struct_name {}

        impl shared::types::block::Block for $struct_name {
            $($body)*
        }

        impl Block for $struct_name {
            $($body)*
        }
    };
}

// Example usage

pub trait Block: block::Block {
}

/*declare_block!(MyStruct, {
});

fn main() {
    let my_struct = MyStruct {
    };

    println!("Foo: {}", my_struct.foo());
    println!("Bar: {}", my_struct.bar());
}*/