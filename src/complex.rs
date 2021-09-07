pub mod array;

/// Complex scalar.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Complex{
    pub re: i32,
    pub im: i32,
}

impl crate::trait_definitions::NewComplex for Complex {
    /// Generate an array of a value.
    fn new( real:i32, imag:i32 ) -> Self {
        Self{
            re: real,
            im: imag,
        }
    }
}

/* // TODO
impl crate::trait_definitions::ComplexExponential for Complex {

}
*/