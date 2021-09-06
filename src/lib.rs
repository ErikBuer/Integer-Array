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

// Modules (files in the top hierarchy)
pub mod utility_functions;
pub mod constants;
pub mod trait_definitions;
pub mod functions;
//#[macro_export]
pub mod real;
//#[macro_export]
pub mod complex;