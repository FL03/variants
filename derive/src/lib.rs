/*
    Appellation: variants-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This crate provides a derive macro for generating functional constructors for enums.
extern crate proc_macro;
extern crate quote;
extern crate syn;

pub(crate) mod attrs {
    pub mod attr;
}
pub(crate) mod enums;
pub(crate) mod utils;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

/// This macro automatically generates functional constructors for all enclosed variants.
#[proc_macro_derive(VariantConstructors, attributes(variant))]
pub fn variant_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => enums::impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}
