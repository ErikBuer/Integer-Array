use fixed::traits::Fixed;

/// Rase integer to an integer-valued power.
/// base^power.
pub fn powi( base:i32, power:usize ) -> i32 {
    let mut temp:i32 = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}

/// Rase float number to an integer-valued power.
/// - `base^power`.
pub fn fpowi<T>( base:T, power:usize ) -> T
    where T: num::traits::float::FloatCore
{
    let mut temp:T = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}

/// Rase fixed number to an integer-valued power.
/// - `base^power`.
pub fn fixed_powi<T>( base:T, power:usize ) -> T
    where T: fixed::traits::Fixed
{
    let mut temp:T = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}

/// Numerical square root of an integer.
pub fn sqrt( item:i32 ) -> i32 {
    // Initial approximation
    let mut root:i32 = item/2;
    let mut y:i32 = 1;
    // Accuracy level
    let error:i32 = 1;
    while  error <= root - y
    {
        root = (root + y) / 2;
        y = item / root;
    }
    return root;
}

/// Calculate atan(y/x) using a polynomial approximation.
/// Utilizes the following polynomial to estimate the angle θ \[radians\].
/// 
/// `atan(y,x) = ((y,x)+0.372003(y,x)^3) / (1+0.703384(y/x)^2 + 0.043562(y/x)^4)`
/// 
/// The method is accurat within 0.003 degrees when |θ|<=π/4.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Etition, IEEE Press, 2012.
/// 
/// # Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// # Example
/// 
/// ```
/// use integer_array::utility as util;
/// assert_eq!{util::atan_precise_float( 0.6f32, 0.4f32 ), 0.98300606 };
/// ``` 
pub fn atan_precise_float<T>( y:T, x:T ) -> T
    where T: num::traits::float::FloatCore
{
    let division:T = y/x;
    return ( (division) + T::from( 0.372003 ).unwrap()*fpowi(division, 3) ) 
                        / ( T::from( 1 ).unwrap() + T::from( 0.703384 ).unwrap()*fpowi(division, 2) + T::from(0.043562).unwrap()*fpowi( division , 4) );
}


/// Calculate atan(y/x) using a polynomial approximation.
/// Utilizes the following polynomial to estimate the angle θ \[radians\].
/// 
/// `atan(y,x) = ((y,x)+0.372003(y,x)^3) / (1+0.703384(y/x)^2 + 0.043562(y/x)^4)`
/// 
/// The method is accurat within 0.003 degrees when |θ|<=π/4.
/// 
/// \[1\] R. G. Lyons, Streamlining Digital Signal Processing, Second Etition, IEEE Press, 2012.
/// 
/// # Arguments 
///
/// * `y` - Is the argument along the y or imaginary axis.
/// * `x` - Is the argument along the x or real axis.
/// 
/// # Example
/// 
/// ```
/// use fixed::{types::extra::U28, FixedI32};
/// use integer_array::utility as util;
/// let arg = util::atan_precise_fixed( FixedI32::<U28>::from_num(0.6), FixedI32::<U28>::from_num(0.4) );
/// assert_eq!{ arg, 0.983006064 };
/// ``` 
pub fn atan_precise_fixed<T>( y: T, x: T ) -> T
    where T: Fixed
{
    let division: T = y/x;
    return ( (division) + T::from_num(0.372003f32)*fixed_powi(division,3) ) 
                        / (T::from_num(1) + T::from_num(0.703384f32)*fixed_powi(division,2) + T::from_num(0.043562f32)*fixed_powi(division,4) );
}


#[cfg(test)]
mod tests {
    #[test]
    fn powi() {
        assert_eq!{super::powi(3,2), 9};
    }
    #[test]
    fn fpowi() {
        assert_eq!{super::fpowi(3.0,2), 9.0};
    }
    #[test]
    fn sqrt() {
        assert_eq!{super::sqrt(100), 10};
    }
}