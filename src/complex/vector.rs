use super::*;

/// Create vector type of size N.
#[macro_export]
macro_rules! declare_vector_complex{
    ( $name:ident, $N:expr) => {

        #[derive(Clone, Debug, PartialEq)]
        /// Real numeric vector of type int32.
        pub struct $name{
            pub data: [numeric_vector::complex::Complex; $N],
        }

        impl numeric_vector::trait_definitions::NewComplex for $name {
            /// Generate a vector of a value.
            fn new( real:i32, imag:i32 ) -> Self {
                let item =  numeric_vector::complex::Complex::new(real, imag);
                $name {
                    data: [item;$N],
                }
            }
        }

        impl numeric_vector::trait_definitions::Len for $name {
            /// Returns the length of the vector.
            fn len( &self ) -> usize {
                return $N;
            }
        }
        
        impl numeric_vector::trait_definitions::VectorIndexingComplex for $name {
            /// Returns indexed item of the vector.
            /// Index Clips at N-1.
            fn at( &self, index:usize) -> numeric_vector::complex::Complex {
                if( $N <= index)
                {
                    return self.data[$N - 1];
                }
                return self.data[index];
            }
            /// Returns the first item of the vector.
            fn front( &self ) -> numeric_vector::complex::Complex {
                return self.data[0];
            }
            /// Returns the last item of the vector.
            fn back( &self ) -> numeric_vector::complex::Complex {
                return self.data[$N-1];
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = numeric_vector::complex::Complex;
            /// Trait for returning an indexed value of the vector.
            #[inline]
            fn index(&self, index: usize) -> &numeric_vector::complex::Complex {
                return &self.data[index];
            }
        }
        
        impl core::ops::IndexMut<usize> for $name {
            /// Trait for returning a mutable reference to indexed item.
            /// ```
            /// use crate as numeric_vector;
            /// use numeric_vector::trait_definitions::*;
            /// declare_vector_real!( Vec8, 8);
            /// let mut x = Vec8::ramp(0,22);
            /// x[2] = 56;
            /// assert_eq!{x[2], 56i32 };
            /// ```
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut numeric_vector::complex::Complex {
                return &mut self.data[index];
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
        declare_vector_complex!( CVec4, 4 );
        let x = CVec4::new( 1, 2 );
        assert_eq!{ x[1], numeric_vector::complex::Complex{re:1, im:2} };
    }
}
