use math::RandomSource;
use noise::RandomNumberGenerator;

pub struct RNGAdapter<'a>(&'a mut RandomNumberGenerator);

impl<'a> RNGAdapter<'a> {
    pub fn new(rng: &'a mut RandomNumberGenerator) -> RNGAdapter<'a> {
        RNGAdapter(rng)
    }
}

impl RandomSource<f32> for RNGAdapter<'_> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f32 {
        self.0.next_f32()
    }
    fn next_range(&mut self, min: f32, max: f32) -> f32 {
        self.0.next_range_f32(min, max)
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
}
