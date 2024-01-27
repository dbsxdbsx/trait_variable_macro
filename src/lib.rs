extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, AttributeArgs, DeriveInput, Meta,
    NestedMeta,
};
use syn::{ItemStruct, ItemTrait};

#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析属性参数
    let args = parse_macro_input!(args as AttributeArgs);
    let trait_name = match &args[0] {
        NestedMeta::Meta(Meta::Path(path)) => path.get_ident().unwrap().to_string(),
        _ => panic!("Expected a trait name"),
    };

    // 解析输入结构体
    let input_struct = parse_macro_input!(input as ItemStruct);

    // 生成新的代码
    let expanded = quote! {
        trait_variable! {
            (#trait_name)
            struct #input_struct
        }
    };

    // 返回生成的代码
    expanded.into()
}

// for tag trait
// #[proc_macro_attribute]
// pub fn trait_var(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as ItemTrait);
//     let ident = input.ident;
//     let items = input.items;
//     let expanded = quote! {
//         trait_variable! trait #ident {
//             #(#items)*
//         }
//     };
//     TokenStream::from(expanded)
// }
