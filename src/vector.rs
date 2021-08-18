//use super::*;



/// Create vector type of size N and type T.
macro_rules! declare_type_real{
    ( $name:ident, $N:expr) => {
        /// Real numeric vector of type int32.
        pub struct $name{
            pub len: usize,
            pub data: [i32; $N],
        }

    };
}

declare_type_real!( scalar, 1);

#[cfg(test)]
mod tests {
    //use crate::*;
    use super::*;

    #[test]
    fn test_scalar_len() {
        let x = $name{ len:4, data: [0,0,0,0] };
        assert_eq!{x.len, 4};
    }
}

