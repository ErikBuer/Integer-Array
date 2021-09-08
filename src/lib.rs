//! # Numeric-Array
//! No-STD numeric fixed-size array for Rust.
//!
//! The library lets the user declare arrays with traits that simplify the writhing of DSP-code.
//! 
//! ## Examle
//! ```rust
//! use numeric_array as na;
//! use numeric_array::trait_definitions::*;
//! 
//! // Define an array type of size 4, and implemnets a buch of traits to it.
//! na::declare_array_real!( Arr4, 4);
//! 
//! // Create the actual array.
//! let mut x = Arr4::new(66);
//! assert_eq!(x.data, [66, 66, 66, 66]);
//! 
//! // Do some math with the arrays.
//! let y     = Arr4::new(2);
//! x = x/y;
//! assert_eq!(x.front(), 33);
//! ```
//! 
//! ## Backgorund
//! This is an experimental library for no-std DSP.
//! The library only supports fixed-sized arrays of i32.
//! 
//! 32-bit arrays can be handled by most DSP-capable embedded devices and provides 6.02×32>192 dB of dynamic range, which is sufficient for most DSP use.
//! The rationale is that if 64-bit processing is available, then so is an OS, and the Rust standard library. 


#![crate_name = "numeric_array"]
#![no_std]

/*
// Use std for test.
#[cfg(any(feature = "std", test))]
extern crate std;
*/

// Pull in core as std.
//#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

// Modules (files in the top hierarchy)
pub mod utility_functions;
pub mod constants;
pub mod trait_definitions;
pub mod real;
pub mod complex;