/// Create a complex i32 array type of size N.
/// Complete with traits.
/// 
/// # Traits implemented with the macro:
/// The complex support is currently under development.
/// 
/// ### Generate an array of a specific value.
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// ia::declare_array_complex!( CArr4, Arr4, 4 );
/// let x = CArr4::new( 1, 2 );
/// assert_eq!{ x[1], num::complex::Complex{re:1, im:2} };
/// ```
/// 
/// ### Real and imaginary component.
/// Get the real and imaginary array-components by running the `real()` and `imag()` traits.
/// The traits return a real integer-array of the same length.
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// ia::declare_array_complex!( CArr4, Arr4, 4 );
/// let x = CArr4::new( 1, 2 );
/// assert_eq!{ x.real(), Arr4::new(1) };
/// assert_eq!{ x.imag(), Arr4::new(2) };
/// ```
/// 
/// ### Magnitude.
/// Get the item-wies magnitude of the complex array.
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// ia::declare_array_complex!( CArr4, Arr4, 4 );
/// let x = CArr4::new( 100, 200 );
/// assert_eq!{ x.mag(), Arr4::new(223) };
/// ```
#[macro_export]
macro_rules! declare_array_complex{
    ( $name:ident, $r_name:ident, $N:expr) => {

        // Declare the real array counterpart.
        integer_array::declare_array_real!($r_name, $N);

        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        /// Real numeric array of type int32.
        pub struct $name{
            pub data: [num::complex::Complex<i32>; $N],
        }

        impl integer_array::trait_definitions::NewComplex for $name {
            /// Generate an array of a value.
            fn new( real:i32, imag:i32 ) -> Self {
                let item =  num::complex::Complex::new(real, imag);
                $name {
                    data: [item;$N],
                }
            }
        }

        impl integer_array::trait_definitions::Len for $name {
            /// Returns the length of the array.
            fn len( &self ) -> usize {
                return $N;
            }
        }
        
        impl integer_array::trait_definitions::ArrayIndexingComplex for $name {
            /// Returns indexed item of the array.
            /// Index Clips at N-1.
            fn at( &self, index:usize) -> num::complex::Complex<i32> {
                if( $N <= index)
                {
                    return self.data[$N - 1];
                }
                return self.data[index];
            }
            /// Returns the first item of the array.
            fn front( &self ) -> num::complex::Complex<i32> {
                return self.data[0];
            }
            /// Returns the last item of the array.
            fn back( &self ) -> num::complex::Complex<i32> {
                return self.data[$N-1];
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = num::complex::Complex<i32>;
            /// Trait for returning an indexed value of the array.
            #[inline]
            fn index(&self, index: usize) -> &num::complex::Complex<i32> {
                return &self.data[index];
            }
        }
        
        impl core::ops::IndexMut<usize> for $name {
            /// Trait for returning a mutable reference to indexed item.
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut num::complex::Complex<i32> {
                return &mut self.data[index];
            }
        }

        impl $name {
            /// Return the real component of the complex array
            #[allow(dead_code)]
            fn real( &self ) -> $r_name {
                let mut r_array = $r_name::zeros();
                for n in 0..$N {
                    r_array[n] = self[n].re;
                }
                return r_array;
            }

            /// Return the imaginary component of the complex array
            #[allow(dead_code)]
            fn imag( &self ) -> $r_name {
                let mut r_array = $r_name::zeros();
                for n in 0..$N {
                    r_array[n] = self[n].im;
                }
                return r_array;
            }

            /// Return the real component of the complex array
            #[allow(dead_code)]
            fn mag( &self ) -> $r_name {
                let mut r_array = $r_name::zeros();
                for n in 0..$N {
                    let re_pow = integer_array::utility::powi( self[n].re, 2 );
                    let im_pow = integer_array::utility::powi( self[n].im, 2 );
                    r_array[n] = integer_array::utility::sqrt(re_pow+im_pow);
                }
                return r_array;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new() {
        use crate as integer_array;
        use integer_array::trait_definitions::*;
        declare_array_complex!( CArr4, Arr4, 4 );
        let x = CArr4::new( 1, 2 );
        assert_eq!{ x[1], num::complex::Complex{re:1, im:2} };
    }
    #[test]
    fn real() {
        use crate as integer_array;
        use integer_array::trait_definitions::*;
        declare_array_complex!( CArr4, Arr4, 4 );
        let x = CArr4::new( 1, 2 );
        assert_eq!{ x.real(), Arr4::new(1) };
    }
}
