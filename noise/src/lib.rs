use external::squirrel_noise::*;
use math::RandomSource;

pub struct RandomNumberGenerator {
    pos: i32,
    seed: u32,
}

impl RandomNumberGenerator {
    pub fn new(seed: u32) -> RandomNumberGenerator {
        RandomNumberGenerator { pos: 0, seed }
    }

    pub fn next_int(&mut self) -> i32 {
        let x = self.pos;
        self.pos += 1;
        squirrel_noise5(x, self.seed)
    }

    pub fn next_float(&mut self) -> f32 {
        let x = self.pos;
        self.pos += 1;
        get1d_noise_zero_to_one(x, self.seed)
    }

    pub fn next_double(&mut self) -> f64 {
        let x = self.pos;
        self.pos += 1;
        get1d_noise_zero_to_one_f64(x, self.seed)
    }
}

impl RandomSource<f32> for RandomNumberGenerator {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f32 {
        self.next_float()
    }
    fn next_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_float() * (max - min)
    }
}
impl RandomSource<f64> for RandomNumberGenerator {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f64 {
        self.next_double()
    }
    fn next_range(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_double() * (max - min)
    }
}
