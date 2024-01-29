extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse attributes
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let (trait_path, trait_name) = match args.first().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => (path, path.get_ident().unwrap()),
        _ => panic!("Expected a trait name"),
    };

    // parse input, only accept `struct`
    let input_struct = parse_macro_input!(input as ItemStruct);
    let visible = &input_struct.vis;
    let struct_name = &input_struct.ident;
    let struct_fields = input_struct.fields.iter().map(ToTokens::to_token_stream);

    // expand code
    let expanded = quote! {
        use #trait_path;
        trait_variable! {
            (#trait_name)
            #visible struct #struct_name {
                #(#struct_fields),*
            }
        }
    };

    // return
    expanded.into()
}
