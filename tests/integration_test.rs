use numeric_vector;
use numeric_vector::trait_definitions::*;


#[test]
fn create_type_real() {
    numeric_vector::declare_vector_real!( Vec11, 11);
    let mut x = Vec11::new(66);
    let y     = Vec11::new(2);
    x = x/y;
    assert_eq!(x.front(), 33);
}

#[test]
fn create_type_complex() {
    numeric_vector::declare_vector_complex!( CVec11, 11);
    let x = CVec11::new( 1, 2 );
    assert_eq!{ x[1], numeric_vector::complex::Complex{re:1, im:2} };

}