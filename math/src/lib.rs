mod colors;
mod floatops;
mod geometric_primitives;
mod hittable;
mod interval;
mod rays;
mod vectors;

pub use colors::*;
pub use geometric_primitives::*;
pub use hittable::*;
pub use interval::*;
pub use rays::*;
pub use vectors::*;

pub trait RandomSource<T> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> T;
    fn next_range(&mut self, min: T, max: T) -> T;
}
