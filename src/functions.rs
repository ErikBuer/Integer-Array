use crate::trait_definitions as td;
use core::ops as ops;


/*
/// Element-wise addition of two vectors of equal or unequal size.
/// The returned vector is of the same size as the first argument.
pub fn add< V1, V2 >( vec1:V1, vec2:V2 ) -> V1
            where V1: td::VectorTraits + ops::Index<usize> + ops::IndexMut<usize>, 
                  V2: td::VectorTraits + ops::Index<usize>
{
    let mut min_len = vec1.len();
    // Determine length of output
    if vec2.len() < vec1.len() {
        min_len = vec2.len();
    }
    let mut r_vector = V1::zeros();
    
    for i in 0..min_len {
        r_vector[i] = vec1[i]+vec2[i];
    }
    return r_vector;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn func_add() {
        use crate as nv;
        use nv::trait_definitions::*;
        nv::declare_type_real!( Vec6, 6);
        nv::declare_type_real!( Vec4, 4);
        let x = Vec6::new(5);
        let y = Vec4::new(3);
        let z = nv::functions::add(x, y);
        assert_eq!{z.len(), x.len()};
        assert_eq!{z.data, [11,11,11,8,8,8] };
    }
}
*/