#![crate_name = "numeric_vector"]
#![no_std]

/*
// Use std for test.
#[cfg(any(feature = "std", test))]
extern crate std;
*/


// Pull in core as std.
#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

// Include the file vector.rs

pub mod utility_functions;
pub mod constants;
pub mod trait_definitions;

pub mod vector;
