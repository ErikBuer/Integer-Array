use integer_array;
use integer_array::trait_definitions::*;


#[test]
fn create_type_real() {
    integer_array::declare_array_real!( Vec11, 11);
    let mut x = Vec11::new(66);
    let y     = Vec11::new(2);
    x = x/y;
    assert_eq!(x.front(), 33);
}

#[test]
fn create_type_complex() {
    integer_array::declare_array_complex!( CVec11, 11);
    let x = CVec11::new( 1, 2 );
    assert_eq!{ x[1], num::complex::Complex{re:1, im:2} };

}