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

/// This macro implements a i32 array type of size N.
/// Complete with the traits shown below.
/// 
/// ## Arguments
/// * `name`  - The name of the array type. E.g. Arr4
/// * `N`     - The length of the array. E.g 4.
/// 
/// ## Example
/// Arrays types of fixed size is defined with traits through a macro as follows:
/// ```rust
/// use integer_array as ia;
/// use integer_array::trait_definitions::*;
/// integer_array::declare_array_real!( Arr11, 11 );
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
/// assert_eq!{x.len(), 2};
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
/// assert_eq!{x.data, [200, 200]};
/// ```
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
/// assert_eq!{x[0], 1};
/// ```
/// 
/// # ::ramp
/// Generate a ramp of increasing value.
/// 
/// ## Arguments
/// * `start` - The starting value (of item 0).
/// * `step`  - The incrimental value.
/// 
/// ## Example
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let x = Arr4::ramp(100,20);
/// assert_eq!{x.data, [100, 120, 140, 160] };
/// ```
/// 
/// # ::front and ::back:
/// Access the first element of the array using the `.front()` attribute.
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
/// Access the last element of the array using the `.back()` attribute.
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp(100,20);
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
/// let x = Arr8::ramp(0,22);
/// assert_eq!{x[2], 44i32 };
/// ```
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp(0,22);
/// x[2] = 56;
/// assert_eq!{x[2], 56i32 };
/// ```
/// 
/// # ::bias
/// The `bias` attribute adds a scalar bias to every element in the array.
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
/// let x = Arr2::new(200);
/// let y = x.bias(5);
/// assert_eq!{y.front(), 205};
/// ```
///
/// The bias can also be implemented by adding or subtracting the array with a scalar. 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 8, FixedI32<U20> );
/// let mut x = Arr4::ramp(0,22);
/// x = x+3;
/// assert_eq!{x[1], 25i32 };
/// 
/// let mut x = Arr4::ramp(0,22);
/// x = x-3;
/// assert_eq!{x[1], 19i32 };
/// ```
/// 
/// # ::scale
/// The `scale` attribute scales every element in the array with a scalar value.
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
/// let x = Arr2::new(100);
/// let y = x.scale(5);
///    assert_eq!{y.front(), 500};
/// ```
///
/// Scaling can also be used by simply multiplying the array with a scalar value,
///  ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp(0,22);
/// x = x*3;
/// assert_eq!{x[1], 66i32 };
/// ```
/// 
/// or through a division.
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let mut x = Arr4::ramp(0,22);
/// x = 1000/x;
/// assert_eq!{x.data, [2147483647, 45, 22, 15] };
/// ```
/// 
/// # ::sqrt
/// The `sqrt` attribute finds the item-wise square root of array.
/// 
/// ## Argument
/// 
/// * `error` - The gratest allowed error in the result.
/// 
/// ## Example
/// 
/// ```rust 
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let x = Arr4::ramp(10000,1000);
/// assert_eq!{x.sqrt(  ).data, [100, 104, 109, 114] };
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
/// let mut x = Arr4::ramp(0,22);
/// let y = Arr4::new(10);
/// x = x+y;
/// assert_eq!{x.data, [10,32,54,76] };
/// 
/// x = x-y;
/// assert_eq!{x.data, [0,22,44,66] };
/// ```
///
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let mut x = Arr4::ramp(10,22);
/// let  y = Arr4::new(10);
/// x = x*y;
/// assert_eq!{x.data, [100,320,540,760] };
/// 
/// x = Arr4::ramp(0,22);
/// x = x/y;
/// assert_eq!{x.data, [0,2,4,6] };
/// ```
/// 
/// In a divide-by-zero case, the maximum int value is returned. 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr4, 4, FixedI32<U20> );
/// let mut x = Arr4::ramp(0,22);
/// let y = Arr4::new(1000);
/// x = y/x;
/// assert_eq!{x.data, [i32::MAX,45,22,15] };
/// ```
///
/// # ::wrap_phase
/// Wrap array to a fixed-point -π=<x<π range.
/// 
/// ## Arguments
/// 
/// * `pi` - The integer level which represents π in the input data.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let x = Arr8::ramp(0,22);
/// assert_eq!{x.wrap_phase( 50 ).data, [0,22,44,-34,-12,10,32,-46] };
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
/// ## Arguments
/// 
/// * `pi`   - The integer level which represents π in the input data.
/// * `norm` - The integer level which represents 1 in the output data.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp(0,60);
/// x = x.wrap_phase( 180 );
/// assert_eq!{x.sin( 180, 100).data, [0, 86, 86, 0, -86, -86, 0, 86] };
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
/// ## Arguments
/// 
/// * `pi`   - The integer level which represents π in the input data.
/// * `norm` - The integer level which represents 1 in the output data.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let mut x = Arr8::ramp(0,60);
/// x = x.wrap_phase( 180 );
/// assert_eq!{x.cos( 180, 100).data, [100, 50, -50, -100, -50, 50, 100, 50] };
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
/// ## Arguments
/// 
/// * `pi`   - The integer level which represents π in the input data.
/// * `norm` - The integer level which represents 1 in the output data.
/// 
/// ## Example
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr8, 8, FixedI32<U20> );
/// let x = Arr8::ramp(0,20);
/// assert_eq!{x.tan( 180, 100).data, [0, 36, 83, 158, 373, 2155, 19696, 158268] };
/// ```
/// 
/// Below is the the taylor approximation for tan compared to the Julia native tan function.
/// The below plot is generated with double presicion floationg point for demonstrative purposes.
/// ![Alt version](https://github.com/ErikBuer/Integer-Array/blob/main/numerical_verificatons/figures/tan/time_domain.png?raw=true)
/// 
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
/// let x = Arr32::ramp(100,20);
/// assert_eq!{x.max(), 720};
/// ```
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};

