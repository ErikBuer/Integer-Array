use numeric_vector;
use numeric_vector::trait_definitions::*;


#[test]
fn test_create_type() {
    numeric_vector::declare_type_real!( Vec11, 11);
    numeric_vector::declare_type_real!( Vec12, 12);
    let test_vec = Vec11::new(77);
    assert_eq!(test_vec.front(), 77);
}