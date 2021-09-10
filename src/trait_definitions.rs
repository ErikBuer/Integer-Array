pub trait New {
    fn new( value:i32 ) -> Self;
}

pub trait NewComplex {
    fn new( real:i32, imag:i32 ) -> Self;
}

pub trait Ramp {
    fn ramp( start:i32, step:i32 ) -> Self;
}

pub trait Initializers {
    fn ones()       -> Self;
    fn zeros()      -> Self;
}

pub trait Len {
    fn len( &self ) -> usize;
}

pub trait ArrayIndexing {
    fn at( &self, index:usize) -> i32;
    fn front( &self )   -> i32;
    fn back( &self )    -> i32;
}

pub trait ArrayIndexingComplex {
    fn at( &self, index:usize) -> num::complex::Complex<i32>;
    fn front( &self )          -> num::complex::Complex<i32>;
    fn back( &self )           -> num::complex::Complex<i32>;
}

pub trait ArithmeticTraits {
    fn bias( &self, value:i32 )        -> Self;
    fn scale( &self, value:i32 )       -> Self;
    fn scale_float( &self, value:f32 ) -> Self;
}

pub trait SquareRoot {
    fn sqrt( &self )                   -> Self;
}

pub trait Pow {
    fn powi( &self, power:u32 ) -> Self;
}

pub trait TrigonometryTraits {
    fn sin( &self, norm_pi:i32, norm:i32 )  -> Self;
    fn cos( &self, norm_pi:i32, norm:i32 )  -> Self;
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

pub trait StdUtilities {
    fn todo( &self ) -> Self;
}

/* // TODO
pub trait ComplexExponential {
    fn exp( &self, norm_pi:i32, norm:i32 )  -> Self;
}

*/
/*
pub trait Complexutilities {
    fn real() -> Self;
    fn imag() -> Self;
    fn mag()  -> Self;
    fn ang()  -> Self;
}
*/

pub trait ComplexCartesian {
    fn real() -> Self;
    fn imag() -> Self;
}


/*
#[macro_export]
macro_rules! declare_complex_traits{
    ( $name:ident, $r_name:ident, $N:expr) => {
        pub trait ComplexCartesian {
            fn real() -> Self;
            fn imag() -> Self;
        }

        pub trait ComplexPolar {
            fn real() -> Self;
            fn imag() -> Self;
        }
    }
}
*/

