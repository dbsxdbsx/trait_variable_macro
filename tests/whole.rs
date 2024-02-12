#[cfg(test)]
mod test {
    use trait_variable::trait_variable;
    use trait_variable_macros::trait_var;

    //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
    trait_variable! {
        // the lint is also activated inside the macro, using rust_anaylzer for example
        trait MyTrait {  // feel free to add `pub` when needed
            // 1.put the variable fields definition at the top of the target trait before any function
            pub x: i32; // TODO: can't be without variable at present
            pub y: bool;
            pub z : f32;

            // 2.the order of the function definition doesn't matter
            // fn print_x(&self); // ok
            fn print_x(&self) {
                // println!("(original)x: `{}`", self.x);// TODO: make self.<> valid
                println!("(original)x: `{}`", self._x());
            }
            fn print_y(&self);
            fn print_z(&self);

            // fn change_and_print_z(&mut self, new_num: f32) {
            //     *self.get_fields_mut().z = new_num;
            //     println!("z: `{}`", self.get_fields().z);
            // }
        }
    }
    //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

    //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
    #[trait_var(MyTrait)]
    struct MyStruct {
        a: i32,
    }
    // trait_variable! {
    //     (MyTrait) // put this at the top of the struct
    //     struct MyStruct { // feel free to add `pub` when needed
    //     // feel free to add any fields as usual or leave it empty
    //      a: i32,
    //     }
    // }
    //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

    //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓impl block↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
    impl MyStruct {
        fn print_a(&self) {
            println!("a: `{}`", self.a);
        }
    }
    impl MyTrait for MyStruct {
        // fn print_x(&self) {
        //     println!("x: `{}`", self.x);
        // }
        fn print_y(&self) {
            println!("y: `{}`", self.y);
        }
        fn print_z(&self) {
            println!("z: `{}`", self.z);
        }
    }
    //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑impl block↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

    // #[test_macro_output(MyTrait2)]
    struct MyStruct2;

    // NOTE: when using Rust-analyzer, the following code will cause the cyclic reference error by hitting` Run Test|Debug`
    // please use `cargo test` in terminal instead
    #[test]
    fn test() {
        let s = MyStruct {
            a: 2,
            x: 3,
            y: true,
            z: 1.,
        };
        s.print_a();
        s.print_x();
        s.print_y();
        // s.change_and_print_z(3.14);
    }
}
