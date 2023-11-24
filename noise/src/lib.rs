use external::squirrel_noise::*;

pub struct RandomNumberGenerator {
    pos: i32,
    seed: u32,
}

impl RandomNumberGenerator {
    pub fn new(seed: u32) -> RandomNumberGenerator {
        RandomNumberGenerator { pos: 0, seed }
    }

    pub fn next_bool_with_probability(&mut self, probability: f32) -> bool {
        self.next_f32() < probability
    }

    pub fn next_i32(&mut self) -> i32 {
        let x = self.pos;
        self.pos += 1;
        squirrel_noise5(x, self.seed)
    }

    pub fn next_f32(&mut self) -> f32 {
        let x = self.pos;
        self.pos += 1;
        get1d_noise_zero_to_one_f32(x, self.seed)
    }

    pub fn next_f64(&mut self) -> f64 {
        let x = self.pos;
        self.pos += 1;
        get1d_noise_zero_to_one_f64(x, self.seed)
    }

    pub fn next_range_f32(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }

    pub fn next_range_f64(&mut self, min: f64, max: f64) -> f64 {
        min + self.next_f64() * (max - min)
    }
}
