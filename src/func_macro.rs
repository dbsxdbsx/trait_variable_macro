extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::token::Semi;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, ItemTrait, Token, TraitItem, Type, Visibility,
};

// 自定义解析器，用于解析形如 `<vis> <var_name>:<ty_name>;` 的模式
pub struct Field {
    vis: Visibility,
    var_name: Ident,
    colon_token: Token![:],
    ty: Type,
    semi_token: Token![;],
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Field {
            vis: input.parse()?,
            var_name: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}
