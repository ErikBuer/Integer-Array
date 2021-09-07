//use super::*;

/// Create array type of size N.
#[macro_export]
macro_rules! declare_array_complex{
    ( $name:ident, $N:expr) => {

        #[derive(Clone, Debug, PartialEq)]
        /// Real numeric array of type int32.
        pub struct $name{
            pub data: [numeric_array::complex::Complex; $N],
        }

        impl numeric_array::trait_definitions::NewComplex for $name {
            /// Generate an array of a value.
            fn new( real:i32, imag:i32 ) -> Self {
                let item =  numeric_array::complex::Complex::new(real, imag);
                $name {
                    data: [item;$N],
                }
            }
        }

        impl numeric_array::trait_definitions::Len for $name {
            /// Returns the length of the array.
            fn len( &self ) -> usize {
                return $N;
            }
        }
        
        impl numeric_array::trait_definitions::ArrayIndexingComplex for $name {
            /// Returns indexed item of the array.
            /// Index Clips at N-1.
            fn at( &self, index:usize) -> numeric_array::complex::Complex {
                if( $N <= index)
                {
                    return self.data[$N - 1];
                }
                return self.data[index];
            }
            /// Returns the first item of the array.
            fn front( &self ) -> numeric_array::complex::Complex {
                return self.data[0];
            }
            /// Returns the last item of the array.
            fn back( &self ) -> numeric_array::complex::Complex {
                return self.data[$N-1];
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = numeric_array::complex::Complex;
            /// Trait for returning an indexed value of the array.
            #[inline]
            fn index(&self, index: usize) -> &numeric_array::complex::Complex {
                return &self.data[index];
            }
        }
        
        impl core::ops::IndexMut<usize> for $name {
            /// Trait for returning a mutable reference to indexed item.
            /// ```
            /// use crate as numeric_array;
            /// use numeric_array::trait_definitions::*;
            /// declare_array_real!( Vec8, 8);
            /// let mut x = Vec8::ramp(0,22);
            /// x[2] = 56;
            /// assert_eq!{x[2], 56i32 };
            /// ```
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut numeric_array::complex::Complex {
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
        use crate as numeric_array;
        use numeric_array::trait_definitions::*;
        declare_array_complex!( CVec4, 4 );
        let x = CVec4::new( 1, 2 );
        assert_eq!{ x[1], numeric_array::complex::Complex{re:1, im:2} };
    }
}
