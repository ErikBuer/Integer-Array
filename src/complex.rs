//#[macro_export]
pub mod vector;

/// Complex scalar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex{
    pub re: i32,
    pub im: i32,
}

impl crate::trait_definitions::NewComplex for Complex {
    /// Generate a vector of a value.
    fn new( real:i32, imag:i32 ) -> Self {
        Self{
            re: real,
            im: imag,
        }
    }
}