use math::{Float, RandomSource, Ray, Vector3, RGB};
use noise::RandomNumberGenerator;
use num::traits::AsPrimitive;

use crate::*;

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

pub struct Metal<U> {
    albedo: RGB<U>,
    fuzz: U,
}

impl<U> Metal<U> {
    pub fn new(albedo: RGB<U>, fuzz: U) -> Self {
        Self { albedo, fuzz }
    }
}

impl<T, U> Material<T, U> for Metal<U>
where
    T: Float,
    U: Float + Into<T>,
    for<'a> RNGAdapter<'a>: RandomSource<T>,
{
    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        ray: &Ray<T>,
        hit_record: &HitRecord<T, U>,
    ) -> Option<(Ray<T>, RGB<U>)> {
        let reflected = ray.direction().reflect(hit_record.normal);
        let scattered = (reflected
            + Vector3::random_unit_vector(&mut RNGAdapter::new(rng)) * self.fuzz.into())
        .unit_vector();
        if scattered.dot(hit_record.normal) > T::constant(0.0) {
            Some((Ray::new(hit_record.point, scattered), self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric<U>(U);

impl<U> Dielectric<U> {
    pub fn new(index_of_refraction: U) -> Self {
        Self(index_of_refraction)
    }

    fn reflectance<T: Float>(cosine: T, ref_idx: T) -> T {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (T::constant(1.0) - ref_idx) / (T::constant(1.0) + ref_idx);
        r0 = r0 * r0;
        r0 + (T::constant(1.0) - r0) * (T::constant(1.0) - cosine).powi(5)
    }
}

impl<T, U> Material<T, U> for Dielectric<U>
where
    T: Float + AsPrimitive<f32>,
    U: Float + Into<T>,
{
    fn scatter(
        &self,
        rng: &mut RandomNumberGenerator,
        ray: &Ray<T>,
        hit_record: &HitRecord<T, U>,
    ) -> Option<(Ray<T>, RGB<U>)> {
        let refraction_ratio = if hit_record.front_face {
            T::constant(1.0) / self.0.into()
        } else {
            self.0.into()
        };

        assert!(ray.direction().is_unit_vector());
        let cos_theta = -ray.direction().dot(hit_record.normal).min(T::constant(1.0));
        let sin_theta = (T::constant(1.0) - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > T::constant(1.0);

        let reflectance = Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || rng.next_bool_with_probability(reflectance.as_()) {
            ray.direction().reflect(hit_record.normal)
        } else {
            ray.direction().refract(hit_record.normal, refraction_ratio)
        }
        .unit_vector();

        Some((Ray::new(hit_record.point, direction), RGB::white()))
    }
}
