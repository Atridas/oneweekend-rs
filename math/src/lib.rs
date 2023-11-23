mod colors;
mod floatops;
mod rays;
mod vectors;

use std::ops::{Add, Div, Mul, Sub};

pub use colors::*;
use floatops::{Algebraic, LossyCast};
pub use rays::*;
pub use vectors::*;

pub fn ray_color<T, U>(ray: &Ray<T>) -> RGB<U>
where
    T: Algebraic,
    U: Copy,
    f32: LossyCast<U>,
    T: Div,
    U: Add<Output = U> + Sub<Output = U> + Div<Output = U> + Mul<Output = U>,
    T: LossyCast<U>,
{
    let unit_direction = Vector3::unit_vector(ray.direction());
    let a = ((unit_direction.y + T::one()) / T::two()).lossy_cast();
    RGB::new(1.0.lossy_cast(), 1.0.lossy_cast(), 1.0.lossy_cast()) * (1.0.lossy_cast() - a)
        + RGB::new(0.5.lossy_cast(), 0.7.lossy_cast(), 1.0.lossy_cast()) * a
}
