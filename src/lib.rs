extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, AttributeArgs, DeriveInput, ItemStruct,
    ItemTrait, Lit, Meta, MetaNameValue, NestedMeta,
};

#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析属性输入
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let trait_name = match args.first().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap().to_string(),
        _ => panic!("Expected a trait name"),
    };

    // 解析输入结构体
    let input_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &input_struct.ident;
    let struct_fields = input_struct.fields.iter().map(ToTokens::to_token_stream);

    // 生成新的代码
    let expanded = quote! {
        trait_variable! {
            (#trait_name)
            struct #struct_name {
                #(#struct_fields),*
            }
        }
    };

    // 返回生成的代码
    expanded.into()
}

// TODO: for tag trait, not ok yet
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
