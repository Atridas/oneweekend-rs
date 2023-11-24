use super::floatops::Float;

struct Degree<T>(T);
struct Radian<T>(T);

impl<T> From<Degree<T>> for Radian<T>
where
    T: Float,
{
    fn from(d: Degree<T>) -> Radian<T> {
        T::to_radians()
    }
}

impl<T> From<Radian<T>> for Degree<T>
where
    T: Float,
{
    fn from(r: Radian<T>) -> Degree<T> {
        T::to_degrees()
    }
}
