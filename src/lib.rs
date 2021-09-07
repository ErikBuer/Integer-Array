#![crate_name = "numeric_array"]
#![no_std]

/*
// Use std for test.
#[cfg(any(feature = "std", test))]
extern crate std;
*/


// Pull in core as std.
#[cfg(all(not(feature = "std"), not(test)))]
extern crate core as std;

// Modules (files in the top hierarchy)
pub mod utility_functions;
pub mod constants;
pub mod trait_definitions;
pub mod real;
pub mod complex;