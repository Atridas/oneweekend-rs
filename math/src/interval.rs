use std::ops::Neg;

use super::floatops::Constants;

pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T> Interval<T> {
    pub fn new(min: T, max: T) -> Interval<T> {
        Interval { min, max }
    }
}

impl<T> Interval<T>
where
    T: Constants + Neg<Output = T>,
{
    pub fn empty() -> Interval<T> {
        Interval {
            min: T::infinity(),
            max: -T::infinity(),
        }
    }
    pub fn universe() -> Interval<T> {
        Interval {
            min: -T::infinity(),
            max: T::infinity(),
        }
    }
}

impl<T> Interval<T>
where
    T: Clone + PartialOrd,
{
    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }
}

impl<T> Default for Interval<T>
where
    T: Constants + Neg<Output = T>,
{
    fn default() -> Interval<T> {
        Interval {
            min: T::infinity(),
            max: -T::infinity(),
        }
    }
}
