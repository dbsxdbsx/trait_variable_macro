#[cfg(test)]
mod tests {
    use trait_variable_macros::trait_var;
    macro_rules! trait_variable {
        (pub struct $struct_name:ident {
            $($body:tt)*
        }) => {
            trait_variable!(@impl pub struct $struct_name { $($body)* });
        };
        (struct $struct_name:ident {
            $($body:tt)*
        }) => {
            trait_variable!(@impl struct $struct_name { $($body)* });
        };
        //
        (@impl $vis:vis struct $struct_name:ident {
            $($user_field_vis:vis $user_field_name:ident : $user_field_type:ty),*
            $(,)?
        }) => {
            paste::paste! {
                $vis struct $struct_name {
                    // original_field
                    $($user_field_vis $user_field_name : $user_field_type,)*
                }

                impl [<$struct_name>] {
                    pub fn print(&self) {
                        println!("Hello, xxxworld!");
                    }
                }
            }
        }
    }

    #[trait_var]
    struct StructName {
        pub prop: i32,
    }

    #[test]
    fn test_procedure_macro_syntax() {
        let s = StructName { prop: 4 };
        s.print();
    }
}
