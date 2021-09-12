pub trait New {
    fn new<T>( value:T ) -> Self;
}

pub trait NewComplex {
    fn new( real:i32, imag:i32 ) -> Self;
}

pub trait Ramp {
    fn ramp<T>( start:T, step:T ) -> Self;
}

pub trait Initializers {
    fn ones()       -> Self;
    fn zeros()      -> Self;
}

pub trait Len {
    fn len( &self ) -> usize;
}

pub trait Pow {
    fn powi( &self, power:u32 ) -> Self;
}

pub trait Sin {
    fn sin( &self )  -> Self;
}

pub trait Cos {
    fn cos( &self )  -> Self;
}

pub trait Tan {
    fn tan( &self )  -> Self;
}

pub trait WrapPhase {
    fn wrap_phase( &self )  -> Self;
}

/*
pub trait StatisticTraits {
    fn sum( &self )  -> i32;
    fn mean( &self ) -> i32;
    fn var( &self )  -> i32;
    fn max( &self )  -> i32;
    fn min( &self )  -> i32;
    fn argmax( &self ) -> usize;
    fn argmin( &self ) -> usize;
}
*/

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

