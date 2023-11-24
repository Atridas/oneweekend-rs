use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul};

use crate::{floatops::Float, RandomSource};

#[derive(Clone, Copy)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGB<T>
where
    T: Float,
{
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB { r, g, b }
    }

    pub fn random<RNG: RandomSource<T>>(rng: &mut RNG) -> Self {
        Self {
            r: rng.next(),
            g: rng.next(),
            b: rng.next(),
        }
    }

    pub fn random_range<RNG: RandomSource<T>>(rng: &mut RNG, min: T, max: T) -> Self {
        Self {
            r: rng.next_range(min, max),
            g: rng.next_range(min, max),
            b: rng.next_range(min, max),
        }
    }

    pub fn white() -> Self {
        Self::new(T::constant(1.0), T::constant(1.0), T::constant(1.0))
    }

    pub fn black() -> Self {
        Self::new(T::constant(0.0), T::constant(0.0), T::constant(0.0))
    }
}

impl<T> RGB<T>
where
    T: Float + num::traits::cast::AsPrimitive<u8>,
{
    pub fn to_srgb_array(input: &[RGB<T>]) -> Vec<u8> {
        let mut result = Vec::with_capacity(input.len());
        for i in input {
            result.push((i.r.powf(T::constant(1.0 / 2.2)) * T::constant(255.0)).as_());
            result.push((i.g.powf(T::constant(1.0 / 2.2)) * T::constant(255.0)).as_());
            result.push((i.b.powf(T::constant(1.0 / 2.2)) * T::constant(255.0)).as_());
        }
        result
    }
}

// overrides rgb[idx]
impl<T, Idx: Into<i64>> Index<Idx> for RGB<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        match index.into() {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("index out of bounds"),
        }
    }
}

// overrides rgb[idx]
impl<T, Idx: Into<i64>> IndexMut<Idx> for RGB<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        match index.into() {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("index out of bounds"),
        }
    }
}

// overrides rgb1 * rgb2
impl<T: Float> Mul for RGB<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

// overrides rgb * s
impl<T: Float> Mul<T> for RGB<T> {
    type Output = RGB<T>;
    fn mul(self, rhs: T) -> RGB<T> {
        RGB::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

// overrides rgb / s
impl<T: Float> Div<T> for RGB<T> {
    type Output = RGB<T>;
    fn div(self, rhs: T) -> RGB<T> {
        RGB::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

// overrides rgb1 + rgb2
impl<T: Float> Add for RGB<T> {
    type Output = RGB<T>;
    fn add(self, rhs: RGB<T>) -> RGB<T> {
        RGB::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

// overrides rgb1 += rgb2
impl<T: Float> AddAssign for RGB<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
