use super::*;

pub trait VectorTraits {
    fn new( value:i32 ) -> Self;
    fn ones()   -> Self;
    fn zeros()  -> Self;
    fn ramp( start:i32, step:i32  )  -> Self;
    fn at( &self, index:usize) -> i32;
    fn front( &self )   -> i32;
    fn back( &self )    -> i32;
    fn len( &self )     -> usize;
}

pub trait ArithmeticTraits {
    fn bias( &self, value:i32 )  -> Self;
    fn scale( &self, value:i32 ) -> Self;
    fn sqrt( &self )             -> Self;
}

pub trait TrigonometryTraits {
    fn sin( &self, norm_pi:i32, norm:i32 )  -> Self;
    fn tan( &self, norm_pi:i32, norm:i32 )  -> Self;
    fn wrap_phase( &self, norm_pi:i32)      -> Self;
}

pub trait StatisticTraits {
    fn sum( &self )  -> i32;
    fn mean( &self ) -> i32;
    fn var( &self )  -> i32;
    fn max( &self )  -> i32;
    fn min( &self )  -> i32;
    fn argmax( &self ) -> usize;
    fn argmin( &self ) -> usize;
}

/*
pub trait StdUtilities {
    fn todo( &self ) -> Self;
}
*/


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

pub const PI: f32 = 3.14159265358979323846264338327950288f32;

/// Rase integer to an integer-valued power.
/// base^power.
fn powi( base:i32, power:u32 ) -> i32 {
    let mut temp:i32 = base;
    for _i in 0..power {
        temp = temp*base;
    }
    return temp;
}

/// Rase float number to an integer-valued power.
/// base^power.
fn fpowi( base:f32, power:u32 ) -> f32 {
    let mut temp:f32 = base;
    for _i in 0..power {
        temp = temp*base;
    }
    return temp;
}

/// Create vector type of size N and type T.
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

        impl VectorTraits for $name {
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

        impl ArithmeticTraits for $name {
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
            /// Scales the vector by a scalar value.
            fn scale( &self, value:i32 ) -> Self {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self.data[index]*value;
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
        
        impl StatisticTraits for $name {
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

        impl TrigonometryTraits for $name {
            /// Take the item-wise sine using a Taylor approximation of sine x.
            /// Self must be wrapped to the -pi=<x<=pi range.
            /// * 'pi' The integer level which represents pi in the input data.
            /// * 'norm' The integer level which represents 1 in the output data.
            fn sin( &self, norm_pi:i32, norm:i32 ) -> Self {
                const PI_HALF:f32 = PI/2.0;

                let mut temp = self.data.clone();

                for idx in 0..$N {
                    let mut x:f32 = (temp[idx] as f32)*PI/(norm_pi as f32 );
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
                    let sinx:f32 = x-(fpowi(x,3)/6.0 )+( fpowi(x,5)/120.0 )-( fpowi(x,7)/5040.0 )+( fpowi(x,9)/362880.0 );
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

                for idx in 0..$N {
                    let x:f32 = (temp[idx] as f32)*PI/(norm_pi as f32 );
                    // Calculate tan by using a polynomial 
                    let tanx:f32 = x+(fpowi(x,3)/3.0 )+( fpowi(x,5)*2.0/15.0 )-( fpowi(x,7)*17.0/315.0 )+( fpowi(x,9)*62.0/2835.0 );
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

        #[cfg(feature = "std")]
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.data)
            }
        }

    }
}

declare_type_real!( Scalar, 1);
declare_type_real!( Vec2, 2);
declare_type_real!( Vec3, 3);
declare_type_real!( Vec4, 4);
declare_type_real!( Vec5, 5);
declare_type_real!( Vec6, 6);
declare_type_real!( Vec7, 7);
declare_type_real!( Vec8, 8);
declare_type_real!( Vec16, 16);
declare_type_real!( Vec24, 24);
declare_type_real!( Vec32, 32);
declare_type_real!( Vec64, 64);
declare_type_real!( Vec128, 128);
declare_type_real!( Vec256, 256);
declare_type_real!( Vec1024, 1024);
declare_type_real!( Vec4096, 4096);


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scalar_len() {
        let x = Vec2::zeros();
        assert_eq!{x.len(), 2};
    }
    #[test]
    fn test_scalar_at() {
        let x = Vec2::ones();
        assert_eq!{x.at(0), 1};
    }
    #[test]
    fn test_scalar_new() {
        let x = Vec2::new(200);
        assert_eq!{x.at(0), 200};
    }
    #[test]
    fn test_scalar_front() {
        let x = Vec2::new(200);
        assert_eq!{x.front(), 200};
    }
    #[test]
    fn test_scalar_back() {
        let x = Vec32::ramp(100,20);
        assert_eq!{x.back(), 720};
    }
    #[test]
    fn test_scalar_bias() {
        let x = Vec2::new(200);
        let y = x.bias(5);
        assert_eq!{y.front(), 205};
    }
    #[test]
    fn test_zeros() {
        let x = Vec2::zeros();
        assert_eq!{x.at(1), 0};
    }
    #[test]
    fn test_scalar_scale() {
        let x = Vec2::new(100);
        let y = x.scale(5);
        assert_eq!{y.front(), 500};
    }
    #[test]
    fn test_max() {
        let x = Vec32::ramp(100,20);
        assert_eq!{x.max(), 720};
    }
    #[test]
    fn test_min() {
        let x = Vec32::ramp(100,20);
        assert_eq!{x.min(), 100};
    }
    #[test]
    fn test_argmax() {
        let x = Vec32::ramp(100,20);
        assert_eq!{x.argmax(), 31};
    }
    #[test]
    fn test_argmin() {
        let x = Vec32::ramp(100,20);
        assert_eq!{x.argmin(), 0};
    }
    #[test]
    fn test_sqrt() {
        let x = Vec4::ramp(10000,1000);
        assert_eq!{x.sqrt().data, [100, 104, 109, 114] };
    }
    #[test]
    fn test_sin() {
        let x = Vec8::ramp(0,20);
        assert_eq!{x.sin( 180, 100).data, [1,2,3,4,5,6,7,8] };
    }
    #[test]
    fn test_tan() {//TODO
        let x = Vec8::ramp(0,20);
        assert_eq!{x.tan( 180, 100).data, [1,2,3,4,5,6,7,8] };
    }
    #[test]
    fn test_wrap_phase() {
        let x = Vec8::ramp(0,22);
        assert_eq!{x.wrap_phase( 50 ).data, [0,22,44,-34,-12,10,32,-46] };
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