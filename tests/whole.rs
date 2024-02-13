use trait_variable::trait_variable;

//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
trait_variable! {
    trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the top of the target trait before any function
         x: i32; // TODO: visibility modifier is not supported yet
         y: bool;
         z : f32;

        // 2.the order of the function definition doesn't matter
        fn get_print_field_x(&self) -> &i32{
            println!("x: `{}`", self._x());// TODO: make self.<> valid
            self._x()
        }
        fn get_print_field_y(&self) -> &bool;
        fn get_print_field_z(&self) -> &f32;
        fn change_and_print_z(&mut self, _new_num: f32) {
            // self.z = new_num; // TODO
            // println!("z: `{}`",self.z);
        }
    }
}
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
// way1: use the attribute macro to generate the struct (Recommended)
// #[trait_var(MyTrait)]
// struct MyStruct {
//     a: i32,
// }
// way2: use the hidden declarative macro to generate the struct (Not recommended)
MyTrait_for_struct! {
    (_MyTrait) // inputput the hiddent parent trait
    struct MyStruct { // TODO: feel free to add `pub` when needed
     // feel free to add any fields as usual or leave it empty
     a: i32,
    }
}

impl MyStruct {
    fn get_print_field_a(&self) -> &i32 {
        println!("a: `{}`", self.a);
        &self.a
    }
}
impl MyTrait for MyStruct {
    fn get_print_field_y(&self) -> &bool {
        println!("y: `{}`", self.y);
        &self.y
    }
    fn get_print_field_z(&self) -> &f32 {
        println!("z: `{}`", self.z);
        &self.z
    }
}
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

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
    // test struct fields
    let _aa = s.a;
    let _x = s.x;

    // test methods for struct fields
    assert_eq!(s.get_print_field_a(), &2);
    assert_eq!(s.get_print_field_x(), &3);
    assert_eq!(s.get_print_field_y(), &true);
    // s.change_and_print_z(3.14);
    // assert_eq!(s.get_print_field_z(), &3.14);
}
