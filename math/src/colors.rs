use std::ops::{Index, IndexMut, Mul};

#[derive(Clone, Copy)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGB<T>
where
    T: Copy,
    T: Mul<Output = T>,
    T: From<f64>,
    T: Into<u8>,
{
    pub fn to_byte_array(input: &[RGB<T>]) -> Vec<u8> {
        let mut result = Vec::with_capacity(input.len());
        for i in input {
            result.push((i.r * T::from(255.999)).into());
            result.push((i.g * T::from(255.999)).into());
            result.push((i.b * T::from(255.999)).into());
        }
        result
    }
}

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
