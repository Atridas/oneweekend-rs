use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul};

use crate::floatops::ToFixed;

#[derive(Clone, Copy)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGB<T> {
    pub fn new(r: T, g: T, b: T) -> RGB<T> {
        RGB { r, g, b }
    }
}

impl<T> RGB<T>
where
    T: ToFixed<u8>,
{
    pub fn to_byte_array(input: &[RGB<T>]) -> Vec<u8> {
        let mut result = Vec::with_capacity(input.len());
        for i in input {
            result.push(i.r.to_fixed());
            result.push(i.g.to_fixed());
            result.push(i.b.to_fixed());
        }
        result
    }
}

// overrides rgb[idx]
impl<T, Idx: Into<usize>> Index<Idx> for RGB<T> {
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
impl<T, Idx: Into<usize>> IndexMut<Idx> for RGB<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        match index.into() {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("index out of bounds"),
        }
    }
}

// overrides rgb * s
impl<T: Mul<Output = T> + Copy> Mul<T> for RGB<T> {
    type Output = RGB<T>;
    fn mul(self, rhs: T) -> RGB<T> {
        RGB::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

// overrides rgb / s
impl<T: Div<Output = T> + Copy> Div<T> for RGB<T> {
    type Output = RGB<T>;
    fn div(self, rhs: T) -> RGB<T> {
        RGB::new(self.r / rhs, self.g / rhs, self.b / rhs)
    }
}

// overrides rgb1 + rgb2
impl<T: Add<Output = T> + Copy> Add for RGB<T> {
    type Output = RGB<T>;
    fn add(self, rhs: RGB<T>) -> RGB<T> {
        RGB::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

// overrides rgb1 += rgb2
impl<T: AddAssign + Copy> AddAssign for RGB<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}
