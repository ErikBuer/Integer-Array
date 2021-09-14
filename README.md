# Integer-Array

No-STD fixed-point fixed-size array for Rust.

The library lets the user declare arrays with traits that simplify the writhing of DSP-code.

It utilizes the [fixed](https://crates.io/crates/fixed) library to allow flexibility in fixed point sizes and precisions.

This is an experimental library for no-std DSP. Expect breaking changes.

The package can be found at https://crates.io/crates/integer_array.

The documentation can be found at https://docs.rs/integer_array.

Release notes are found under RELEASES.md

## Use example

```rust
use integer_array as ia;
use integer_array::trait_definitions::*;
use fixed::{types::extra::U20, FixedI32};
 
// Define an array type of size 4, and implemnets a buch of traits to it.
ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
 
// Create the actual array.
let mut x = Arr4::new_from_i32(66);
assert_eq!(x.to_i32(), [66, 66, 66, 66]);

// Do some math with the arrays.
let y     = Arr4::new_from_f32( 2.0 );
x = x/y;
 
assert_eq!(x.front(), 33);
```
