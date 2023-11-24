use super::floatops::Float;

pub struct Degrees<T>(pub T);
pub struct Radians<T>(pub T);

impl<T> From<Degrees<T>> for Radians<T>
where
    T: Float,
{
    fn from(d: Degrees<T>) -> Radians<T> {
        Radians(d.0.to_radians())
    }
}

impl<T> From<Radians<T>> for Degrees<T>
where
    T: Float,
{
    fn from(r: Radians<T>) -> Degrees<T> {
        Degrees(r.0.to_degrees())
    }
}
