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

### Sine generation
As this is a no-std library a cusom sine function is implemented. It utilizes a fifth order tailor approximation of sin(x). The trait implementation utilizes floating point multiplications, but returns a fixed point numeric array.

The following polynomial is used:
```julia
x-( x^3/6.0 )+( x^5/120.0 )-( x^7/5040.0 )+( x^9/362880.0 )
```

Below is the taylor approwimation compared to the native sine function, generated in Julia:
The error of the Taylor aproximation is magnified by 100 and showed as well. 
![Image](numerical_verificatons/figures/time_domain_sinx.png?raw=true)

Allthough small, the error introduces strong harmonic tones, which can be seen below.
![Image](numerical_verificatons/figures/frequency_domain_sinx.png?raw=true)

To counter these, the fact that all cuarters of the sine(x) function are mirrored versions of each other. Therefore the first quarters, having the least error, which can be seen in the time domain plot above, can be used for all values of x, with the below correction:

The function below is written in Julia, see /src/real/array.rs for the Rust implementation.
```julia
function sine( x::Float64 ) 
        if x < -π/2
            x = -π/2 + abs(x+π/2);
        elseif π/2 < x
            x = π/2 - abs(x-π/2);
        end
    return x-( x^3/6.0 )+( x^5/120.0 )-( x^7/5040.0 )+( x^9/362880.0 );
end
```

The resulting spectrum is shown below. Note that these plots are generated using 64-bit floating point arithemtic, not integers as in this Rust library.
![Image](numerical_verificatons/figures/taylor_sine_comparison.png?raw=true)

This is a vast improvement showing close to no reduction in Singal to Noise and Distortion (SiNaD) compared to the native sine function in the Julia language.
The comparison can be found in numerical_verificatons/numerical_verificaton.jl.

The sine(x) function is used as follows:
```rust
declare_array_real!( Vec8, 8);
let mut x = Vec8::ramp(0,60);
x = x.wrap_phase( 180 );
assert_eq!{x.sin( 180, 100 ).data, [0, 86, 86, 0, -86, -86, 0, 86] };
```