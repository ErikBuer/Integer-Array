pub trait VectorTraits {
    fn new( value:i32 ) -> Self;
    fn ones()   -> Self;
    fn zeros()  -> Self;
    fn ramp( start:i32, step:i32  )  -> Self;
    fn at( &self, index:usize) -> i32;
    fn front( &self )   -> i32;
    fn back( &self )    -> i32;
    fn len( &self )     -> usize;
}

pub trait ArithmeticTraits {
    fn bias( &self, value:i32 )  -> Self;
    fn scale( &self, value:i32 ) -> Self;
    fn scale_float( &self, value:f32 ) -> Self;
    fn sqrt( &self )                   -> Self;
}

pub trait TrigonometryTraits {
    fn sin( &self, norm_pi:i32, norm:i32 )  -> Self;
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
