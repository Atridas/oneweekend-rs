use math::{Float, RandomSource, Ray, Vector3, RGB};
use noise::RandomNumberGenerator;

use crate::{HitRecord, RNGAdapter};

pub trait Material<T, U>
where
    T: Float,
    U: Float,
{
    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        ray: &Ray<T>,
        hit_record: &HitRecord<T, U>,
    ) -> Option<(Ray<T>, RGB<U>)>;
}

pub struct Lambertian<U>(RGB<U>);

impl<U> Lambertian<U> {
    pub fn new(albedo: RGB<U>) -> Self {
        Self(albedo)
    }
}

impl<T, U> Material<T, U> for Lambertian<U>
where
    T: Float,
    U: Float,
    for<'a> RNGAdapter<'a>: RandomSource<T>,
{
    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        _: &Ray<T>,
        hit_record: &HitRecord<T, U>,
    ) -> Option<(Ray<T>, RGB<U>)> {
        let mut direction = (hit_record.normal
            + Vector3::random_unit_vector(&mut RNGAdapter::new(rng)))
        .unit_vector();

        if direction.near_zero() {
            direction = hit_record.normal;
        }

        Some((Ray::new(hit_record.point, direction), self.0))
    }
}
