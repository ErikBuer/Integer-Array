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
            fn at( &self, index:usize)  -> i32;
            fn front( &self )   -> i32;
            fn back( &self )    -> i32;
            fn len( &self )     -> usize;
        }

        impl VectorTraits for $name {
            fn new( value:i32 ) -> $name {
                $name {
                    data: [value;$N]
                }

            }
            fn ones() -> $name {
                $name {
                    data: [1;$N]
                }
            }
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
        /*
        pub trait ArithmeticTraits {
            fn add( value:i32 ) -> Self;
            fn sub( value:i32 ) -> Self;
            fn scale( value:i32 ) -> Self;
        }

        impl VectorTraits for $name {
            fn new( value:i32 ) -> $name {
                $name {
                    data: [value;$N]
                }

            }
        }
        */
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
}

