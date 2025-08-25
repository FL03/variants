/*
    Appellation: variants-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The derive macros for the `variants` crate.
//! 
//! - [`VariantConstructors`]: generate functional constructors for all variants of an enum
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate proc_macro;
extern crate quote;
extern crate syn;

#[allow(dead_code)]
pub(crate) mod attrs;
pub(crate) mod impls;
pub(crate) mod utils;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

/// This macro automatically generates functional constructors for all enclosed variants.
#[proc_macro_derive(VariantConstructors, attributes(variants))]
pub fn variant_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => impls::impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}
