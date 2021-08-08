//use super::*;


/// Numeric vector of type T.
/// # Examples
/// ```
/// use numeric_vector::vector::NVector;
/// let x = NVector::<u32>{ len:4, data: [0,0,0,0] };
/// assert_eq!{x.len, 4};
/// ```
pub struct NVector<T> {
    pub len: usize,
    pub data: [T; 4],
}
