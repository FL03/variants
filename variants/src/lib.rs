/*
    Appellation: variants <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # variants
//!
//! Useful macros for constructing enum variants
#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "variants"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
#[cfg(feature = "derive")]
pub use variants_derive::*;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use variants_derive::*;
}
