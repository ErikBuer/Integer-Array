//use super::*;



/// Create vector type of size N and type T.
macro_rules! declare_type{
    ( $name:ident, $N:expr) => {
        /// Numeric vector of type T.
        /// # Examples
        /// ```
        /// use numeric_vector::vector::$name;
        /// let x = $name{ len:4, data: [0,0,0,0] };
        /// assert_eq!{x.len, 4};
        /// ```
        pub struct $name<T> {
            pub len: usize,
            pub data: [T; $N],
        }

    };
}



declare_type!( Vec4, 4);

/*
#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn cv_trait_add() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        let sum = &vec_obj+&vec_obj;
        assert_eq!( VecC32F::new(vec![ C32F!(4,0), C32F!(0,8), C32F!(-4,0) ]), sum );
    }
}
*/
