# Integer-Array
No-STD numeric fixed-size array for Rust.

This is an experimental library for no-std DSP.
The library only supports fixed-sized arrays of i32.
32-bit arrays can be handled by most DSP-capable embedded devices and provides 6.02×32>192 dB of dynamic range, which is sufficient for most DSP use.
The rationale is that if 64-bit processing is available, then so is an OS, and the Rust standard library. 

See the macro documentation for the implemented traits.

The package can be found at https://crates.io/crates/integer_array.
The documentation can be found at https://docs.rs/integer_array.

## Release notes are found under RELEASES.md

## Use example
Here are some use examples. Find the complete documentation at https://docs.rs/integer_array.

arrays types of fixed size is defined with traits through a macro as follows:
```rust
use integer_array as ia;
use integer_array::trait_definitions::*;

integer_array::declare_array_real!( Arr11, 11);
```
The name of the type for the above array is `Arr11`. The size of the error is 11 elements.

### Initialization
The array can be initalized with a single value across the array, or by creating a ramp.
```rust
ia::declare_array_real!( Arr2, 2);
let x = Arr2::new(6);
assert_eq!{x[0], 6};
```

Zeros is made as a separate trait for convenience.
```rust
ia::declare_array_real!( Arr2, 2);
let x = Arr2::zeros();
assert_eq!{x[0], 0};
```

### Indexing
The elements of the array can be indexed using square brackets as normal arrays
```rust
ia::declare_array_real!( Arr4, 4);
let mut x = Arr4::ramp(0,22);
x = -x;
assert_eq!{x[1], -22i32 };
```

In addition to this, there are utilities such as `.front()` and `.back()`.
```rust
ia::declare_array_real!( Arr2, 2);
let x = Arr2::new(200);
let y = x.bias(5);
assert_eq!{y.front(), 205};
```

### Element-wise arithmetic
Math can then be performed on the array types, like one would expect from a modern language.
```rust
let mut x = Arr11::new(66);
let y     = Arr11::new(2);
x = x/y;
assert_eq!(x.front(), 33);
```
### array-scalar operations
There is also some support for array-scalar operations.
```rust
ia::declare_array_real!( Arr4, 8);
let mut x = Arr4::ramp(0,22);
x = x+3;
assert_eq!{x[1], 25i32 };
```
### Esimator utilities
The arrays are equipped with traits for estimators, such as var, mean, max, min and argmax.
```rust
ia::declare_array_real!( Arr32, 32);
let x = Arr32::ramp(100,20);
assert_eq!{x.argmax(), 31};
```

### Sine generation
As this is a no-std library a cusom sine function is implemented. It utilizes a fifth order tailor approximation of sin(x). The trait implementation utilizes floating point multiplications, but returns a fixed point numeric array.

The following polynomial is used:
```julia
sin(x) = x-( x^3/6.0 )+( x^5/120.0 )-( x^7/5040.0 )+( x^9/362880.0 )
```

Below is the taylor approwimation compared to the native sine function, generated in Julia:
The error of the Taylor aproximation is magnified by 100 and showed as well. In the figure it is apparent that there is greater error in the Taylor approximation further from origo.
![Image](numerical_verificatons/figures/sin/time_domain_sinx.png?raw=true)

The error allthough small, introduces strong harmonic components, which can be seen below. These limits the practicality of the resulting signal.
![Image](numerical_verificatons/figures/sin/frequency_domain_sinx.png?raw=true)

To counter these, the fact that all quarters of the sine(x) function are mirrored versions of each other. Therefore the first quarters, having the least error, which can be seen in the time domain plot above, can be used for all values of x, with the below correction:

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
![Image](numerical_verificatons/figures/sin/taylor_sine_comparison.png?raw=true)

This is a vast improvement showing close to no reduction in Singal to Noise and Distortion (SiNaD) compared to the native sine function in the Julia language.

The sin(x) function is used as follows:
```rust
ia::declare_array_real!( Arr8, 8);
let mut x = Arr8::ramp(0,60);
x = x.wrap_phase( 180 );
assert_eq!{x.sin( 180, 100 ).data, [0, 86, 86, 0, -86, -86, 0, 86] };
```

The numerical comparison can be found in numerical_verificatons/numerical_verificaton.jl.