/// This macro implements a type which consists of an array of complex fixed-point numberts of size N.
/// Complete with the traits shown below.
/// 
/// ## Arguments
/// * `name`      - The name of the array type. E.g. CArr4
/// * `real_name` - The name of the real array type. E.g. Arr4
/// * `N`         - The length of the array. E.g 4.
/// * `T`         - The fixed type of the elements.
/// 
/// The complex support is currently under development.
/// 
/// # `::new`
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// use num::complex::Complex as C;
/// 
/// ia::declare_array_complex!( CArr4, Arr4, 4, FixedI32<U20> );
/// let x = CArr4::new_from_i32( 1, 2 );
/// assert_eq!{ x.as_array_f32(), [ C{re:1.0, im:2.0}, C{re:1.0, im:2.0}, C{re:1.0, im:2.0}, C{re:1.0, im:2.0} ]};
/// ```
/// 
/// /// # `::odd` and `::even`
/// Get the items in the odd or even indexes as an array.
/// 
/// See examples under the declare_array_real macro.
/// 
/// # `::real` and `::imag`
/// Get the real and imaginary array-components by running the `real()` and `imag()` traits.
/// The traits return a real integer-array of the same length.
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_complex!( CArr4, Arr4, 4, FixedI32<U20> );
/// let x = CArr4::new_from_f32( 1.0, 2.0 );
/// assert_eq!{ x.real(), Arr4::new_from_i32(1) };
/// assert_eq!{ x.imag(), Arr4::new_from_i32(2) };
/// ```
/// 
/// # `::mag`
/// Get the item-wise magnitude of the complex array.
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_complex!( CArr4, Arr4, 4, FixedI32<U20> );
/// let x = CArr4::new_from_f32( 1.0, 2.0 );
/// let y = x.mag();
/// assert_eq!{ y.as_array_f32(), [1.75, 1.75, 1.75, 1.75] };
/// ```
/// 
/// # `::arg`
/// Get the item-wise argumetn of the complex array.
/// 
/// ```rust
/// use integer_array as ia;
/// use ia::trait_definitions::*;
/// use fixed::{types::extra::U20, FixedI32};
/// 
/// ia::declare_array_complex!( CArr4, Arr4, 4, FixedI32<U20> );
/// let x = CArr4::new_from_f32( 1.0, 2.0 );
/// let y = x.arg();
/// assert_eq!{ y.as_array_f32(), [1.1032009, 1.1032009, 1.1032009, 1.1032009] };
/// ```
#[macro_export]
macro_rules! declare_array_complex{
    ( $name:ident, $real_name:ident, $N:expr, $T:ty ) => {

        // Declare the real array counterpart.
        integer_array::declare_array_real!($real_name, $N, $T);

        #[derive(Copy, Clone, Default, Debug, PartialEq)]
        /// Real numeric array of type int32.
        pub struct $name{
            pub data: [num::complex::Complex<$T>; $N],
        }

        impl $name {
            /// Generate an array of a value.
            #[allow(dead_code)]
            fn new( real:$T, imag:$T ) -> Self {
                let item =  num::complex::Complex::new(real, imag);
                $name {
                    data: [item;$N],
                }
            }
            #[allow(dead_code)]
            fn new_from_i32( real:i32, imag:i32 ) -> Self
            {
                let item =  num::complex::Complex::new(<$T>::from_num(real), <$T>::from_num(imag));
                $name {
                    data: [item;$N],
                }
            }
            #[allow(dead_code)]
            fn new_from_f32( real:f32, imag:f32 ) -> Self
            {
                let item =  num::complex::Complex::new(<$T>::from_num(real), <$T>::from_num(imag));
                $name {
                    data: [item;$N],
                }
            }
            #[allow(dead_code)]
            fn new_from_f64( real:f64, imag:f64 ) -> Self
            {
                let item =  num::complex::Complex::new(<$T>::from_num(real), <$T>::from_num(imag));
                $name {
                    data: [item;$N],
                }
            }
            /// Return self as a primitive array of floats. 
            #[allow(dead_code)]
            fn as_array_f32( &self ) -> [num::complex::Complex<f32>; $N]
            {
                let mut r_array: [num::complex::Complex<f32>; $N] = [num::complex::Complex::<f32>::new(0.0,0.0); $N];
                for n in 0..$N {
                    r_array[n].re = self[n].re.to_num::<f32>();
                    r_array[n].im = self[n].im.to_num::<f32>();
                }
                return r_array;
            }
            /// Return self as a primitive array of floats. 
            #[allow(dead_code)]
            fn as_array_i32( &self ) -> [num::complex::Complex<i32>; $N]
            {
                let mut r_array: [num::complex::Complex<i32>; $N] = [num::complex::Complex::<i32>::new(0,0); $N];
                for n in 0..$N {
                    r_array[n].re = self[n].re.to_num::<i32>();
                    r_array[n].im = self[n].im.to_num::<i32>();
                }
                return r_array;
            }
        }

        impl integer_array::trait_definitions::Len for $name {
            /// Returns the length of the array.
            fn len( &self ) -> usize {
                return $N;
            }
        }
        
        impl $name {
            /// Returns indexed item of the array.
            /// Index Clips at N-1.
            #[allow(dead_code)]
            fn at( &self, index:usize) -> num::complex::Complex<$T> {
                if( $N <= index)
                {
                    return self.data[$N - 1];
                }
                return self.data[index];
            }
            /// Returns the first item of the array.
            #[allow(dead_code)]
            fn front( &self ) -> num::complex::Complex<$T> {
                return self.data[0];
            }
            /// Returns the last item of the array.
            #[allow(dead_code)]
            fn back( &self ) -> num::complex::Complex<$T> {
                return self.data[$N-1];
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = num::complex::Complex<$T>;
            /// Trait for returning an indexed value of the array.
            #[inline]
            fn index(&self, index: usize) -> &num::complex::Complex<$T> {
                return &self.data[index];
            }
        }

        impl $name {
            /// Return the real component of the complex array
            #[allow(dead_code)]
            fn real( &self ) -> $real_name {
                let mut r_array = $real_name::new_from_i32(0);
                for n in 0..$N {
                    r_array[n] = self[n].re;
                }
                return r_array;
            }

            /// Return the imaginary component of the complex array
            #[allow(dead_code)]
            fn imag( &self ) -> $real_name {
                let mut r_array = $real_name::new_from_i32(0);
                for n in 0..$N {
                    r_array[n] = self[n].im;
                }
                return r_array;
            }

            /// Return the real component of the complex array
            #[allow(dead_code)]
            fn mag( &self ) -> $real_name {
                let mut r_array = $real_name::new_from_i32(0);
                for n in 0..$N {
                    let re_pow = integer_array::utility::fixed_powi( self[n].re, 2 );
                    let im_pow = integer_array::utility::fixed_powi( self[n].im, 2 );
                    r_array[n] = integer_array::utility::sqrt(re_pow+im_pow, <$T>::from_num(0.001) );
                }
                return r_array;
            }

            /// Return the item-wise argument of the complex array.
            #[allow(dead_code)]
            fn arg( &self ) -> $real_name {
                let mut r_array = $real_name::new_from_i32(0);
                for n in 0..$N {
                    r_array[n] = integer_array::utility::atan2_precise_fixed( self[n].im, self[n].re );
                }
                return r_array;
            }
        }

        impl $name {
            /// Trait for returning an array of the odd-indexed numbers in self.
            #[allow(dead_code)]
            fn odd(&self) -> [num::complex::Complex<$T>; $N/2] {
                let mut r_array = [num::complex::Complex::<$T>::new( <$T>::from_num(0), <$T>::from_num(0) ); $N/2];
                for n in 0..$N/2 {
                    r_array[n] = self[2*n+1];
                }
                return r_array;
            }

            /// Trait for returning an array of the even-indexed numbers in self.
            #[allow(dead_code)]
            fn even(&self) -> [num::complex::Complex<$T>; $N/2] {
                let mut r_array = [num::complex::Complex::<$T>::new( <$T>::from_num(0), <$T>::from_num(0) ); $N/2];
                for n in 0..$N/2 {
                    r_array[n] = self[2*n];
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
        use fixed::{types::extra::U18, FixedI32};
        use num::complex::Complex as C;

        declare_array_complex!( CArr4, Arr4, 4, FixedI32<U18> );
        let x = CArr4::new_from_i32( 1, 2 );
        assert_eq!{ x.as_array_f32(), [ C{re:1.0, im:2.0}, C{re:1.0, im:2.0}, C{re:1.0, im:2.0}, C{re:1.0, im:2.0} ]};
    }
    #[test]
    fn real() {
        use crate as integer_array;
        use fixed::{types::extra::U20, FixedI32};

        integer_array::declare_array_complex!( CArr4, Arr4, 4, FixedI32<U20> );
        let x = CArr4::new_from_i32( 1, 2 );
        assert_eq!{ x.real(), Arr4::new_from_i32(1) };
    }
}
