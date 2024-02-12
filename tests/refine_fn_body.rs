#[cfg(test)]
mod test {
    use trait_variable_macros::refine_trait_fn_body;

    trait _MyTrait {
        fn _my_var(&self) -> &i32;
        fn _my_var_mut(&mut self) -> &mut i32;

        fn _my_var2(&self) -> &u64;
        fn _my_var2_mut(&mut self) -> &mut u64;
    }
    // 只能先用函数宏将所有trait的东西展开，并附带一个包含field的声明宏内容（用以struct部分调用）
    refine_trait_fn_body! {
        trait MyTrait: _MyTrait {
            fn test1(&mut self);
            fn test2(&self) {
                println!("I am test2");
            }

            fn test3(&mut self) {
                    self.test2();
                    let mut x = self.my_var; // NOTE: don't forget the `*`
                    x += 1;
                    // self.my_var = 1; // TODO: not ok yet
                    // (*self._my_var_mut()) = 1; // ok
                    println!("I am test3:{},{}", self.my_var, self.my_var2);
                }
        }
        // ... more functions can be added
    }

    struct MyStruct {
        field_1: i32,
        field_2: u64,
    }

    impl _MyTrait for MyStruct {
        fn _my_var(&self) -> &i32 {
            &5
        }
        fn _my_var2(&self) -> &u64 {
            &2
        }

        fn _my_var_mut(&mut self) -> &mut i32 {
            &mut self.field_1
        }

        fn _my_var2_mut(&mut self) -> &mut u64 {
            &mut self.field_2
        }
    }

    impl MyTrait for MyStruct {
        fn test1(&mut self) {
            // Change the method signature to take a mutable reference to self
            println!("test");
        }
        fn test2(&self) {
            println!("I am test22");
        }
    }

    // Declarative macro to expand into `let x = 0;`
    // #[macro_export]
    // macro_rules! test_dec {
    //     ($x:ident) => {
    //         let mut $x = 0;
    //     };
    // }

    #[test]
    fn test() {
        test_dec!(x);
        x = 1;
        println!("{x}");
        let mut s = MyStruct {
            field_1: 1,
            field_2: 2,
        };
        s.test1();
        s.test2();
        s.test3();
        s._my_var();
        println!("{}", s._my_var());
        println!("{}", s._my_var_mut());
        // println!("{}", my_var2);
    }
}
