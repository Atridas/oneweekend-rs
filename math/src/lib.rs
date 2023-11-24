mod angles;
mod colors;
mod floatops;
mod interval;
mod rays;
mod vectors;

pub use angles::*;
pub use colors::*;
pub use floatops::Float;
pub use interval::*;
pub use rays::*;
pub use vectors::*;

pub trait RandomSource<T> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> T;
    fn next_range(&mut self, min: T, max: T) -> T;
    fn next_bool_with_probability(&mut self, p: T) -> bool;
}
