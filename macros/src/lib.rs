/*
    appellation: variants-macros <library>
    authors: @FL03
*/
//! Procedural macros supporting the `variants` crate

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

