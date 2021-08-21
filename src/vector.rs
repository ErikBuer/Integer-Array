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
    fn bias( &self, value:i32 ) -> Self;
    fn scale( &self, value:i32 ) -> Self;
}

pub trait StatisticTraits {
    fn sum( &self ) -> i32;
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

/// Create vector type of size N and type T.
macro_rules! declare_type_real{
    ( $name:ident, $N:expr) => {

        #[derive(Clone, Debug, PartialEq)]
        /// Real numeric vector of type int32.
        pub struct $name{
            pub data: [i32; $N],
        }

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
                $name {
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
        let x = Scalar::ones();
        assert_eq!{x.at(0), 1};
    }
    #[test]
    fn test_scalar_new() {
        let x = Scalar::new(200);
        assert_eq!{x.at(0), 200};
    }
    #[test]
    fn test_scalar_front() {
        let x = Scalar::new(200);
        assert_eq!{x.front(), 200};
    }
    #[test]
    fn test_scalar_back() {
        let x = Vec32::ramp(100,20);
        assert_eq!{x.back(), 720};
    }
    #[test]
    fn test_scalar_bias() {
        let x = Scalar::new(200);
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
        let x = Scalar::new(100);
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