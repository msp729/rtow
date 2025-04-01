use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub const I: Vec3 = Vec3(1.0, 0.0, 0.0);
pub const J: Vec3 = Vec3(0.0, 1.0, 0.0);
pub const K: Vec3 = Vec3(0.0, 0.0, 1.0);
pub const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);

#[inline]
const fn lift(mut f: impl FnMut(f64) -> f64) -> impl FnMut(Vec3) -> Vec3 {
    move |v| Vec3(f(v.0), f(v.1), f(v.2))
}

#[inline]
const fn lift1<F: Copy>(mut f: impl FnMut(f64, F) -> f64) -> impl FnMut(Vec3, F) -> Vec3 {
    move |v, x| Vec3(f(v.0, x), f(v.1, x), f(v.2, x))
}

#[inline]
const fn lift2(mut f: impl FnMut(f64, f64) -> f64) -> impl FnMut(Vec3, Vec3) -> Vec3 {
    move |v1, v2| Vec3(f(v1.0, v2.0), f(v1.1, v2.1), f(v1.2, v2.2))
}

#[inline]
const fn liftm<F: Copy>(mut f: impl FnMut(&mut f64, F)) -> impl FnMut(&mut Vec3, F) {
    move |v, x| {
        f(&mut v.0, x);
        f(&mut v.1, x);
        f(&mut v.2, x);
    }
}

#[inline]
const fn liftm2(mut f: impl FnMut(&mut f64, f64)) -> impl FnMut(&mut Vec3, Vec3) {
    move |v1, v2| {
        f(&mut v1.0, v2.0);
        f(&mut v1.1, v2.1);
        f(&mut v1.2, v2.2);
    }
}

macro_rules! sft {
    ($type:ty : $trait:ty, out $out:ty, $function:ident($($args:ident : $ts:ty),*), $rv:expr) => {
        impl $trait for $type {
            type Output = $out;
            fn $function(self, $($args:$ts),*) -> $out {
                $rv(self$(, $args)*)
            }
        }
    };
    ($type:ty : $trait:ty, mut $function:ident($($args:ident : $ts:ty),*), $rv:expr) => {
        impl $trait for $type {
            fn $function(&mut self, $($args:$ts),*) {
                $rv(self$(, $args)*)
            }
        }
    };
}

sft!(Vec3 : Add, out Self, add(rhs : Self), lift2(|x,y|x+y));
sft!(Vec3 : Sub, out Self, sub(rhs : Self), lift2(|x,y|x-y));
sft!(Vec3 : Neg, out Self, neg(), lift(|x| -x));
sft!(Vec3 : AddAssign, mut add_assign(rhs : Self), liftm2(|x,y| *x += y));
sft!(Vec3 : SubAssign, mut sub_assign(rhs : Self), liftm2(|x,y| *x -= y));
sft!(Vec3 : Mul<f64>, out Self, mul(rhs: f64), lift1(|x,y| x * y));
sft!(Vec3 : MulAssign<f64>, mut mul_assign(rhs: f64), liftm(|x,y| *x *= y));
sft!(Vec3 : Div<f64>, out Self, div(rhs: f64), lift1(|x,y| x / y));
sft!(Vec3 : DivAssign<f64>, mut div_assign(rhs: f64), liftm(|x,y| *x /= y));

impl Vec3 {
    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn parallel_normal(self, other: Vec3) -> (Vec3, Vec3) {
        let other = other.unit();
        let pmag = self.dot(other);
        let par = other * pmag;
        (par, self - par)
    }

    pub fn perpendicular(self, mut other: Vec3) -> Vec3 {
        other.normalize();
        other.cross(self.cross(other))
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "〈{}, {}, {}〉", self.0, self.1, self.2)
    }
}
