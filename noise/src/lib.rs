use external::squirrel_noise::*;
use num::Float;

pub struct RandomNumberGenerator {
    pos: i32,
    seed: u32,
}

impl RandomNumberGenerator {
    pub fn new(seed: u32) -> RandomNumberGenerator {
        RandomNumberGenerator { pos: 0, seed }
    }

    pub fn next_bool_with_probability(&mut self, probability: f32) -> bool {
        self.next_float() < probability
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