/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp(100,20);
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
/// let x = Arr32::ramp(0,1);
/// assert_eq!{x.argmax(), 31};
/// ```
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_real!( Arr32, 32, FixedI32<U20> );
/// let x = Arr32::ramp(100,20);
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
            fn ramp_from_float( start: f32, step: f32 ) -> Self {
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
        
        /*
        impl integer_array::trait_definitions::TrigonometryTraits for $name {
            /// Take the elemtent-wise sine using a Taylor approximation of sin(x).
            /// Self must be wrapped to the -π=<x<π range.
            /// * `pi`   -  The integer level which represents π in the input data.
            /// * `norm` - The integer level which represents 1 in the output data.
            fn sin( &self, norm_pi:i32, norm:i32 ) -> Self {
                use integer_array::utility as util;
                use integer_array::constants as cnst;

                const PI_HALF:f32 = cnst::PI/2.0;

                let mut r_array = $name::zeros();

                for idx in 0..$N {
                    let mut x:f32 = (self[idx] as f32)/(norm_pi as f32 ) *cnst::PI;

                    // Ensure that the angle is within the accurate range of the tailor series. 
                    if x < -PI_HALF
                    {   
                        let delta = x+PI_HALF;
                        x = -PI_HALF+delta.abs();
                    }
                    else if PI_HALF < x
                    {
                        let delta = x-PI_HALF;
                        x = PI_HALF-delta.abs();
                    }

                    // Calculate sine by using 
                    let sinx:f32 = x-( util::fpowi(x,3)/6.0 )+( util::fpowi(x,5)/120.0 )-( util::fpowi(x,7)/5040.0 )+( util::fpowi(x,9)/362880.0 );

                    r_array[idx] = ( sinx*(norm as f32) ) as i32;
                } 
                return r_array;
            }

            /// Take the elemtent-wise cosine using a Taylor approximation of cos(x).
            /// Self must be wrapped to the -π=<x<π range.
            /// * `pi`   - The integer level which represents π in the input data.
            /// * `norm` - The integer level which represents 1 in the output data.
            fn cos( &self, norm_pi:i32, norm:i32 ) -> Self {
                use integer_array::utility as util;
                use integer_array::constants as cnst;

                const PI_HALF:f32 = cnst::PI/2.0;

                let mut r_array = $name::zeros();
                for idx in 0..$N {
                    // Add a π/2 phase shift and run the sine computation from above.
                    let mut x:f32 = ((self[idx] as f32)/(norm_pi as f32 ) *cnst::PI);
                    
                    let cosx:f32;
                    // Ensure that the angle is within the accurate range of the tailor series. 
                    if x < -PI_HALF
                    {   
                        let delta = x+PI_HALF;
                        x = -PI_HALF+delta.abs();
                        cosx = -(1.0-( util::fpowi(x,2)/2.0 )+( util::fpowi(x,4)/24.0 )-( util::fpowi(x,6)/720.0 )+( util::fpowi(x,8)/40320.0 ));

                    }
                    else if PI_HALF < x
                    {
                        let delta = x-PI_HALF;
                        x = PI_HALF-delta.abs();
                        cosx = -(1.0-( util::fpowi(x,2)/2.0 )+( util::fpowi(x,4)/24.0 )-( util::fpowi(x,6)/720.0 )+( util::fpowi(x,8)/40320.0 ));
                    }
                    else
                    {
                        cosx = 1.0-( util::fpowi(x,2)/2.0 )+( util::fpowi(x,4)/24.0 )-( util::fpowi(x,6)/720.0 )+( util::fpowi(x,8)/40320.0 );
                    }
                    r_array[idx] = ( cosx*(norm as f32) ) as i32;
                } 
                return r_array;
            }

            /// Take the element-wise tan using a Taylor approximation of tan x.
            /// Self must be wrapped to the -π/2=<x<π/2 range.
            /// The function is based on a Taylor expansion. Its error increases as |x| approaches π/2.
            /// * `pi`   - The integer level which represents π in the input data.
            /// * `norm` - The integer level which represents 1 in the output data.
            fn tan( &self, norm_pi:i32, norm:i32 ) -> Self {
                let mut temp = self.data.clone();
                
                use integer_array::utility as util;
                use integer_array::constants as cnst;

                for idx in 0..$N {
                    let x:f32 = (temp[idx] as f32)*cnst::PI/(norm_pi as f32 );
                    // Calculate tan by using a polynomial 
                    let tanx:f32 = x+( util::fpowi(x,3)/3.0 )+( util::fpowi(x,5)*2.0/15.0 )-( util::fpowi(x,7)*17.0/315.0 )+( util::fpowi(x,9)*62.0/2835.0 )+( util::fpowi(x,11)*1382.0/155925.0 )
                                    +( util::fpowi(x,13)*21844.0/6081075.0 )+( util::fpowi(x,15)*929569.0/638512875.0 );
                    temp[idx] = ( tanx*(norm as f32) ) as i32;
                } 
                Self {
                    data: temp
                }
            }
            /// Wrapps Self to the -π=<x<π range.
            /// * `pi` - The integer level which represents π in the input data.
            fn wrap_phase( &self, norm_pi:i32 ) -> Self {

                let mut temp_arr = self.data.clone();
                for idx in 0..$N {
                    let mut temp_scalar = temp_arr[idx];
                    
                    while temp_scalar < -norm_pi 
                    {
                        temp_scalar = &temp_scalar+2*norm_pi;
                    }
                    while norm_pi <= temp_scalar
                    {
                        temp_scalar = &temp_scalar-2*norm_pi;
                    }
                    temp_arr[idx] = temp_scalar;
                } 
                Self {
                    data: temp_arr
                }
            }
        }
        */

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
