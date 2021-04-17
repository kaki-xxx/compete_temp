use cargo_snippet::snippet;

use geo::Vec2D;

#[snippet("_geo", prefix = "use geo::Vec2D;")]
mod geo {
    use core::f64;
    use std::{fmt, u64};
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

    #[derive(Debug, Clone, Copy)]
    pub struct Vec2D<T> {
        pub x: T,
        pub y: T,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct LineSeg<T> {
        pub start: Vec2D<T>,
        pub end: Vec2D<T>,
    }

    impl<T> Vec2D<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    macro_rules! impl_ccw_int {
        ($($t:ty),+) => {
            $(
                impl Vec2D<$t> {
                    pub fn ccw(self, b: Vec2D<$t>, c: Vec2D<$t>) -> i32 {
                        if (b - self).det(c - self) > 0 {
                            1
                        } else if (b - self).det(c - self) < 0 {
                            -1
                        } else if (b - self).dot(c - self) < 0 {
                            2
                        } else if (b - self).norm() < (c - self).norm() {
                            -2
                        } else {
                            0
                        }
                    }
                }
            )+
        };
    }

    impl_ccw_int!(i32, i64);

    impl Vec2D<f64> {
        pub fn from_polar(r: f64, theta: f64) -> Self {
            Self {
                x: r * theta.cos(),
                y: r * theta.sin(),
            }
        }

        pub fn ccw(self, b: Vec2D<f64>, c: Vec2D<f64>) -> i32 {
            if (b - self).det(c - self) > 0. {
                1
            } else if (b - self).det(c - self) < 0. {
                -1
            } else if (b - self).dot(c - self) < 0. {
                2
            } else if (b - self).norm() < (c - self).norm() {
                -2
            } else {
                0
            }
        }
    }

    impl<T> Vec2D<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
    {
        #[inline]
        pub fn dot(&self, rhs: Self) -> T {
            self.x * rhs.x + self.y * rhs.y
        }

        #[inline]
        pub fn det(&self, rhs: Self) -> T {
            self.x * rhs.y - self.y * rhs.x
        }

        #[inline]
        pub fn norm(&self) -> T {
            self.x * self.x + self.y * self.y
        }
    }

    impl Vec2D<f64> {
        #[inline]
        pub fn abs(&self) -> f64 {
            self.x.hypot(self.y)
        }

        #[inline]
        pub fn rot(&self, theta: f64) -> Self {
            Self {
                x: self.x * theta.cos() - self.y * theta.sin(),
                y: self.x * theta.sin() + self.y * theta.cos(),
            }
        }

        #[inline]
        pub fn rot_around(&self, theta: f64, center: Vec2D<f64>) -> Self {
            let mut ret = self.clone();
            ret -= center;
            ret = ret.rot(theta);
            ret += center;
            ret
        }
    }

    impl<T: Add<Output = T>> Add for Vec2D<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T: Add<Output = T> + Copy> AddAssign for Vec2D<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl<T: Sub<Output = T>> Sub for Vec2D<T> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl<T: Sub<Output = T> + Copy> SubAssign for Vec2D<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl<T: Neg<Output = T>> Neg for Vec2D<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    macro_rules! impl_vec2d_scalar_mul {
        ($($t:ty),+) => {
            $(
                impl Mul<Vec2D<$t>> for $t {
                    type Output = Vec2D<$t>;
                    fn mul(self, rhs: Vec2D<$t>) -> Self::Output {
                        Vec2D {
                            x: rhs.x * self,
                            y: rhs.y * self,
                        }
                    }
                }
            )+
        };
    }

    impl_vec2d_scalar_mul!(i32, i64, u32, u64, f64);

    impl<T: fmt::Display> fmt::Display for Vec2D<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {}", self.x, self.y)
        }
    }

    impl LineSeg<f64> {
        fn distance(&self, v: Vec2D<f64>) -> f64 {
            let a = self.start - v;
            let b = self.end - v;
            a.det(b) / 2. / (self.end - self.start).abs()
        }
    }
}
