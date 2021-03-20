use gcd_lcm::{gcd, lcm};

mod gcd_lcm {
    use std::ops::{Div, Mul, Rem};

    pub fn gcd<T>(v1: T, v2: T) -> T
    where
        T: Rem<Output=T>
        + Copy
        + Default
        + PartialEq    
    {
        let zero = T::default();
        match v2 {
            v2 if v2 == zero => v1,
            v2 => gcd(v2, v1 % v2)
        }
    }

    pub fn lcm<T>(v1: T, v2: T) -> T
    where
        T: Rem<Output=T>
        + Mul<Output=T>
        + Div<Output=T>
        + Copy
        + Default
        + PartialEq
    {
        v1 * v2 / gcd(v1, v2)
    }
}
