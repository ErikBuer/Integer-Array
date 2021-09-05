/// Create vector type of size N.
#[macro_export]

macro_rules! declare_type_complex{
    ( $name:ident, $N:expr) => {

        #[derive(Clone, Debug, PartialEq)]
        /// Real numeric vector of type int32.
        pub struct $name{
            pub re: [i32; $N],
            pub im: [i32; $N],
        }

        impl numeric_vector::trait_definitions::NewComplex for $name {
            /// Generate a vector of a value.
            fn new( real:i32, imag:i32 ) -> Self {
                $name {
                    re: [real;$N]
                    im: [imag;$N]
                }
            }
        }

        impl numeric_vector::trait_definitions::VectorTraits for $name {
            /// Generate a vector of ones.
            fn ones() -> Self {
                $name {
                    re: [1;$N]
                    im: [0;$N]
                }
            }
            /// Generate a vector of zeroes.
            fn zeros() -> Self {
                $name {
                    re: [0;$N]
                    im: [0;$N]
                }
            }
            /// Returns the length of the vector.
            fn len( &self ) -> usize {
                return $N;
            }
        }

        impl numeric_vector::trait_definitions::VectorIndexingComplex for $name {
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
            fn back( &self ) -> Complex {
                return self.data[$N-1];
                Complex{
                    re: self.re[$N-1],
                    im: self.im[$N-1],
                }
            }
        }
    }
}
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_new() {
        use crate as numeric_vector;
        use numeric_vector::trait_definitions::*;
        declare_type_complex
    }
}
