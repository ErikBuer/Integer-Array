# Numeric-Array
No-STD numeric fixed-size array for Rust.

This is an experimental library for no-std DSP.
The library only supports fixed-sized arrays of i32.
This is because the intention of the code is to experiment with ways of expressing DSP in Rust.

Note that this project is not hosted on crates.io.

See the main workings in /src/real/array.rs.

## Use example
arrays types of fixed size is defined with traits through a macro as follows:
```rust
use numeric_array;
use numeric_array::trait_definitions::*;

numeric_array::declare_array_real!( Vec11, 11);
```
### Element-wise arithmetic
Math can then be performed on the array types.
```rust
let mut x = Vec11::new(66);
let y     = Vec11::new(2);
x = x/y;
assert_eq!(x.front(), 33);
```
### array-scalar operations
There is also some support for array-scalar operations.
```rust
declare_array_real!( Vec4, 8);
let mut x = Vec4::ramp(0,22);
x = x+3;
assert_eq!{x[1], 25i32 };
```
### Esimator utilities
```rust
declare_array_real!( Vec32, 32);
let x = Vec32::ramp(100,20);
assert_eq!{x.argmax(), 31};
```