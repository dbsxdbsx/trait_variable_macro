extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Ident, ItemStruct};

#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse attributes
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let (trait_path, trait_name) = match args.first().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => (path, path.get_ident().unwrap()),
        _ => panic!("Expected a trait name"),
    };
    // TODO: seems no need?
    // parse hidden_trait_path
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
        trait_variable! {
            (#trait_name)
            (#hidden_trait_path)
            #visible struct #struct_name {
                #(#struct_fields),*
            }
        }
    };

    // return
    expanded.into()
}

#[proc_macro_attribute]
pub fn test_macro_output(args: TokenStream, _input: TokenStream) -> TokenStream {
    // 解析属性
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let (trait_path, _trait_name) = match args.first().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => (path, path.get_ident().unwrap()),
        _ => panic!("Expected a trait name"),
    };
    // 解析 hidden_trait_path
    let mut hidden_trait_path = trait_path.clone();
    if let Some(last_segment) = hidden_trait_path.segments.last_mut() {
        let ident = Ident::new(
            &format!("_{}", last_segment.ident),
            last_segment.ident.span(),
        );
        last_segment.ident = ident;
    }

    // 扩展代码
    let expanded = quote! {
    use #trait_path;
    use crate::#hidden_trait_path; };
    // 返回
    expanded.into()
}

// TODO: delete?
// use proc_macro::Span;

// #[proc_macro_attribute]
// pub fn test_macro_output2(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     // 获取当前Span的源文件信息
//     let source_file = Span::call_site().source_file();

//     // 获取源文件的路径
//     let path = source_file.path();

//     // 将路径转换为字符串
//     let path_string = path.to_string_lossy();

//     // 打印出路径
//     println!("The source file path is: {}", path_string);

//     // 返回原始的TokenStream
//     item
// }
