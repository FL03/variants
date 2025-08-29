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

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "Either feature `std` or `alloc` must be enabled"
}

#[allow(dead_code)]
pub(crate) mod attrs;
pub(crate) mod impls;
pub(crate) mod utils;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput};

/// This macro automatically generates functional constructors for all enclosed variants.
///
/// ## Examples
///
/// ```rust
/// use variants_derive::VariantConstructors;
///
/// #[derive(Clone, Debug, Default, PartialEq, PartialOrd, VariantConstructors)]
/// pub enum MyEnum {
///     Unit,
///     Tuple(u32, f64),
///     Struct { id: u32, name: String },
/// }
/// 
/// assert_eq!(MyEnum::unit(), MyEnum::Unit);
/// assert_eq!(MyEnum::tuple(42, 1.0), MyEnum::Tuple(42, 1.0));
/// assert_eq!(MyEnum::struct_(1, "Test".to_string()), MyEnum::Struct { id: 1, name: "Test".to_string() });
/// ```
#[proc_macro_derive(VariantConstructors, attributes(variants))]
pub fn variant_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => impls::impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}
