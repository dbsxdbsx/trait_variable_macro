use proc_macro::TokenStream;
use quote::quote;
use regex::{Captures, Regex};
use syn::{braced, token, ItemTrait};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token, TraitItem, Type,
};

extern crate proc_macro;

#[proc_macro]
pub fn refine_trait_fn_body(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    // Extract trait_ident and parent_trait_ident from input
    let trait_ident = &input.ident;
    let parent_trait_ident = input.supertraits.first().expect("Expected a parent trait");

    let refined_trait_fns = input.items.into_iter().map(|item| {
        if let TraitItem::Method(mut method) = item {
            if let Some(body) = &mut method.default {
                // Use regular expressions or other methods to find and replace text
                let re = Regex::new(r"self\.([a-zA-Z_]\w*)").unwrap();
                let body_str = quote!(#body).to_string();
                let new_body_str = re
                    .replace_all(&body_str, |caps: &Captures| {
                        let name = &caps[1];
                        // Check if it is followed by parentheses
                        if body_str.contains(&format!("{}(", name)) {
                            format!("self.{}", name)
                        } else {
                            format!("(*self._{}())", name) // TODO：what about the `mut` version?
                        }
                    })
                    .to_string();

                let new_body: TokenStream = new_body_str.parse().expect("Failed to parse new body");
                method.default = Some(syn::parse(new_body).expect("Failed to parse method body"));
            }
            quote!(#method)
        } else {
            quote!(#item)
        }
    });

    // expand code
    let expanded = quote! {
        trait #trait_ident: #parent_trait_ident {
            #(#refined_trait_fns)*
        }
    };
    TokenStream::from(expanded)
}

// ------------------1st functional macro：ok--------------------------
struct TraitVarField {
    var_name: Ident,
    _colon_token: Token![:],
    ty: Type,
}
impl Parse for TraitVarField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TraitVarField {
            var_name: input.parse()?,
            _colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

struct TraitInput {
    _trait_token: Token![trait],
    trait_ident: Ident,
    _brace_token: token::Brace,
    trait_variables: Punctuated<TraitVarField, Token![;]>,
    trait_items: Vec<TraitItem>,
}

impl Parse for TraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(TraitInput {
            _trait_token: input.parse()?,
            trait_ident: input.parse()?,
            _brace_token: braced!(content in input),
            // Parse all variable declarations until a method or end of input is encountered
            trait_variables: {
                let mut vars = Punctuated::new();
                while !content.peek(Token![fn]) && !content.peek(Token![;]) && !content.is_empty() {
                    vars.push_value(content.parse()?);
                    // Ensure that a semicolon follows the variable declaration
                    if !content.peek(Token![;]) {
                        return Err(content.error("expected `;` after variable declaration"));
                    }
                    vars.push_punct(content.parse()?);
                }
                vars
            },
            // Parse all method declarations
            // trait_methods: content.parse_terminated(TraitItem::parse)?,
            trait_items: {
                let mut items = Vec::new();
                while !content.is_empty() {
                    items.push(content.parse()?);
                }
                items
            },
        })
    }
}

#[proc_macro]
pub fn test_fn_macro(input: TokenStream) -> TokenStream {
    let TraitInput {
        trait_ident,
        trait_variables,
        trait_items,
        ..
    } = parse_macro_input!(input as TraitInput);
    // 1.get (parent) trait name
    let parent_trait_ident = Ident::new(&format!("_{}", trait_ident), trait_ident.span());
    // 2. generate methods for parent trait
    let parent_trait_methods =
        trait_variables
            .iter()
            .map(|TraitVarField { var_name, ty, .. }| {
                let method_name = Ident::new(&format!("_{}", var_name), var_name.span());
                let method_name_mut = Ident::new(&format!("_{}_mut", var_name), var_name.span());
                quote! {
                    fn #method_name(&self) -> &#ty;
                    fn #method_name_mut(&mut self) -> &mut #ty;
                }
            });
    // 3. generate methods for the original trait
    // let original_trait_items = trait_items.into_iter().map(|item| quote! { #item });
    let original_trait_items = trait_items.into_iter().map(|item| {
        if let TraitItem::Method(mut method) = item {
            if let Some(body) = &mut method.default {
                // 使用正则表达式或其他方法来查找和替换文本
                let re = Regex::new(r"self\.([a-zA-Z_]\w*)").unwrap();
                let body_str = quote!(#body).to_string();
                let new_body_str = re
                    .replace_all(&body_str, |caps: &Captures| {
                        let name = &caps[1];
                        // 检查是否跟随括号
                        if body_str.contains(&format!("{}(", name)) {
                            format!("self.{}", name)
                        } else {
                            format!("self._{}()", name)
                        }
                    })
                    .to_string();

                let new_body: TokenStream = new_body_str.parse().expect("Failed to parse new body");
                method.default = Some(syn::parse(new_body).expect("Failed to parse method body"));
            }
            quote! { #method }
        } else {
            quote! { #item }
        }
    });
    // 4. expand code
    let expanded = quote! {
        trait #parent_trait_ident {
            #(#parent_trait_methods)*
        }
        trait #trait_ident: #parent_trait_ident {
            #(#original_trait_items)*
        }
    };
    TokenStream::from(expanded)
}

// OLD: OK
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
    let input_struct = parse_macro_input!(input as syn::ItemStruct);
    let visible = &input_struct.vis;
    let struct_name = &input_struct.ident;
    let struct_fields = input_struct
        .fields
        .iter()
        .map(quote::ToTokens::to_token_stream);

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
