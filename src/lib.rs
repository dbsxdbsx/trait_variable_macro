extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use regex::{Captures, Regex};
use syn::{braced, token};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token, TraitItem, Type,
};

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
    trait_name: Ident,
    _brace_token: token::Brace,
    trait_variables: Punctuated<TraitVarField, Token![;]>,
    trait_items: Vec<TraitItem>,
}

impl Parse for TraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(TraitInput {
            _trait_token: input.parse()?,
            trait_name: input.parse()?,
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

/// a functional macro used to generate code for a trait with variable fields
#[proc_macro]
pub fn trait_variable(input: TokenStream) -> TokenStream {
    let TraitInput {
        trait_name,
        trait_variables,
        trait_items,
        ..
    } = parse_macro_input!(input as TraitInput);
    // 1.1 get parent trait name
    let parent_trait_name = Ident::new(&format!("_{}", trait_name), trait_name.span());
    // 1.2 get trait declarative macro name
    let trait_decl_macro_name =
        Ident::new(&format!("{}_for_struct", trait_name), trait_name.span());

    // 2.1 generate parent trait methods declaration
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
    // 2.2 generate trait variable fields definition for structs later
    let struct_trait_fields_defs =
        trait_variables
            .iter()
            .map(|TraitVarField { var_name, ty, .. }| {
                quote! {
                    #var_name: #ty,
                }
            });
    // 2.3 generate parent trait methods implementation for struct
    let parent_trait_methods_impls =
        trait_variables
            .iter()
            .map(|TraitVarField { var_name, ty, .. }| {
                let method_name = Ident::new(&format!("_{}", var_name), var_name.span());
                let method_name_mut = Ident::new(&format!("_{}_mut", var_name), var_name.span());
                quote! {
                    fn #method_name(&self) -> &#ty{
                        &self.#var_name
                    }
                    fn #method_name_mut(&mut self) -> &mut #ty{
                        &mut self.#var_name
                    }
                }
            });

    // 3. generate methods for the original trait
    let original_trait_items = trait_items.into_iter().map(|item| {
        if let TraitItem::Method(mut method) = item {
            if let Some(body) = &mut method.default {
                // Use regular expressions or other methods to find and replace text
                let re = Regex::new(r"self\.([a-zA-Z_]\w*)").unwrap();
                let body_str = quote!(#body).to_string();
                let new_body_str = re
                    .replace_all(&body_str, |caps: &Captures| {
                        let name = &caps[1];
                        // Check if it is followed by braces
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

    // 4. generate the hidden declarative macro for target struct
    let decl_macro_code = quote! {
        #[doc(hidden)]
        #[macro_export] // TODO: <-- Only if the trait's visibility is `pub`
        macro_rules! #trait_decl_macro_name { // NOTE: the reexpanded macro is used for rust struct only
            (
                ($hidden_parent_trait:path)
                $(#[$struct_attr:meta])* // NOTE: make sure the style is consistent with that in arm 2 output
                $vis:vis struct $struct_name:ident {
                    $($struct_content:tt)*
                }
            ) => {
                $(#[$struct_attr])*
                $vis struct $struct_name {
                    $($struct_content)*
                    // NOTE: the following part is from root macro:
                    // $(
                    //     // $(#[$field_attr])* // TODO:
                    //     // $field_vis  // TODO:
                    //     $trait_field_name: $field_type,// TODO:
                    // )*
                    #(
                        #struct_trait_fields_defs
                    )*
                }
                impl $hidden_parent_trait for $struct_name {
                //     $( // TODO:
                //         fn [< _$trait_field_name >](&self) -> &$field_type {
                //             &self.$trait_field_name
                //         }
                //         fn [< _$trait_field_name _mut>](&mut self) -> &mut $field_type {
                //             &mut self.$trait_field_name
                //         }
                //     )*
                    #(
                        #parent_trait_methods_impls
                    )*
                }
            };
        }
    };
    // 5. expand code
    let expanded = quote! {
        trait #parent_trait_name {
            #(#parent_trait_methods)*
        }
        trait #trait_name: #parent_trait_name {
            #(#original_trait_items)*
        }
        #decl_macro_code
    };
    TokenStream::from(expanded)
}

/// This attribute macro is used to tag Rust struct like: `#[trait_var(<trait_name>)]`
#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse attributes
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let (trait_path, trait_name) = match args.first().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => (path, path.get_ident().unwrap()),
        _ => panic!("Expected a trait name"),
    };

    // TODO: delete? seems no need?
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
    let trait_macro_name = Ident::new(&format!("{}_for_struct", trait_name), trait_name.span());
    let parent_trait_name = Ident::new(&format!("_{}", trait_name), trait_name.span());
    let expanded = quote! {
        #trait_macro_name! {// TODO: not hard code
            (#parent_trait_name)
            // (#hidden_trait_path) // TODO: delete?
            #visible struct #struct_name {
                #(#struct_fields),*
            }
        }
    };

    // return
    expanded.into()
}
