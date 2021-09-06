use numeric_vector;
use numeric_vector::trait_definitions::*;


#[test]
fn create_type_real() {
    numeric_vector::declare_vector_real!( Vec11, 11);
    let test_vec = Vec11::new(77);
    assert_eq!(test_vec.front(), 77);
}

#[test]
fn create_type_complex() {
    numeric_vector::declare_vector_complex!( CVec11, 11);
    let x = CVec11::new( 1, 2 );
    assert_eq!{ x[1], numeric_vector::complex::Complex{re:1, im:2} };

}