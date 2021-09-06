use super::*;

/// Create vector type of size N.
#[macro_export]
macro_rules! declare_vector_complex{
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
                    re: [real;$N],
                    im: [imag;$N],
                }
            }
        }

        impl numeric_vector::trait_definitions::VectorTraits for $name {
            /// Generate a vector of ones.
            fn ones() -> Self {
                $name {
                    re: [1;$N],
                    im: [0;$N],
                }
            }
            /// Generate a vector of zeroes.
            fn zeros() -> Self {
                $name {
                    re: [0;$N],
                    im: [0;$N],
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
            use numeric_vector::complex::Complex as Complex;
            fn at( &self, index:usize) -> Complex {
                if( $N <= index)
                {
                    return Complex{ re: self.re[$N-1], im: self.im[$N-1] };
                }
                return Complex{ re: self.re[index], im: self.im[index] };

            }
            /// Returns the first item of the vector.
            fn front( &self ) -> Complex {
                {
                    Complex{
                        re: self.re[0],
                        im: self.im[0],
                    };
                }
            }
            /// Returns the last item of the vector.
            fn back( &self ) -> Complex {
                {
                    Complex{
                        re: self.re[$N-1],
                        im: self.im[$N-1],
                    }
                }
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = numeric_vector::complex::Complex;
            /// Trait for returning an indexed value of the vector.
            #[inline]
            fn index(&self, index: usize) -> &numeric_vector::complex::Complex {
                {
                    Complex
                }
            }
        }
        
        impl core::ops::IndexMut<usize> for $name {
            /// Trait for returning a mutable reference to indexed item.
            /// ´´´
            /// use crate as numeric_vector;
            /// use numeric_vector::trait_definitions::*;
            /// declare_vector_real!( Vec8, 8);
            /// let mut x = Vec8::ramp(0,22);
            /// x[2] = 56;
            /// assert_eq!{x[2], 56i32 };
            /// ´´´
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
