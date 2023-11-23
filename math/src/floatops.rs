pub trait HasSqrt {
    fn sqrt(&self) -> Self;
}

impl HasSqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl HasSqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}
