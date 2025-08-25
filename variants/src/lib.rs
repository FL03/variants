/*
    Appellation: variants <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # variants
//!
//! Useful macros for constructing enum variants
#![crate_name = "variants"]
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
#[cfg(feature = "derive")]
pub use variants_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use variants_macros::*;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use variants_derive::*;
}
