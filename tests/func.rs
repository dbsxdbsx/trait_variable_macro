#[cfg(test)]
mod test {
    use trait_variable_macros::test_fn_macro;

    test_fn_macro! {
        trait MyTrait{
            my_var: i32;
            my_var2: u64;
            // ... more variables can be added
            fn test1(&self);
            fn test2(&self){
                println!("I am test2");
            }
            fn test3(&self){
                self.test2();
                println!("I am test3:{},{}",self.my_var,self.my_var2);
            }
            // ... more functions can be added
        }
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
        fn test1(&self) {
            println!("test");
        }
        fn test2(&self) {
            println!("I am test22");
        }
    }
    #[test]
    fn test() {
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
