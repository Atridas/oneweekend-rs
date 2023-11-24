use std::ops::{AddAssign, DivAssign, MulAssign};

pub trait Float:
    num::traits::Float
    + num::traits::FloatConst
    + num::traits::cast::FromPrimitive
    + AddAssign
    + MulAssign
    + DivAssign
{
    fn constant(c: f32) -> Self {
        <Self as num::traits::cast::FromPrimitive>::from_f32(c).unwrap()
    }
}

impl Float for f32 {}
impl Float for f64 {}
