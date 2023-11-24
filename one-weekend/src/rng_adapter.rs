use math::RandomSource;
use noise::RandomNumberGenerator;

pub struct RNGAdapter<'a>(pub &'a mut RandomNumberGenerator);

impl RandomSource<f32> for RNGAdapter<'_> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f32 {
        self.0.next_f32()
    }
    fn next_range(&mut self, min: f32, max: f32) -> f32 {
        self.0.next_range_f32(min, max)
    }
    fn next_bool_with_probability(&mut self, p: f32) -> bool {
        self.0.next_bool_with_probability(p)
    }
}

impl RandomSource<f64> for RNGAdapter<'_> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f64 {
        self.0.next_f64()
    }
    fn next_range(&mut self, min: f64, max: f64) -> f64 {
        self.0.next_range_f64(min, max)
    }
    fn next_bool_with_probability(&mut self, p: f64) -> bool {
        self.0.next_bool_with_probability(p as f32)
    }
}

pub struct DynAdapter<'a, T>(pub &'a mut dyn RandomSource<T>);

impl<T> RandomSource<T> for DynAdapter<'_, T> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> T {
        self.0.next()
    }
    fn next_range(&mut self, min: T, max: T) -> T {
        self.0.next_range(min, max)
    }
    fn next_bool_with_probability(&mut self, p: T) -> bool {
        self.0.next_bool_with_probability(p)
    }
}
