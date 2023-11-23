use external::squirrel_noise::*;

pub struct RandomGenerator {
    pos: i32,
    seed: u32,
}

impl RandomGenerator {
    pub fn new(seed: u32) -> RandomGenerator {
        RandomGenerator { pos: 0, seed }
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
