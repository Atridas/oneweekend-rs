use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

pub trait HasSqrt {
    fn sqrt(&self) -> Self;
}

impl HasSqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl HasSqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}

pub trait Constants {
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
    fn infinity() -> Self;
    fn pi() -> Self;
    fn pi_over_180() -> Self;
}

impl Constants for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn two() -> Self {
        2.0
    }
    fn infinity() -> Self {
        Self::INFINITY
    }
    fn pi() -> Self {
        3.1415926535897932385
    }
    fn pi_over_180() -> Self {
        3.1415926535897932385 / 180.0
    }
}

impl Constants for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn two() -> Self {
        2.0
    }
    fn infinity() -> Self {
        Self::INFINITY
    }
    fn pi() -> Self {
        3.1415926535897932385
    }
    fn pi_over_180() -> Self {
        3.1415926535897932385 / 180.0
    }
}

pub trait ToFixed<T> {
    fn to_fixed(&self) -> T;
}

impl ToFixed<u8> for f32 {
    fn to_fixed(&self) -> u8 {
        (*self * 255.999) as u8
    }
}
impl ToFixed<u8> for f64 {
    fn to_fixed(&self) -> u8 {
        (*self * 255.999) as u8
    }
}

pub trait LossyCast<T> {
    fn lossy_cast(&self) -> T;
}

impl LossyCast<f32> for f32 {
    fn lossy_cast(&self) -> f32 {
        *self
    }
}
impl LossyCast<f64> for f32 {
    fn lossy_cast(&self) -> f64 {
        *self as f64
    }
}

impl LossyCast<f32> for f64 {
    fn lossy_cast(&self) -> f32 {
        *self as f32
    }
}
impl LossyCast<f64> for f64 {
    fn lossy_cast(&self) -> f64 {
        *self
    }
}

pub trait Algebraic:
    Copy
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + MulAssign
    + DivAssign
    + Constants
    + HasSqrt
{
}

impl Algebraic for f32 {}
impl Algebraic for f64 {}
