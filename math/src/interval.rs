use crate::floatops::Float;

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
    T: Float,
{
    // creation

    pub fn empty() -> Interval<T> {
        Interval {
            min: T::infinity(),
            max: -T::infinity(),
        }
    }

    pub fn default() -> Interval<T> {
        Self::empty()
    }

    pub fn universe() -> Interval<T> {
        Interval {
            min: -T::infinity(),
            max: T::infinity(),
        }
    }

    // check functions

    /// checks if a value is inside the range, inclusive
    /// use "surrounds" for exclusive check
    pub fn contains(&self, x: T) -> bool {
        self.min <= x && x <= self.max
    }

    /// checks if a value is inside the range, exclusive
    /// use "contains" for inclusive check
    pub fn surrounds(&self, x: T) -> bool {
        self.min < x && x < self.max
    }

    /// clamps a value inside the interval
    pub fn clamp(&self, x: T) -> T {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
