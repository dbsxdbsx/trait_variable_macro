extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, AttributeArgs, DeriveInput, ItemStruct,
    ItemTrait, Lit, Meta, MetaNameValue, NestedMeta,
};

#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute input tokens into a MetaNameValue
    let args = parse_macro_input!(args as Meta);
    let trait_name = match args {
        Meta::List(meta_list) => {
            // Expecting only one nested meta inside the list
            if meta_list.nested.len() != 1 {
                panic!("Expected exactly one trait name as argument");
            }
            match &meta_list.nested[0] {
                syn::NestedMeta::Lit(Lit::Str(lit_str)) => lit_str.value(),
                _ => panic!("Expected a string literal for the trait name"),
            }
        }
        _ => panic!("Expected a list of arguments"),
    };

    // Parse the input TokenStream into an ItemStruct
    let input_struct = parse_macro_input!(input as ItemStruct);

    // Generate the expanded code
    let expanded = quote! {
        trait_variable! {
            (#trait_name)
            struct #input_struct
        }
    };

    // Return the generated code
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
