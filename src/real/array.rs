#[cfg(feature = "std")]
mod std_support {
    use crate::{
        std::{
            string::String,
            fmt,
        },
        write,
    };
}

/// This macro implements a type which consists of an array of fixed-point numberts of size N.
/// Complete with the traits shown below.
/// 
/// ## Arguments
/// * `name`  - The name of the array type. E.g. Arr4
/// * `N`     - The length of the array. E.g 4.
/// * `T`     - The fixed type of the elements.
/// 
/// ## Example
/// Arrays types of fixed size is defined with traits through the macro as follows:
/// ```rust
/// use integer_array as ia;
/// use integer_array::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// // Declare the type.
/// integer_array::declare_array_real!( Arr11, 11, FixedI32<U20> );
/// ```
/// 
/// The name of the type for the above array is `Arr11`. The size of the error is 11 elements, and it has 18 fractional bits.
/// The implemented traits are documented below. See the source for a complete list.
/// 
/// # Initialization
/// The array can be initalized with a single value across the array, or by creating a ramp.
/// ### Generate an array of zeros.
/// `zeros` is made as separate trait for convenience.
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr2, 2, FixedI32<U20> );
/// let x = Arr2::zeros();
/// assert_eq!{x.to_i32(), [0, 0]};
/// ```
/// 
/// # New
/// For creating an array with a specific value.
/// 
/// ## Arguments
/// * `value` - The falue to fill each item with.
/// 
/// ## Example
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr2, 2, FixedI32<U20> );
/// let x = Arr2::new(FixedI32::<U20>::from_num(200));
/// assert_eq!{x.to_i32(), [200, 200]};
/// ```
/// 
/// To simplify decleration of arrays, float and int types are sypported through the following methods.
///  
/// `::new_from_i32( value )`, `::new_from_f32( value )`, `::new_from_f64( value )`.
/// 
/// # ::ones
/// `ones` is made as separate trait for convenience.
/// It generates an array of ones.
/// 
/// ## Example
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr2, 2, FixedI32<U20> );
/// let x = Arr2::ones();
/// assert_eq!{x.to_i32(), [1,1]};
/// ```
/// 
/// # ::ramp
/// Generate an array of increasing value.
/// 
/// ## Arguments
/// * `start` - The starting value (of item 0).
/// * `step`  - The incrimental value.
/// 
/// ## Example
/// `ramp_from_f32( value )` is added for convenience, but using ´ramp( value )´ with a value of the correct `fixed` type is also supported. 
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let x = Arr4::ramp_from_f32(100.0,20.0);
/// assert_eq!{x.to_i32(), [100, 120, 140, 160] };
/// ```
/// 
/// # ::front and ::back:
/// Access the first element of the array using the `.front()` trait.
/// 
/// ## Examples
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr2, 2, FixedI32<U20> );
/// let x = Arr2::new(FixedI32::<U20>::from_num(200));
/// assert_eq!{x.front(), 200};
/// ```
/// 
/// Access the last element of the array using the `.back()` trait.
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp_from_f32(100.0,20.0);
/// assert_eq!{x.back(), 720};
/// ```
///
/// # ::at
/// A specific item can be accessed through either using square bracket notation, or `.at( index )`.
/// 
/// ## Arguments
/// 
/// * `index` - The index of the item, in the range 0..N-1.
/// 
/// ## Example
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr2, 2, FixedI32<U20> );
/// let x = Arr2::zeros();
/// assert_eq!{x.at(1), 0};
/// ```
/// 
/// # Bracket indexing
/// Square bracket indexing can be used both for reading and writing from/to an item.
/// 
/// ## Examples
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let x = Arr8::ramp_from_f32(0.0,22.0);
/// assert_eq!{x[2], 44i32 };
/// ```
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp_from_f32(0.0,22.0);
/// x[2] = FixedI32::<U20>::from_num(56);
/// assert_eq!{x[2], 56i32 };
/// ```
/// 
/// # ::bias
/// The `bias` trai adds a scalar bias to every element in the array.
/// 
/// ## Arguments
/// 
/// * `value` - The bias amount.
/// 
/// ## Examples
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr2, 2, FixedI32<U20> );
/// let x = Arr2::new_from_i32(200);
/// let y = x.bias_f32(0.25);
/// assert_eq!{y.front(), 200.25};
/// ```
///
/// The bias can also be implemented by adding or subtracting the array with a scalar. 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 8, FixedI32<U20> );
/// let mut x = Arr4::ramp_from_f32(0.0,22.0);
/// x = x+3;
/// assert_eq!{x[1], 25i32 };
/// 
/// let mut x = Arr4::ramp_from_f32(0.0,22.0);
/// x = x-3;
/// assert_eq!{x[1], 19i32 };
/// ```
/// 
/// # ::scale
/// The `scale` trait scales every element in the array with a scalar value.
/// 
/// ## Arguments
/// 
/// * `value` - The scaling factor.
/// 
/// ## Examples
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr2, 2, FixedI32<U20> );
/// let x = Arr2::new_from_i32(100);
/// let y = x.scale_f32(5.0);
///    assert_eq!{y.front(), 500};
/// ```
///
/// Scaling can also be used by simply multiplying the array with a scalar value.
/// 
///  ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp_from_f32(0.0,22.0);
/// x = x*3;
/// assert_eq!{x[1], 66i32 };
/// ```
/// 
/// or through a division.
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let mut x = Arr4::ramp_from_f32(0.0,22.0);
/// x = 1000/x;
/// assert_eq!{x.to_f32(), [2048.0, 45.454544, 22.727272, 15.151515] };
/// ```
/// 
/// # ::sqrt
/// The `sqrt` trait finds the item-wise square root of array.
/// 
/// ## Argument
/// 
/// * `error` - The gratest allowed error in the numerical method.
/// 
/// ## Example
/// 
/// ```rust 
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let x = Arr4::ramp_from_f32(18.0, 10.0);
/// let y = x.sqrt( FixedI32::<U20>::from_num(0.1) );
/// assert_eq!{ y[1], 5.300915f32 };
/// ```
/// 
/// # Array operations
/// Operations can also be performed on an inter-array-basis.
/// The arrays must be of the same size.
/// The operations are written similarly as one would for scalars in Rust.
/// 
/// ## Examples
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let mut x = Arr4::ramp_from_f32(0.0,22.0);
/// let y = Arr4::new_from_i32(10);
/// x = x+y;
/// assert_eq!{x.to_i32(), [10,32,54,76] };
/// 
/// x = x-y;
/// assert_eq!{x.to_i32(), [0,22,44,66] };
/// ```
///
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let mut x = Arr4::ramp_from_f32(10.0,22.0);
/// let  y = Arr4::new_from_i32(10);
/// x = x*y;
/// assert_eq!{x.data, [100,320,540,760] };
/// 
/// x = Arr4::ramp_from_f32(0.0,22.0);
/// x = x/y;
/// assert_eq!{x.to_i32(), [0,2,4,6] };
/// ```
/// 
/// In a divide-by-zero case, the maximum int value is returned. 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let mut x = Arr4::ramp_from_f32(0.0,22.0);
/// let y = Arr4::new_from_i32(1);
/// x = y/x;
/// assert_eq!{x.to_f32(), [2048.0, 0.045454025, 0.022727013, 0.015151024] };
/// ```
///
/// # ::wrap_phase
/// Wrap array to a fixed-point -π=<x<π range.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let x = Arr8::ramp_from_f32(0.0, 3.1415/3.0);
/// let y = x.wrap_phase();
/// assert_eq!{ y.to_f32(), [0.0, 1.0471668, 2.0943336, 3.1415005, -2.0945168, -1.0473499, -0.00018310547, 1.0469837] };
/// ```
/// 
/// # ::sin
/// Take the elemtent-wise sine using a Taylor approximation of sine x.
/// 
/// Sin is calculated using the following polynomial:
/// 
/// `sin(x) = x -( x^3/6.0 )+( x^5/120.0 )-( x^7/5040.0 )+( x^9/362880.0 )`
/// 
/// Self must be wrapped to the -π=<x<π range.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp_from_f32(0.0,3.1415/6.0);
/// x = x.wrap_phase();
/// let y = x.sin();
/// assert_eq!{ y.to_f32(), [0.0, 0.49998665, 0.8660097, 1.0000038, 0.86605644, 0.50006676, 0.000091552734, -0.4999075] };
/// ```
/// 
/// Below is the the taylor approximation for sine compared to the Julia native sin function.
/// The below plot is generated with double presicion floationg point for demonstrative purposes.
/// In the figure it is apparent that there is greater error in the Taylor approximation further from origo.
/// ![Alt version](https://github.com/ErikBuer/Integer-Array/blob/main/numerical_verificatons/figures/sin/time_domain_sinx.png?raw=true)
/// To counter these, the fact that all quarters of the sine(x) function are mirrored versions of each other. 
/// Therefore the first quarters, having the least error, which can be seen in the time domain plot above, are used for all values of x.
/// Resulting in a practically ideal sine function, as can be seen in the frequency domain comparison below. 
/// ![Alt version](https://github.com/ErikBuer/Integer-Array/blob/main/numerical_verificatons/figures/sin/time_domain_comparison2.png?raw=true)
/// 
/// 
/// # ::cos
/// Take the elemtent-wise cosine using a Taylor approximation of cos x.
/// 
/// Cos is calculated using the following polynomial:
/// 
/// `cos(x) = 1 -( x^2/2 )+( x^4/24.0 )-( x^6/720.0 )+( x^8/40320.0 )`
/// 
/// Self must be wrapped to the -π=<x<π range.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp_from_f32(0.0,60.0);
/// x = x.wrap_phase( );
/// let y = x.cos();
/// assert_eq!{ y.to_f32(), [1.0, -0.95240974, 0.814167, -0.5984316, 0.3257389, -0.022058487, -0.2837639, 0.56254864] };
/// ```
/// 
/// A first-quarter method as described for the sine implementation is also used on cosine. The pure Taylor approximation is displayed below.
/// ![Alt version](https://github.com/ErikBuer/Integer-Array/blob/main/numerical_verificatons/figures/cos/time_domain.png?raw=true)
/// With the first-quarter method, the resulting cosine power spectrum is displayed below.
/// ![Alt version](https://github.com/ErikBuer/Integer-Array/blob/main/numerical_verificatons/figures/cos/frequency_domain2.png?raw=true)
/// 
/// 
/// 
/// # ::tan
/// Take the element-wise tan using a Taylor approximation of tan x.
/// 
/// Tan is calculated using the following polynomial:
/// 
/// `tan(x) = x+( x^3/3 )+( x^5*2/15.0 )+( x^7*17/315.0 )+( x^9*62/2835.0 )+( x^11*1382/155925.0 )+( x^13*21844/6081075.0 )+( x^15*929569/638512875.0 )`
/// 
/// Self must be wrapped to the -π/2=<x<π/2 range.
/// The function is based on a Taylor expansion. Its error increases as |x| approaches π/2.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U4, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U4> );
/// let x = Arr8::ramp_from_f32(0.0,0.17);
/// let y = x.tan();
/// assert_eq!{ y.to_f32(), [0.0, 0.1875, 0.375, 0.5625, 0.875, 1.25, 1.6875, 2.5625] };
/// ```
/// 
/// Below is the the taylor approximation for tan compared to the Julia native tan function.
/// The below plot is generated with double presicion floationg point for demonstrative purposes.
/// ![Alt version](https://github.com/ErikBuer/Integer-Array/blob/main/numerical_verificatons/figures/tan/time_domain.png?raw=true)
///
/// # ::atan
/// Calculate atan(x) using a polynomial approximation.
/// Utilizes the following polynomial to estimate the angle θ \[radians\].
/// 
/// `atan(x) = ((x)+0.372003(x)^3) / (1+0.703384(x)^2 + 0.043562(x)^4)`
/// 
/// The method is accurat within 0.003 degrees when |θ|<=π/4.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Etition, IEEE Press, 2012.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U4, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U4> );
/// let x = Arr8::ramp_from_f32(0.0,0.1);
/// let y = x.atan();
/// assert_eq!{ y.to_f32(), [0.0, 0.125, 0.25, 0.3125, 0.4375, 0.5, 0.625, 0.6875] };
/// ```
/// 
/// # ::max and ::min
/// 
/// The maimum and minimum value in the array can be found through the `.max()` and `.min()` traits respectively.
/// 
/// ## Examples
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp_from_f32(100.0,20.0);
/// assert_eq!{x.max(), 720};
/// ```
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp_from_f32(100.0,20.0);
/// assert_eq!{x.min(), 100};
/// ```
/// 
/// # ::argmax and ::argmin
/// 
/// The index of the maximum and minimum items in the array can be found through the `.argmax()` and `.argmin()` traits respectively.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp_from_f32(0.0,1.0);
/// assert_eq!{x.argmax(), 31};
/// ```
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp_from_f32(100.0,20.0);
/// assert_eq!{x.argmin(), 0};
/// ```
#[macro_export]
macro_rules! declare_array_real{
    ( $name:ident, $N:expr, $T:ty ) => {

        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        /// Real numeric array of type int32.
        pub struct $name{
            pub data: [$T; $N],
        }

        impl $name {
            /// Generate an array of a value.
            #[allow(dead_code)]
            fn new( value: $T ) -> Self
            {
                $name {
                    data: [value; $N],
                }
            }
            #[allow(dead_code)]
            fn new_from_i32( value: i32 ) -> Self
            {
                $name {
                    data: [<$T>::from_num(value); $N],
                }
            }
            #[allow(dead_code)]
            fn new_from_f32( value: f32 ) -> Self
            {
                $name {
                    data: [<$T>::from_num(value); $N],
                }
            }
            #[allow(dead_code)]
            fn new_from_f64( value: f64 ) -> Self
            {
                $name {
                    data: [<$T>::from_num(value); $N],
                }
            }
            #[allow(dead_code)]
            fn new_from_f32_array( in_arr: [f32; $N] ) -> Self
            {
                let mut r_array:Self = Self::new_from_i32(0);
                for n in 0..$N {
                    r_array[n] = <$T>::from_num(in_arr[n]);
                }
                return r_array;
            }
            #[allow(dead_code)]
            fn new_from_f64_array( in_arr: [f32; $N] ) -> Self
            {
                let mut r_array:Self = Self::new_from_i32(0);
                for n in 0..$N {
                    r_array[n] = <$T>::from_num(in_arr[n]);
                }
                return r_array;
            }
            /// Return self as a primitive array of floats. 
            #[allow(dead_code)]
            fn to_f32( &self ) -> [f32; $N]
            {
                let mut r_array: [f32; $N] = [0.0; $N];
                for n in 0..$N {
                    r_array[n] = self[n].to_num::<f32>();
                }
                return r_array;
            }
            /// Return self as a primitive array of floats. 
            #[allow(dead_code)]
            fn to_i32( &self ) -> [i32; $N]
            {
                let mut r_array: [i32; $N] = [0; $N];
                for n in 0..$N {
                    r_array[n] = self[n].to_num::<i32>();
                }
                return r_array;
            }
        }

        impl $name {
            /// Generate a linear ramp of values with increment step.
            #[allow(dead_code)]
            fn ramp( start: $T, step: $T ) -> Self {
                let mut temp: [$T; $N] = [<$T>::from_num(0); $N];
                for n in 0..$N {
                    temp[n] = start+((<$T>::from_num(n))*step);
                }
                $name {
                    data: temp
                }
            }
            #[allow(dead_code)]
            fn ramp_from_f32( start: f32, step: f32 ) -> Self {
                let mut temp: [$T; $N] = [<$T>::from_num(0); $N];
                for n in 0..$N {
                    temp[n] = <$T>::from_num(start)+((<$T>::from_num(n))*<$T>::from_num(step));
                }
                $name {
                    data: temp
                }
            }
        }

        impl integer_array::trait_definitions::Initializers for $name {
            /// Generate an array of ones.
            fn ones() -> Self {
                $name {
                    data: [<$T>::from_num(1); $N]
                }
            }
            /// Generate an array of zeroes.
            fn zeros() -> Self {
                $name {
                    data: [<$T>::from_num(0); $N]
                }
            }
        }

        impl integer_array::trait_definitions::Len for $name {
            /// Returns the length of the array.
            fn len( &self ) -> usize {
                return $N;
            }
        }

        impl $name {
            /// Returns indexed item of the array.
            /// Index Clips at N-1.
            #[allow(dead_code)]
            fn at( &self, index:usize) -> $T {
                if( $N <= index)
                {
                    return self[$N - 1];
                }
                return self[index];
            }
            /// Returns indexed item of the array as f32.
            /// Index Clips at N-1.
            #[allow(dead_code)]
            fn at_as_f32( &self, index:usize) -> i32 {
                if( $N <= index)
                {
                    return self[$N - 1].to_num::<i32>();
                }
                return self[index].to_num::<i32>();
            }
            /// Returns the first item of the array.
            #[allow(dead_code)]
            fn front( &self ) -> $T {
                return self[0];
            }
            /// Returns the last item of the array.
            #[allow(dead_code)]
            fn back( &self ) -> $T {
                return self[$N-1];
            }
        }

        impl $name {
            /// Return the elemtent-wise square root using the 
            /// Babylonian square root implementation.
            #[allow(dead_code)]
            fn sqrt( &self, error: $T ) -> Self {
                let mut r_array = self.clone();
                for index in 0..$N {
                    r_array[index] = integer_array::utility::sqrt( self[index] , error );
                } 
                return r_array;
            }
            /// Adds a scalar bias value to the entire array.
            fn bias( &self, value:$T ) -> Self
            {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self[index]+value;
                } 
                $name {
                    data: temp
                }
            }
            /// Adds a scalar bias value to the entire array.
            #[allow(dead_code)]
            fn bias_f32( &self, value:f32 ) -> Self
            {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self[index]+<$T>::from_num(value);
                } 
                $name {
                    data: temp
                }
            }
            /// Scales the array by a scalar value.
            fn scale( &self, value:$T ) -> Self
            {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self[index]*value;
                } 
                Self {
                    data: temp
                }
            }
            /// Scales the array by a scalar value.
            #[allow(dead_code)]
            fn scale_f32( &self, value:f32 ) -> Self
            {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self[index]*<$T>::from_num(value);
                } 
                Self {
                    data: temp
                }
            }
        }

        impl integer_array::trait_definitions::Pow for $name {
            /// Raise the items to an integer-valued power.
            fn powi( &self, power:u32 ) -> Self {
                let mut r_array = self.clone();
                for index in 0..$N {
                    r_array[index] = integer_array::utility::fixed_powi( self[index], power as usize );
                }
                return r_array;
            }
        }

        impl core::ops::Mul<$name> for $name {
            type Output = Self;
            fn mul( self, other:$name ) -> $name {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self[index]*other[index];
                } 
                Self {
                    data: temp
                }
            }
        }

        impl core::ops::Mul<i32> for $name {
            type Output = Self;
            fn mul( self, rhs:i32 ) -> $name {
                return self.scale( <$T>::from_num(rhs) );
            }
        }

        impl core::ops::Mul<f32> for $name {
            type Output = Self;
            fn mul( self, rhs:f32 ) -> $name {
                return self.scale( <$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Div<$name> for $name {
            type Output = Self;
            fn div( self, other:$name ) -> $name {
                let mut r_array = self.clone();
                for index in 0..$N {
                    if (self[index] == 0)
                    {
                        r_array[index] = <$T>::from_num(0);
                    }
                    else if(other[index] == 0)
                    {
                        r_array[index] = <$T>::MAX;
                    }
                    else
                    {
                        r_array[index] = self[index]/other[index];
                    }
                } 
                return r_array;
            }
        }

        impl core::ops::Div<i32> for $name {
            type Output = Self;
            fn div( self, other:i32 ) -> $name {
                let mut r_array = self.clone();
                for index in 0..$N {
                    if (self[index] == 0)
                    {
                        r_array[index] = <$T>::from_num(0);
                    }
                    else if(other == 0)
                    {
                        r_array[index] = <$T>::MAX;
                    }
                    else
                    {
                        r_array[index] = self[index]/other;
                    }
                } 
                return r_array;
            }
        }

        impl core::ops::Div<$name> for i32 {
            type Output = $name;
            fn div( self, other:$name ) -> $name {
                let mut r_array = other.clone();
                for index in 0..$N {
                    if (self == 0)
                    {
                        r_array[index] = <$T>::from_num(0);
                    }
                    else if(other[index] == 0)
                    {
                        r_array[index] = <$T>::MAX;
                    }
                    else
                    {
                        r_array[index] = <$T>::from_num(self)/other[index];
                    }
                } 
                return r_array;
            }
        }

        impl core::ops::Div<f32> for $name {
            type Output = Self;
            fn div( self, other:f32 ) -> $name {
                let mut r_array = self.clone();
                for index in 0..$N {
                    if (self[index] == 0)
                    {
                        r_array[index] = <$T>::from_num(0);
                    }
                    else if(other == 0.0)
                    {
                        r_array[index] = <$T>::MAX;
                    }
                    else
                    {
                        r_array[index] = self[index]/<$T>::from_num(other);
                    }
                } 
                return r_array;
            }
        }

        impl core::ops::Div<$name> for f32 {
            type Output = $name;
            fn div( self, other:$name ) -> $name {
                let mut r_array = other.clone();
                for index in 0..$N {
                    if (self == 0.0)
                    {
                        r_array[index] = <$T>::from_num(0);
                    }
                    else if(other[index] == 0)
                    {
                        r_array[index] = <$T>::MAX;
                    }
                    else
                    {
                        r_array[index] = <$T>::from_num(self)/other[index];
                    }
                } 
                return r_array;
            }
        }

        impl core::ops::Add<$name> for $name {
            type Output = Self;
            fn add( self, other:$name ) -> $name {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self[index]+other[index];
                } 
                Self {
                    data: temp
                }
            }
        }

        impl core::ops::Add<f32> for $name {
            type Output = Self;
            fn add( self, rhs:f32 ) -> $name {
                return self.bias( <$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Add<f64> for $name {
            type Output = Self;
            fn add( self, rhs:f64 ) -> $name {
                return self.bias( <$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Add<i32> for $name {
            type Output = Self;
            fn add( self, rhs:i32 ) -> $name {
                return self.bias( <$T>::from_num(rhs) ); 
            }
        }
        
        impl core::ops::Add<i16> for $name {
            type Output = Self;
            fn add( self, rhs:i16 ) -> $name {
                return self.bias( <$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Add<i8> for $name {
            type Output = Self;
            fn add( self, rhs:i8 ) -> $name {
                return self.bias( <$T>::from_num(rhs) ); 
            }
        }
        
        impl core::ops::Sub<i32> for $name {
            type Output = Self;
            fn sub( self, rhs:i32 ) -> $name {
                return self.bias( -<$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Sub<i8> for $name {
            type Output = Self;
            fn sub( self, rhs:i8 ) -> $name {
                return self.bias( -<$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Sub<i16> for $name {
            type Output = Self;
            fn sub( self, rhs:i16 ) -> $name {
                return self.bias( -<$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Sub<f32> for $name {
            type Output = Self;
            fn sub( self, rhs:f32 ) -> $name {
                return self.bias( -<$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Sub<f64> for $name {
            type Output = Self;
            fn sub( self, rhs:f64 ) -> $name {
                return self.bias( -<$T>::from_num(rhs) ); 
            }
        }

        impl core::ops::Sub<$name> for $name {
            type Output = Self;
            fn sub( self, other:$name ) -> $name {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self[index]-other[index];
                } 
                Self {
                    data: temp
                }
            }
        }

        impl core::ops::Neg for $name {
            type Output = Self;
            fn neg( self ) -> $name {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = -self.data[index];
                } 
                Self {
                    data: temp
                }
            }
        }

        impl core::ops::Sub<$name> for i32 {
            type Output = $name;
            fn sub( self, other: $name ) -> $name {
                return -other + self; 
            }
        }
        
        impl $name {
            /// Return the sum of the array.
            #[allow(dead_code)]
            fn sum( &self ) -> $T {
                let mut sum:$T = <$T>::from_num(0);
                for index in 0..$N {
                    sum = sum + self.data[index];
                }
                return sum;
            }
            /// Return the mean of the array.
            #[allow(dead_code)]
            fn mean( &self ) -> $T {
                let mut sum:$T = <$T>::from_num(0);
                for index in 0..$N {
                    sum = sum + self.data[index];
                }
                return sum/$N;
            }
            /// Return the variance of the array.
            #[allow(dead_code)]
            fn var( &self ) -> $T {
                let mean = self.mean();
                let mut temp: $T = <$T>::from_num(0);
                for idx in 0..$N {
                    temp = temp + integer_array::utility::fixed_powi((self[idx]-mean), 2);
                }
                return temp/$N;
            }
            /// Return the higherst value in the array.
            #[allow(dead_code)]
            fn max( &self ) -> $T {
                let mut max_val = <$T>::MIN;
                for index in 0..$N {
                    if max_val < self[index]
                    {
                        max_val = self[index];
                    }
                } 
                return max_val;
            }
            /// Return the lowest value in the array.
            #[allow(dead_code)]
            fn min( &self ) -> $T {
                let mut min_val = <$T>::MAX;
                for index in 0..$N {
                    if self.data[index] < min_val
                    {
                        min_val = self[index];
                    }
                } 
                return min_val;
            }
            /// Return the index of the greatest value in the array.
            #[allow(dead_code)]
            fn argmax( &self ) -> usize {
                let mut max_val = <$T>::MIN;
                let mut arg_max = 0;
                for index in 0..$N {
                    if max_val < self[index]
                    {
                        max_val = self[index];
                        arg_max = index;
                    }
                } 
                return arg_max;
            }
            /// Return the index of the lowest value in the array.
            #[allow(dead_code)]
            fn argmin( &self ) -> usize {
                let mut min_val = <$T>::MAX;
                let mut arg_min = 0;
                for index in 0..$N {
                    if self.data[index] < min_val
                    {
                        min_val = self[index];
                        arg_min = index;
                    }
                } 
                return arg_min;
            }
        }
        
        
        impl integer_array::trait_definitions::Sin for $name {
            /// Take the elemtent-wise sine using a Taylor approximation of sin(x).
            /// Self must be wrapped to the -π=<x<π range.
            fn sin( &self) -> Self {
                use integer_array::utility as util;

                let pi_half:$T = <$T>::from_num(fixed::consts::PI/2);

                let mut r_array = $name::new_from_i32(0);

                for idx in 0..$N {
                    let mut x = self[idx];

                    // Ensure that the angle is within the accurate range of the tailor series. 
                    if x < -pi_half
                    {   
                        let delta:$T = x+pi_half;
                        x = -pi_half+delta.abs();
                    }
                    else if pi_half < x
                    {
                        let delta:$T = x-pi_half;
                        x = pi_half-delta.abs();
                    }

                    // Calculate sine by using 
                    let sinx = x-( util::fixed_powi(x,3)/6 )+( util::fixed_powi(x,5)/120 )-( util::fixed_powi(x,7)/5040 )+( util::fixed_powi(x,9)/362880 );

                    r_array[idx] = sinx;
                } 
                return r_array;
            }
        }

        impl integer_array::trait_definitions::WrapPhase for $name {
            /// Wrapps Self to the -π=<x<π range.
            fn wrap_phase( &self ) -> Self {
                let mut temp_arr = self.data.clone();
                let pi = <$T>::from_num(fixed::consts::PI);
                for idx in 0..$N {
                    let mut temp_scalar = temp_arr[idx];
                    
                    while temp_scalar < -pi 
                    {
                        temp_scalar = &temp_scalar+2*pi;
                    }
                    while pi <= temp_scalar
                    {
                        temp_scalar = &temp_scalar-2*pi;
                    }
                    temp_arr[idx] = temp_scalar;
                } 
                Self {
                    data: temp_arr
                }
            }
        }


        impl integer_array::trait_definitions::Cos for $name {
            /// Take the elemtent-wise cosine using a Taylor approximation of cos(x).
            /// Self must be wrapped to the -π=<x<π range.
            fn cos( &self ) -> Self {
                use integer_array::utility as util;

                let pi_half:$T = <$T>::from_num(fixed::consts::PI/2);

                let mut r_array = $name::new_from_i32(0);
                for idx in 0..$N {
                    // Add a π/2 phase shift and run the sine computation from above.
                    let mut x = self[idx];
                    
                    let cosx:$T;
                    // Ensure that the angle is within the accurate range of the tailor series. 
                    if x < -pi_half
                    {   
                        let delta = x+pi_half;
                        x = -pi_half+delta.abs();
                        cosx = -(<$T>::ONE-( util::fixed_powi(x,2)/2 )+( util::fixed_powi(x,4)/24 )-( util::fixed_powi(x,6)/720 )+( util::fixed_powi(x,8)/40320 ));

                    }
                    else if pi_half < x
                    {
                        let delta = x-pi_half;
                        x = pi_half-delta.abs();
                        cosx = -(<$T>::ONE-( util::fixed_powi(x,2)/2 )+( util::fixed_powi(x,4)/24 )-( util::fixed_powi(x,6)/720 )+( util::fixed_powi(x,8)/40320 ));
                    }
                    else
                    {
                        cosx = <$T>::ONE-( util::fixed_powi(x,2)/2 )+( util::fixed_powi(x,4)/24 )-( util::fixed_powi(x,6)/720 )+( util::fixed_powi(x,8)/40320 );
                    }
                    r_array[idx] = cosx;
                } 
                return r_array;
            }
        }

        impl integer_array::trait_definitions::Tan for $name {
            /// Take the element-wise tan using a Taylor approximation of tan x.
            /// Self must be wrapped to the -π/2=<x<π/2 range.
            /// The function is based on a Taylor expansion. Its error increases as |x| approaches π/2.
            fn tan( &self ) -> Self {                
                use integer_array::utility as util;
                let mut r_array = $name::new_from_i32(0);
                for idx in 0..$N {
                    let x = self[idx];
                    // Calculate tan by using a polynomial 
                    r_array[idx] = x+( util::fixed_powi(x,3)/3 )+( util::fixed_powi(x,5)*2/15 )-( util::fixed_powi(x,7)*17/315 )+( util::fixed_powi(x,9)*62/2835 )+( util::fixed_powi(x,11)*1382/155925 )
                                    +( util::fixed_powi(x,13)*21844/6081075 )+( util::fixed_powi(x,15)*929569/638512875 );
                } 
                return r_array;
            }
        }

        impl integer_array::trait_definitions::Atan for $name {
            /// Take the element-wise atan using a Taylor approximation of tan x.
            fn atan( &self ) -> Self {                
                let mut r_array = $name::new_from_i32(0);
                for idx in 0..$N { 
                    r_array[idx] = integer_array::utility::atan_precise_fixed( self[idx] );
                } 
                return r_array;
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = $T;
            /// Trait for returning an indexed value of the array.
            #[inline]
            fn index(&self, index: usize) -> &$T {
                return &self.data[index];
            }
        }
        
        impl core::ops::IndexMut<usize> for $name {
            /// Trait for returning a mutable reference to indexed item.
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut $T {
                return &mut self.data[index];
            }
        }

        #[cfg(feature = "std")]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.data)
            }
        }

    }
}

// Test that are not needed for documentation are placed below.
#[cfg(test)]
mod tests {

    #[test]
    fn neg() {
        use crate as integer_array;
        use fixed::{types::extra::U20, FixedI32};

        declare_array_real!( Arr4, 4, FixedI32<U20> );
        let mut x = Arr4::new( FixedI32::<U20>::from_num(22));
        x = -x;
        assert_eq!{x[1], -22 };
    }

    #[test]
    fn zero_divide() {
        use crate as integer_array;
        use fixed::{types::extra::U20, FixedI32};
        
        declare_array_real!( Arr4, 4, FixedI32<U20> );
        let mut x = Arr4::ramp(FixedI32::<U20>::from_num(0),FixedI32::<U20>::from_num(22));
        let y = 10i32;
        x = x/y;
        assert_eq!{x[0], 0 };
    }

    #[test]
    fn sybtract_by_array() {
        use crate as integer_array;
        use fixed::{types::extra::U20, FixedI32};

        declare_array_real!( Arr4, 8, FixedI32<U20> );
        let mut x = Arr4::ramp(FixedI32::<U20>::from_num(0),FixedI32::<U20>::from_num(22));
        x = 3-x;
        assert_eq!{x[1], -19i32 };
    }

    
}
