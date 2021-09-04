use numeric_vector as nv;
use nv::trait_definitions::*;


#[test]
fn test_create_type() {
    nv::declare_type_real!( Vec11, 11);
    let test_vec = Vec11::new(77);
    assert_eq!(test_vec.front(), 77);
}