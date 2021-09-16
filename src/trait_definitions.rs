pub trait New {
    fn new<T>( value:T ) -> Self;
}

pub trait NewComplex {
    fn new<T>( real:T, imag:T ) -> Self;
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

pub trait Atan {
    fn atan( &self )  -> Self;
}

pub trait WrapPhase {
    fn wrap_phase( &self )  -> Self;
}

pub trait DFT {
    fn dft( &self )  -> Self;
}

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

