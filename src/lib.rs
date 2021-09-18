//! # Integer-Array
//! No-STD fixed-point fixed-size array for Rust.
//!
//! The library lets the user declare arrays with traits that simplify the writhing of DSP-code.
//! 
//! It utilizes the [fixed](https://crates.io/crates/fixed) library to allow flexibility in fixed point sizes and precisions.
//! 
//! This is an experimental library for no-std DSP. Expect breaking changes.
//! 
//! ## Use example
//! ```rust
//! use integer_array as ia;
//! use integer_array::trait_definitions::*;
//! use fixed::{types::extra::U20, FixedI32};
//! 
//! // Define an array type of size 4, and implemnets a buch of traits to it.
//! ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
//! 
//! // Create the actual array.
//! let mut x = Arr4::new_from_i32(66);
//! assert_eq!(x.as_array_i32(), [66, 66, 66, 66]);
//! 
//! // Do some math with the arrays.
//! let y     = Arr4::new_from_f32( 2.0 );
//! x = x/y;
//! 
//! assert_eq!(x.front(), 33);
//! ``` 
//!
//! ## How to use
//! See the macro documentation for the implemented traits.

#![crate_name = "integer_array"]
#![no_std]

// Use std for test.
#[cfg(all(not(feature = "std"), test))]
extern crate std;

// Pull in core as std.
/*
#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;
*/

// Modules (files in the top hierarchy)
pub mod utility;
pub mod trait_definitions;
pub mod real;
pub mod complex;
pub mod filter;