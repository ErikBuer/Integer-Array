use integer_array;
use fixed::{types::extra::U18, FixedI32};

// Testing of real types
#[test]
fn create_type_real() {
    integer_array::declare_array_real!( Arr11, 11, FixedI32<U18> );

    let mut x = Arr11::new( FixedI32::<U18>::from_num(66) );
    let y     = Arr11::new( FixedI32::<U18>::from_num(2) );
    x = x/y;
    assert_eq!(x.front(), 33);
}

// Testing of complex types
#[test]
fn create_type_complex() {
    use fixed::{types::extra::U20, FixedI32};
    integer_array::declare_array_complex!( CArr11, Arr11, 11, FixedI32<U20>);
    let x = CArr11::new_from_i32( 1, 2 );
    assert_eq!{ x.as_array_f32()[0], num::complex::Complex{re:1.0, im:2.0} };

}
#[test]
fn real() {
    use fixed::{types::extra::U20, FixedI32};
    integer_array::declare_array_complex!( CArr11, Arr11, 11, FixedI32<U20>);
    let x = CArr11::new_from_i32( 5, 2 );
    assert_eq!{ x.real() , Arr11::new_from_i32(5) };

}