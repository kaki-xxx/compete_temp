use cargo_snippet::snippet;

use modint::{Mint, Com, comb};

#[snippet("_modint", prefix = "use modint::{Mint, Com, comb};")]
mod modint {
    use std::{fmt, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};
    use std::mem::swap;

    static mut MODULO: u64 = 1000000007;
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Mint(u64);

    impl Mint {
        #[inline]
        fn modulo() -> u64 {
            unsafe {
                MODULO
            }
        }

        pub fn set_modulo(n: u64) {
            unsafe {
                MODULO = n;
            }
        }

        #[inline]
        pub fn pow(self, power: u64) -> Self {
            if power == 0 {
                return Mint(1);
            }
            if power & 1 == 0 {
                let t = self.pow(power / 2);
                t * t
            } else {
                self * self.pow(power - 1)
            }
        }
        
        #[inline]
        pub fn inv(self) -> Self {
            let mut a = self.0 as i64;
            let (mut b, mut u, mut v) = (Self::modulo() as i64, 1, 0);
            while b != 0 {
                let t = a / b;
                a -= t * b; swap(&mut a, &mut b);
                u -= t * v; swap(&mut u, &mut v);
            }
            u %= Self::modulo() as i64;
            if u < 0 {
                u += Self::modulo() as i64;
            }
            Mint(u as u64)
        }
    }

    impl Add for Mint {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Self) -> Self {
            let mut ret = self;
            ret.0 += rhs.0;
            if ret.0 >= Self::modulo() {
                ret.0 -= Self::modulo();
            }
            ret
        }
    }

    impl Add<u64> for Mint {
        type Output = Self;
        #[inline]
        fn add(self, rhs: u64) -> Self {
            self + Mint(rhs)
        }
    }

    impl AddAssign for Mint {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl AddAssign<u64> for Mint {
        #[inline]
        fn add_assign(&mut self, rhs: u64) {
            *self = *self + rhs;
        }
    }

    impl Sub for Mint {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Self) -> Self {
            let mut ret = self;
            if ret.0 < rhs.0 {
                ret.0 += Self::modulo();
            }
            ret.0 -= rhs.0;
            ret
        }
    }

    impl Sub<u64> for Mint {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: u64) -> Self {
            self - Mint(rhs)
        }
    }

    impl SubAssign for Mint {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl SubAssign<u64> for Mint {
        #[inline]
        fn sub_assign(&mut self, rhs: u64) {
            *self = *self - rhs;
        }
    }

    impl Mul for Mint {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Mint) -> Self {
            let mut ret = self;
            ret.0 *= rhs.0;
            ret.0 %= Self::modulo();
            ret
        }
    }

    impl Mul<u64> for Mint {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: u64) -> Self {
            let mut ret = self;
            ret.0 *= rhs;
            ret.0 %= Self::modulo();
            ret
        }
    }

    impl MulAssign for Mint {
        #[inline]
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }

    impl MulAssign<u64> for Mint {
        #[inline]
        fn mul_assign(&mut self, rhs: u64) {
            *self = *self * rhs;
        }
    }

    impl Div for Mint {
        type Output = Mint;
        #[inline]
        fn div(self, rhs: Self) -> Self {
            let mut ret = self;
            ret.0 *= rhs.inv().0;
            ret.0 %= Self::modulo();
            ret
        }
    }

    impl Div<u64> for Mint {
        type Output = Mint;
        #[inline]
        fn div(self, rhs: u64) -> Self {
            let mut ret = self;
            ret.0 *= Mint(rhs).inv().0;
            ret.0 %= Self::modulo();
            ret
        }
    }

    impl DivAssign for Mint {
        #[inline]
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }

    impl DivAssign<u64> for Mint {
        #[inline]
        fn div_assign(&mut self, rhs: u64) {
            *self = *self / rhs;
        }
    }

    impl Neg for Mint {
        type Output = Mint;
        #[inline]
        fn neg(self) -> Self {
            Mint(0) - self
        }
    }

    impl fmt::Display for Mint {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub struct Com {
        len: usize,
        fac: Vec<Mint>,
        finv: Vec<Mint>,
    }
    
    impl Com {
        pub fn new(len: usize) -> Self {
            let mut fac = vec![Mint(0); len + 1];
            let mut finv = vec![Mint(0); len + 1];
            fac[0] = Mint(1);
            if len > 0 {
                fac[1] = Mint(1);
            }
            finv[0] = Mint(1);
            for i in 2..len + 1 {
                fac[i] = fac[i - 1] * Mint(i as u64);
            }
            finv[len] = fac[len].inv();
            for i in (2..len + 1).rev() {
                finv[i - 1] = finv[i] * Mint(i as u64);
            }
            Com { len, fac, finv }
        }

        #[inline]
        pub fn com(&self, n: usize, k: usize) -> Mint {
            if n < k {
                return Mint(0);
            }
            self.fac[n] * self.finv[k] * self.finv[n - k]
        }

        #[inline]
        pub fn fac(&self, n: usize) -> Mint {
            self.fac[n]
        }
    }

    pub fn comb(n: Mint, k: Mint) -> Mint {
        fn fact(n: Mint) -> Mint {
            let Mint(n) = n;
            (1..n+1).fold(Mint(1), |acc, i| acc * Mint(i))
        }
        let mut ret = Mint(1);
        let Mint(n) = n;
        let Mint(k) = k;
        for i in (0..n+1).rev().take(k as usize) {
            ret *= Mint(i);
        }
        ret /= fact(Mint(k));
        ret
    }
}
