#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![warn(
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

//***********************************************
// Re-Exports
//***********************************************

//-----------------------------------------------
// All Errors
//-----------------------------------------------
//mod error;
//pub use error::*;

//-----------------------------------------------
// Blueprint Traits
//-----------------------------------------------

mod slice_bytes;
#[doc(inline)]
pub use slice_bytes::*;

//WIP
//#[cfg(feature = "typed")]
//mod typed;
//#[cfg(feature = "typed")]
//#[doc(inline)]
//pub use typed::*;
