use super::floatops::*;

struct Degree<T>(T);
struct Radian<T>(T);

impl<T> From<Degree<T>> for Radian<T> where T : Constants {
    fn from(d : Degree<T>) -> Radian<T> {
        Radian<T>(d.0 * T::pi_over_180())
    }
}

impl<T> From<Radian<T>> for Degree<T> where T : Constants {
    fn from(r : Radian<T>) -> Degree<T> {
        Degree<T>(r.0 / T::pi_over_180())
    }
}