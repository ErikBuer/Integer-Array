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

/// Create an i32 array type of size N.
/// Complete with the traits shown below.
/// 
/// # Traits implemented with the macro:
/// The implemented traits are focumented below. See the source for a complete list.
/// ## Basic use:
/// arrays types of fixed size is defined with traits through a macro as follows:
/// ```rust
/// use numeric_array as na;
/// use numeric_array::trait_definitions::*;
/// numeric_array::declare_array_real!( Arr11, 11);
/// ```
// The name of the type for the above array is `Arr11`. The size of the error is 11 elements.
/// 
/// ## Initialization
/// The array can be initalized with a single value across the array, or by creating a ramp.
/// ### Generate an array of zeros.
/// `zeros` is made as separate trait for convenience.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr2, 2);
/// let x = Arr2::zeros();
/// assert_eq!{x.len(), 2};
/// ```
/// 
/// ### Generate an array of ones.
/// `ones` is made as separate trait for convenience.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr2, 2);
/// let x = Arr2::ones();
/// assert_eq!{x[0], 1};
/// ```
/// 
/// ### Generate an array of a specific value.
/// An array of a single value is generated using the `new` keyword.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr2, 2);
/// let x = Arr2::new(200);
/// assert_eq!{x.data, [200, 200]};
/// ```
/// 
/// Generate a ramp of increasing value.
/// - `start` The starting value (of item 0).
/// - `step`  The incrimental value.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr4, 4);
/// let x = Arr4::ramp(100,20);
/// assert_eq!{x.data, [100, 120, 140, 160] };
/// ```
/// 
/// ## Indexing:
/// ### Front and back:
/// Access the first element of the array using the `.front()` attribute.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr2, 2);
/// let x = Arr2::new(200);
/// assert_eq!{x.front(), 200};
/// ```
/// 
/// Access the last element of the array using the `.back()` attribute.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr32, 32);
/// let x = Arr32::ramp(100,20);
/// assert_eq!{x.back(), 720};
/// ```
///
/// ### At
/// A specific item can be accessed through either using square bracket notation, or `.at( index )`.
/// - `index` The index of the item, in the range 0..N-1.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr2, 2);
/// let x = Arr2::zeros();
/// assert_eq!{x.at(1), 0};
/// ```
/// 
/// ### Bracket indexing
/// Square bracket indexing can be used both for reading and writing from/to an item.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr8, 8);
/// let x = Arr8::ramp(0,22);
/// assert_eq!{x[2], 44i32 };
/// ```
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr8, 8);
/// let mut x = Arr8::ramp(0,22);
/// x[2] = 56;
/// assert_eq!{x[2], 56i32 };
/// ```
/// 
/// ## Scalar-array arithemetic:
/// ### Bias
/// The `bias` attribute adds a scalar bias to every element in the array.
/// - `value` The bias amount.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr2, 2);
/// let x = Arr2::new(200);
/// let y = x.bias(5);
/// assert_eq!{y.front(), 205};
/// ```
///
/// The bias can also be implemented by adding or subtracting the array with a scalar. 
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr4, 8);
/// let mut x = Arr4::ramp(0,22);
/// x = x+3;
/// assert_eq!{x[1], 25i32 };
/// 
/// let mut x = Arr4::ramp(0,22);
/// x = x-3;
/// assert_eq!{x[1], 19i32 };
/// ```
/// 
/// ### Scale
/// The `scale` attribute scales every element in the array with a scalar value.
/// - `value` The scaling factor.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr2, 2);
/// let x = Arr2::new(100);
/// let y = x.scale(5);
///    assert_eq!{y.front(), 500};
/// ```
///
/// Scaling can also be used by simply multiplying the array with a scalar value. 
///  ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr8, 8);
/// let mut x = Arr8::ramp(0,22);
/// x = x*3;
/// assert_eq!{x[1], 66i32 };
/// ```
/// 
/// or through a division.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr4, 4);
/// let mut x = Arr4::ramp(0,22);
/// x = 1000/x;
/// assert_eq!{x.data, [2147483647, 45, 22, 15] };
/// ```
/// 
/// ### Sqrt
/// The `sqrt` attribute finds the square root of every element in the array.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr4, 4);
/// let x = Arr4::ramp(10000,1000);
/// assert_eq!{x.sqrt().data, [100, 104, 109, 114] };
/// ```
/// 
/// ## Inter-array arithemetic:
/// Operations can also be performed on an inter-array-basis.
/// The arrays must be of the same size.
/// The operations are written similarly as one would for scalars in Rust.
/// 
/// ### Addition and subtraction
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr4, 4);
/// let mut x = Arr4::ramp(0,22);
/// let y = Arr4::new(10);
/// x = x+y;
/// assert_eq!{x.data, [10,32,54,76] };
/// 
/// x = x-y;
/// assert_eq!{x.data, [0,22,44,66] };
/// ```
///
///  ### Multiplication and division
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr4, 4);
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
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr4, 4);
/// let mut x = Arr4::ramp(0,22);
/// let y = Arr4::new(1000);
/// x = y/x;
/// assert_eq!{x.data, [i32::MAX,45,22,15] };
/// ```
/// 
/// ## Trigometric functions:
/// The following trigometric functions are implemented based on a taylor approximation.
/// 
/// ### Wrap phase 
/// Wrap array to a fixed-point -π=<x<π range.
/// * `pi` The integer level which represents π in the input data.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr8, 8);
/// let x = Arr8::ramp(0,22);
/// assert_eq!{x.wrap_phase( 50 ).data, [0,22,44,-34,-12,10,32,-46] };
/// ```
/// 
/// ### Sin
/// Take the elemtent-wise sine using a Taylor approximation of sine x.
/// Self must be wrapped to the -π=<x<π range.
/// * `pi` The integer level which represents π in the input data.
/// * `norm` The integer level which represents 1 in the output data.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr8, 8);
/// let mut x = Arr8::ramp(0,60);
/// x = x.wrap_phase( 180 );
///    assert_eq!{x.sin( 180, 100).data, [0, 86, 86, 0, -86, -86, 0, 86] };
/// ```
/// 
/// ### Tan
/// Take the element-wise tan using a Taylor approximation of tan x.
/// * 'pi' The integer level which represents π in the input data.
/// * 'norm' The integer level which represents 1 in the output data.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr8, 8);
/// let x = Arr8::ramp(0,20);
/// assert_eq!{x.tan( 180, 100).data, [0, 36, 83, 155, 289, 630, 1794, 5875] };
/// ```
/// 
/// ## Estimator utilites:
/// ### Max and min:
/// The maimum and minimum value in the array can be found through the `.max()` and `.min()` traits respectively.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr32, 32);
/// let x = Arr32::ramp(100,20);
/// assert_eq!{x.max(), 720};
/// ```
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr32, 32);
/// let x = Arr32::ramp(100,20);
/// assert_eq!{x.min(), 100};
/// ```
/// ### Argmax and argmin:
/// The index of the maximum and minimum items in the array can be found through the `.argmax()` and `.argmin()` traits respectively.
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr32, 32);
/// let x = Arr32::ramp(0,1);
/// assert_eq!{x.argmax(), 31};
/// ```
/// ```rust
/// use numeric_array as na;
/// use na::trait_definitions::*;
/// na::declare_array_real!( Arr32, 32);
/// let x = Arr32::ramp(100,20);
/// assert_eq!{x.argmin(), 0};
/// ```
#[macro_export]
macro_rules! declare_array_real{
    ( $name:ident, $N:expr) => {

        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        /// Real numeric array of type int32.
        pub struct $name{
            pub data: [i32; $N],
        }

        impl numeric_array::trait_definitions::New for $name {
            /// Generate an array of a value.
            fn new( value:i32 ) -> Self {
                $name {
                    data: [value;$N]
                }
            }
        }

        impl numeric_array::trait_definitions::Ramp for $name {
            /// Generate a linear ramp of values with increment step.
            fn ramp( start:i32, step:i32  ) -> Self {
                let mut temp: [i32; $N] = [start; $N];
                for n in 0..$N {
                    temp[n] = start+((n as i32)*step);
                }
                $name {
                    data: temp
                }
            }
        }

        impl numeric_array::trait_definitions::Initializers for $name {
            /// Generate an array of ones.
            fn ones() -> Self {
                $name {
                    data: [1;$N]
                }
            }
            /// Generate an array of zeroes.
            fn zeros() -> Self {
                $name {
                    data: [0;$N]
                }
            }
        }

        impl numeric_array::trait_definitions::Len for $name {
            /// Returns the length of the array.
            fn len( &self ) -> usize {
                return $N;
            }
        }

        impl numeric_array::trait_definitions::ArrayIndexing for $name {
            /// Returns indexed item of the array.
            /// Index Clips at N-1.
            fn at( &self, index:usize) -> i32 {
                if( $N <= index)
                {
                    return self.data[$N - 1];
                }
                return self.data[index];
            }
            /// Returns the first item of the array.
            fn front( &self ) -> i32 {
                return self.data[0];
            }
            /// Returns the last item of the array.
            fn back( &self ) -> i32 {
                return self.data[$N-1];
            }
        }   

        impl numeric_array::trait_definitions::ArithmeticTraits for $name {
            /// Adds a scalar bias value to the entire array.
            fn bias( &self, value:i32 ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self.data[index]+value;
                } 
                $name {
                    data: temp
                }
            }
            /// Scales the array by a scalar integer value.
            fn scale( &self, value:i32 ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self.data[index]*value;
                } 
                Self {
                    data: temp
                }
            }
            /// Scales the array by a scalar float value.
            fn scale_float( &self, value:f32 ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = (self.data[index]as f32*value) as i32;
                }
                Self {
                    data: temp
                }
            }
            /// Return the elemtent-wise square root using the 
            /// Babylonian square root implementation.
            fn sqrt( &self ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    let item = self.data[index];
                    // Initial approximation
                    let mut root:i32 = item/2;
                    let mut y:i32 = 1;
                    // Accuracy level
                    let error:i32 = 1;
                    while ( error <= root - y)
                    {
                        root = (root + y) / 2;
                        y = item / root;
                    }
                    temp[index] = root;
                } 
                Self {
                    data: temp
                }
            }
        }

        impl numeric_array::trait_definitions::Pow for $name {
            /// Raise the items to an integer-valued power.
            fn powi( &self, power:u32 ) -> Self {
                let mut r_array = self.clone();
                for index in 0..$N {
                    r_array[index] = numeric_array::utility_functions::powi( self[index], power );
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
                return self.scale( rhs );
            }
        }

        impl core::ops::Mul<f32> for $name {
            type Output = Self;
            fn mul( self, rhs:f32 ) -> $name {
                return self.scale_float( rhs ); 
            }
        }

        impl core::ops::Div<$name> for $name {
            type Output = Self;
            fn div( self, other:$name ) -> $name {
                let mut r_array = self.clone();
                for index in 0..$N {
                    if (self[index] == 0)
                    {
                        r_array[index] = 0;
                    }
                    else if(other[index] == 0)
                    {
                        r_array[index] = i32::MAX;
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
                        r_array[index] = 0;
                    }
                    else if(other == 0)
                    {
                        r_array[index] = i32::MAX;
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
                        r_array[index] = 0;
                    }
                    else if(other[index] == 0)
                    {
                        r_array[index] = i32::MAX;
                    }
                    else
                    {
                        r_array[index] = self/other[index];
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

        impl core::ops::Add<i32> for $name {
            type Output = Self;
            fn add( self, rhs:i32 ) -> $name {
                return self.bias( rhs ); 
            }
        }
        
        impl core::ops::Add<i16> for $name {
            type Output = Self;
            fn add( self, rhs:i16 ) -> $name {
                return self.bias( rhs as i32 ); 
            }
        }

        impl core::ops::Add<i8> for $name {
            type Output = Self;
            fn add( self, rhs:i8 ) -> $name {
                return self.bias( rhs as i32 ); 
            }
        }
        
        impl core::ops::Sub<i32> for $name {
            type Output = Self;
            fn sub( self, other:i32 ) -> $name {
                return self.bias( -other ); 
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
        
        impl numeric_array::trait_definitions::StatisticTraits for $name {
            /// Return the sum of the array.
            fn sum( &self ) -> i32 {
                let mut sum:i32 = 0;
                for index in 0..$N {
                    sum = sum+ self.data[index];
                }
                return sum;
            }
            /// Return the mean of the array.
            fn mean( &self ) -> i32 {
                let mut sum:i32 = 0;
                for index in 0..$N {
                    sum = sum+ self.data[index];
                }
                return sum/$N;
            }
            /// Return the variance of the array.
            fn var( &self ) -> i32 {
                let mean = self.mean();
                let mut temp: i32 = 0;
                for idx in 0..$N {
                    temp = temp + (self.data[idx]-mean)^2;
                }
                return temp/$N;
            }
            /// Return the higherst value in the array.
            fn max( &self ) -> i32 {
                let mut max_val = i32::MIN;
                for index in 0..$N {
                    if max_val < self.data[index]
                    {
                        max_val = self.data[index];
                    }
                } 
                return max_val;
            }
            /// Return the lowest value in the array.
            fn min( &self ) -> i32 {
                let mut min_val = i32::MAX;
                for index in 0..$N {
                    if self.data[index] < min_val
                    {
                        min_val = self.data[index];
                    }
                } 
                return min_val;
            }
            /// Return the index of the greatest value in the array.
            fn argmax( &self ) -> usize {
                let mut max_val = i32::MIN;
                let mut arg_max = 0;
                for index in 0..$N {
                    if max_val < self.data[index]
                    {
                        max_val = self.data[index];
                        arg_max = index;
                    }
                } 
                return arg_max;
            }
            /// Return the index of the lowest value in the array.
            fn argmin( &self ) -> usize {
                let mut min_val = i32::MAX;
                let mut arg_min = 0;
                for index in 0..$N {
                    if self.data[index] < min_val
                    {
                        min_val = self.data[index];
                        arg_min = index;
                    }
                } 
                return arg_min;
            }
        }

        impl numeric_array::trait_definitions::TrigonometryTraits for $name {
            /// Take the elemtent-wise sine using a Taylor approximation of sine x.
            /// Self must be wrapped to the -π=<x<π range.
            /// * `pi` The integer level which represents π in the input data.
            /// * `norm` The integer level which represents 1 in the output data.
            fn sin( &self, norm_pi:i32, norm:i32 ) -> Self {
                use numeric_array::utility_functions as util;
                use numeric_array::constants as cnst;

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
                    assert!( sinx.abs()<1.0, "sinx = {}, x = {}", sinx, x);

                    r_array[idx] = ( sinx*(norm as f32) ) as i32;
                } 
                return r_array;
            }
            /// Take the element-wise tan using a Taylor approximation of tan x.
            /// Self must be wrapped to the -π=<x<π range.
            /// * `pi` The integer level which represents π in the input data.
            /// * `norm` The integer level which represents 1 in the output data.
            fn tan( &self, norm_pi:i32, norm:i32 ) -> Self {
                let mut temp = self.data.clone();
                
                use numeric_array::utility_functions as util;
                use numeric_array::constants as cnst;


                for idx in 0..$N {
                    let x:f32 = (temp[idx] as f32)*cnst::PI/(norm_pi as f32 );
                    // Calculate tan by using a polynomial 
                    let tanx:f32 = x+(util::fpowi(x,3)/3.0 )+( util::fpowi(x,5)*2.0/15.0 )-( util::fpowi(x,7)*17.0/315.0 )+( util::fpowi(x,9)*62.0/2835.0 );
                    temp[idx] = ( tanx*(norm as f32) ) as i32;
                } 
                Self {
                    data: temp
                }
            }
            /// Wrapps Self to the -π=<x<π range.
            /// * `pi` The integer level which represents π in the input data.
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

        impl core::ops::Index<usize> for $name {
            type Output = i32;
            /// Trait for returning an indexed value of the array.
            #[inline]
            fn index(&self, index: usize) -> &i32 {
                return &self.data[index];
            }
        }
        
        impl core::ops::IndexMut<usize> for $name {
            /// Trait for returning a mutable reference to indexed item.
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut i32 {
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
        use crate as numeric_array;
        use numeric_array::trait_definitions::*;
        declare_array_real!( Arr4, 4);
        let mut x = Arr4::ramp(0,22);
        x = -x;
        assert_eq!{x[1], -22i32 };
    }

    #[test]
    fn zero_divide() {
        use crate as numeric_array;
        use numeric_array::trait_definitions::*;
        declare_array_real!( Arr4, 4);
        let mut x = Arr4::ramp(0,22);
        let y = 10i32;
        x = x/y;
        assert_eq!{x.data, [0,2,4,6] };
    }

    #[test]
    fn sybtract_by_array() {
        use crate as numeric_array;
        use numeric_array::trait_definitions::*;
        declare_array_real!( Arr4, 8);
        let mut x = Arr4::ramp(0,22);
        x = 3-x;
        assert_eq!{x[1], -19i32 };
    }

    
}
