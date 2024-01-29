extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ItemStruct, Path, PathSegment};

#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse attributes
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let (trait_path, trait_name) = match args.first().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => (path, path.get_ident().unwrap()),
        _ => panic!("Expected a trait name"),
    };
    // create hidden_trait_path
    let mut hidden_trait_path = trait_path.clone();
    if let Some(last_segment) = hidden_trait_path.segments.last_mut() {
        let ident = Ident::new(
            &format!("_{}", last_segment.ident),
            last_segment.ident.span(),
        );
        last_segment.ident = ident;
    }
    // parse input, only accept `struct`
    let input_struct = parse_macro_input!(input as ItemStruct);
    let visible = &input_struct.vis;
    let struct_name = &input_struct.ident;
    let struct_fields = input_struct.fields.iter().map(ToTokens::to_token_stream);

    // expand code
    let expanded = quote! {
        use #hidden_trait_path;
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
