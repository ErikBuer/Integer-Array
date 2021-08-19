//use super::*;

/// Create vector type of size N and type T.
macro_rules! declare_type_real{
    ( $name:ident, $N:expr) => {
        
        #[derive(Clone, Debug, PartialEq)]
        /// Real numeric vector of type int32.
        pub struct $name{
            pub data: [i32; $N],
        }
        
        pub trait VectorTraits {
            fn new( value:i32 ) -> Self;
            fn ones()   -> Self;
            fn zeros()  -> Self;
            fn at( &self, index:usize) -> i32;
            fn front( &self ) -> i32;
            fn back( &self )-> i32;
            fn len( &self ) -> usize;
        }

        impl VectorTraits for $name {
            /// Generate a vector of a value.
            fn new( value:i32 ) -> $name {
                $name {
                    data: [value;$N]
                }

            }
            /// Generate a vector of ones.
            fn ones() -> $name {
                $name {
                    data: [1;$N]
                }
            }
            /// Generate a vector of zeroes.
            fn zeros() -> $name {
                $name {
                    data: [0;$N]
                }
            }
            /// A safe way of indexing the vector.
            /// Clips at N-1.
            fn at( &self, index:usize) -> i32 {
                if( $N <= index)
                {
                    return self.data[$N - 1];
                }
                return self.data[index];
            }
            fn front( &self ) -> i32 {
                return self.data[0];
            }
            fn back( &self ) -> i32 {
                return self.data[$N-1];
            }
            fn len( &self ) -> usize {
                return $N;
            }
        }
        
        pub trait ArithmeticTraits {
            fn bias( &self, value:i32 ) -> Self;
            fn scale( &self, value:i32 ) -> Self;
        }

        impl ArithmeticTraits for $name {
            fn bias( &self, value:i32 ) -> $name {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self.data[index]+value;
                } 
                $name {
                    data: temp
                }
            }
            fn scale( &self, value:i32 ) -> $name {
                let mut temp = self.data.clone();
                for index in 0..$N {
                    temp[index] = self.data[index]*value;
                } 
                $name {
                    data: temp
                }
            }
        }
        
        pub trait StatisticTraits {
            fn max( &self ) -> i32;
            fn min( &self ) -> i32;
            //fn argtmax( &self ) -> Self;
            //fn argmin(  &self ) -> Self;
        }
        
        impl StatisticTraits for $name {
            fn max( &self ) -> i32 {
                let mut max_val = i32::MAX;
                for index in 0..$N {
                    if max_val < self.data[index]
                    {
                        max_val = self.data[index];
                    }
                } 
                return max_val;
            }
            fn min( &self ) -> i32 {
                let mut min_val = i32::MIN;
                for index in 0..$N {
                    if min_val < self.data[index]
                    {
                        min_val = self.data[index];
                    }
                } 
                return min_val;
            }

        }        
    }
}

declare_type_real!( Scalar, 1);



#[cfg(test)]
mod tests {
    //use crate::*;
    use super::*;
    
    #[test]
    fn test_scalar_len() {
        let x = Scalar::zeros();
        assert_eq!{x.len(), 1};
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
    fn test_scalar_bias() {
        let x = Scalar::new(200);
        let y = x.bias(5);
        assert_eq!{y.front(), 205};
    }
    #[test]
    fn test_scalar_scale() {
        let x = Scalar::new(100);
        let y = x.scale(5);
        assert_eq!{y.front(), 500};
    }

    //TODO test max and min

}

