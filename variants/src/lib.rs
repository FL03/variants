/*
    Appellation: variants <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # variants
//!
//! The [`variants`](self) crate works to provide a set of utilities for working with enums and
//! their variants.
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

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "Either feature `std` or `alloc` must be enabled"
}

#[doc(inline)]
pub use self::error::{Error, Result};

pub mod error;

#[doc(inline)]
#[cfg(feature = "derive")]
pub use variants_derive::*;
#[doc(inline)]
#[allow(unused_imports)]
#[cfg(feature = "macros")]
pub use variants_macros::*;

pub mod prelude {
    #[cfg(feature = "derive")]
    pub use variants_derive::*;
    #[allow(unused_imports)]
    #[cfg(feature = "macros")]
    pub use variants_macros::*;
}
