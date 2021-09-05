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

/// Create vector type of size N.
#[macro_export]
macro_rules! declare_type_real{
    ( $name:ident, $N:expr) => {

        #[derive(Clone, Debug, PartialEq)]
        /// Real numeric vector of type int32.
        pub struct $name{
            pub data: [i32; $N],
        }

        //TODO implement macro for creating vector.
        /*
        macro_rules! $name{
            ( $( $x:expr ),+ ) => {
                let temp:[i32;$N] = [ $($x),+ ];
                    $name {
                        data: temp,
                    }
            };
        }
        */

        impl numeric_vector::trait_definitions::VectorTraits for $name {
            /// Generate a vector of a value.
            fn new( value:i32 ) -> Self {
                $name {
                    data: [value;$N]
                }
            }
            /// Generate a vector of ones.
            fn ones() -> Self {
                $name {
                    data: [1;$N]
                }
            }
            /// Generate a vector of zeroes.
            fn zeros() -> Self {
                $name {
                    data: [0;$N]
                }
            }
            /// Generate a linear ramp of values with increment step.
            fn ramp( start:i32, step:i32  ) -> Self {
                let mut temp: [i32; $N] = [start; $N];
                for n in 0..$N {
                    temp[n] = start+(n as i32)*step;
                }
                $name {
                    data: temp
                }
            }
            /// Returns indexed item of the vector.
            /// Index Clips at N-1.
            fn at( &self, index:usize) -> i32 {
                if( $N <= index)
                {
                    return self.data[$N - 1];
                }
                return self.data[index];
            }
            /// Returns the first item of the vector.
            fn front( &self ) -> i32 {
                return self.data[0];
            }
            /// Returns the last item of the vector.
            fn back( &self ) -> i32 {
                return self.data[$N-1];
            }
            /// Returns the length of the vector.
            fn len( &self ) -> usize {
                return $N;
            }
        }   

        impl numeric_vector::trait_definitions::ArithmeticTraits for $name {
            /// Adds a scalar bias value to the entire vector.
            fn bias( &self, value:i32 ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self.data[index]+value;
                } 
                $name {
                    data: temp
                }
            }
            /// Scales the vector by a scalar integer value.
            fn scale( &self, value:i32 ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self.data[index]*value;
                } 
                Self {
                    data: temp
                }
            }
            /// Scales the vector by a scalar float value.
            fn scale_float( &self, value:f32 ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = (self.data[index]as f32*value) as i32;
                }
                Self {
                    data: temp
                }
            }
            /// Return the itemwise square root using the 
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
        
        impl numeric_vector::trait_definitions::StatisticTraits for $name {
            /// Return the sum of the vector.
            fn sum( &self ) -> i32 {
                let mut sum:i32 = 0;
                for index in 0..$N {
                    sum = sum+ self.data[index];
                }
                return sum;
            }
            /// Return the mean of the vector.
            fn mean( &self ) -> i32 {
                let mut sum:i32 = 0;
                for index in 0..$N {
                    sum = sum+ self.data[index];
                }
                return sum/$N;
            }
            /// Return the variance of the vector.
            fn var( &self ) -> i32 {
                let mean = self.mean();
                let mut temp: i32 = 0;
                for idx in 0..$N {
                    temp = temp + (self.data[idx]-mean)^2;
                }
                return temp/$N;
            }
            /// Return the higherst value in the vector.
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
            /// Return the lowest value in the vector.
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
            /// Return the index of the greatest value in the vector.
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
            /// Return the index of the lowest value in the vector.
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

        impl numeric_vector::trait_definitions::TrigonometryTraits for $name {
            /// Take the item-wise sine using a Taylor approximation of sine x.
            /// Self must be wrapped to the -pi=<x<=pi range.
            /// * 'pi' The integer level which represents pi in the input data.
            /// * 'norm' The integer level which represents 1 in the output data.

            fn sin( &self, norm_pi:i32, norm:i32 ) -> Self {
                use numeric_vector::utility_functions as util;
                use numeric_vector::constants as cnst;

                const PI_HALF:f32 = cnst::PI/2.0;

                let mut temp = self.data.clone();

                for idx in 0..$N {
                    let mut x:f32 = (temp[idx] as f32)*cnst::PI/(norm_pi as f32 );
                    // Ensure that the angle is within the accurate range of the tailor series. 
                    if x < -PI_HALF
                    {
                        x = &x+(2.0*(&x+PI_HALF));
                    }
                    else if PI_HALF < x
                    {
                        x = &x-(2.0*(&x-PI_HALF));
                    }

                    // Calculate sine by using 
                    let sinx:f32 = x-(util::fpowi(x,3)/6.0 )+( util::fpowi(x,5)/120.0 )-( util::fpowi(x,7)/5040.0 )+( util::fpowi(x,9)/362880.0 );
                    temp[idx] = ( sinx*(norm as f32) ) as i32;
                } 
                Self {
                    data: temp
                }
            }
            /// Take the item-wise tan using a Taylor approximation of tan x.
            /// Self must be wrapped to the -pi=<x<=pi range.
            /// * 'pi' The integer level which represents pi in the input data.
            /// * 'norm' The integer level which represents 1 in the output data.
            fn tan( &self, norm_pi:i32, norm:i32 ) -> Self {
                let mut temp = self.data.clone();
                
                use numeric_vector::utility_functions as util;
                use numeric_vector::constants as cnst;


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
            /// Wrapps Self to the -pi=<x<pi range.
            /// * 'pi' The integer level which represents pi in the input data.
            fn wrap_phase( &self, norm_pi:i32 ) -> Self {

                let mut temp_vec = self.data.clone();
                for idx in 0..$N {
                    let mut temp_scalar = temp_vec[idx];
                    
                    while temp_scalar < -norm_pi 
                    {
                        temp_scalar = &temp_scalar+2*norm_pi;
                    }
                    while norm_pi <= temp_scalar
                    {
                        temp_scalar = &temp_scalar-2*norm_pi;
                    }
                    temp_vec[idx] = temp_scalar;
                } 
                Self {
                    data: temp_vec
                }
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = i32;
            /// Trait for returning an indexed value of the vector.
            #[inline]
            fn index(&self, index: usize) -> &i32 {
                return &self.data[index];
            }
        }
        
        impl core::ops::IndexMut<usize> for $name {
            /// Trait for returning a mutable reference to indexed item.
            /// ´´´
            /// use crate as numeric_vector;
            /// use numeric_vector::trait_definitions::*;
            /// declare_type_real!( Vec8, 8);
            /// let mut x = Vec8::ramp(0,22);
            /// x[2] = 56;
            /// assert_eq!{x[2], 56i32 };
            /// ´´´
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

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_scalar_len() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec2, 2);
        let x = Vec2::zeros();
        assert_eq!{x.len(), 2};
    }
    #[test]
    fn test_scalar_at() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec2, 2);
        let x = Vec2::ones();
        assert_eq!{x.at(0), 1};
    }
    #[test]
    fn test_scalar_new() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec2, 2);
        let x = Vec2::new(200);
        assert_eq!{x.at(0), 200};
    }
    #[test]
    fn test_scalar_front() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        let x = Vec2::new(200);
        declare_type_real!( Vec2, 2);
        assert_eq!{x.front(), 200};
    }
    #[test]
    fn test_scalar_back() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec32, 32);
        let x = Vec32::ramp(100,20);
        assert_eq!{x.back(), 720};
    }
    #[test]
    fn test_scalar_bias() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec2, 2);
        let x = Vec2::new(200);
        let y = x.bias(5);
        assert_eq!{y.front(), 205};
    }
    #[test]
    fn test_zeros() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec2, 2);
        let x = Vec2::zeros();
        assert_eq!{x.at(1), 0};
    }
    #[test]
    fn test_scalar_scale() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec2, 2);
        let x = Vec2::new(100);
        let y = x.scale(5);
        assert_eq!{y.front(), 500};
    }
    #[test]
    fn test_max() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec32, 32);
        let x = Vec32::ramp(100,20);
        assert_eq!{x.max(), 720};
    }
    #[test]
    fn test_min() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec32, 32);
        let x = Vec32::ramp(100,20);
        assert_eq!{x.min(), 100};
    }
    #[test]
    fn test_argmax() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec32, 32);
        let x = Vec32::ramp(100,20);
        assert_eq!{x.argmax(), 31};
    }
    #[test]
    fn test_argmin() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec32, 32);
        let x = Vec32::ramp(100,20);
        assert_eq!{x.argmin(), 0};
    }
    #[test]
    fn test_sqrt() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec4, 4);
        let x = Vec4::ramp(10000,1000);
        assert_eq!{x.sqrt().data, [100, 104, 109, 114] };
    }
    #[test]
    fn test_sin() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec8, 8);
        let x = Vec8::ramp(0,20);
        assert_eq!{x.sin( 180, 100).data, [1,2,3,4,5,6,7,8] };
    }
    #[test]
    fn test_tan() {//TODO
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec8, 8);
        let x = Vec8::ramp(0,20);
        assert_eq!{x.tan( 180, 100).data, [1,2,3,4,5,6,7,8] };
    }
    #[test]
    fn test_wrap_phase() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec8, 8);
        let x = Vec8::ramp(0,22);
        assert_eq!{x.wrap_phase( 50 ).data, [0,22,44,-34,-12,10,32,-46] };
    }
    #[test]
    fn test_index() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec8, 8);
        let x = Vec8::ramp(0,22);
        assert_eq!{x[2], 44i32 };
    }
    #[test]
    fn test_mut_index() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec8, 8);
        let mut x = Vec8::ramp(0,22);
        x[2] = 56;
        assert_eq!{x[2], 56i32 };
    }
    #[test]
    fn test_mul() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec8, 8);
        let mut x = Vec8::ramp(0,22);
        x = x*3;
        assert_eq!{x[1], 66i32 };
    }
    #[test]
    fn test_add() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec4, 8);
        let mut x = Vec4::ramp(0,22);
        x = x+3;
        assert_eq!{x[1], 25i32 };
    }
    #[test]
    fn test_neg() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_real!( Vec4, 8);
        let mut x = Vec4::ramp(0,22);
        x = -x;
        assert_eq!{x[1], -22i32 };
    }

    //TODO implement macro.
    /*
    #[test]
    fn test_macro_generation() {
        let x = Vec2![1,2];
        assert_eq!{x, Vec2::ramp(1,1) };
    }
    */
}


#[cfg(feature = "std")]
mod std_support {
    use super::*;

    #[test]
    fn test_std_display() {
        let x = Vec32::ramp(100,20);
        println! ("{}", x);
        assert_eq!{x.max(), 720};
    }
}