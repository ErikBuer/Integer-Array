/// Rase integer to an integer-valued power.
/// base^power.
pub fn powi( base:i32, power:u32 ) -> i32 {
    let mut temp:i32 = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}

/// Rase float number to an integer-valued power.
/// base^power.
pub fn fpowi( base:f32, power:u32 ) -> f32 {
    let mut temp:f32 = base;
    for _i in 0..power-1 {
        temp = temp*base;
    }
    return temp;
}

/// Absolute value.
pub fn abs( sample:i32 ) -> i32 {
    return sample.abs();
}

#[cfg(test)]
mod tests {
    #[test]
    fn powi() {
        assert_eq!{super::powi(3,2), 9};
    }
    #[test]
    fn fpowi() {
        assert_eq!{super::fpowi(3.0,2), 9.0};
    }
}