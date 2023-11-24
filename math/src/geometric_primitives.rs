use crate::floatops::Float;
use crate::Interval;

use super::HitRecord;
use super::Hittable;
use super::Point3;
use super::Ray;

#[derive(Clone, Copy)]
pub struct Sphere<T> {
    center: Point3<T>,
    radius: T,
}

impl<T> Sphere<T> {
    pub fn new(center: Point3<T>, radius: T) -> Sphere<T> {
        Sphere { center, radius }
    }
}

impl<T> Hittable<T> for Sphere<T>
where
    T: Float,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < T::constant(0.0) {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);

        Some(HitRecord::new(
            &ray,
            point,
            (point - self.center) / self.radius,
            root,
        ))
    }
}

pub enum GeometricPrimitive<T> {
    Sphere(Sphere<T>),
}

impl<T> Hittable<T> for GeometricPrimitive<T>
where
    Sphere<T>: Hittable<T>,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>> {
        match self {
            GeometricPrimitive::Sphere(s) => s.hit(ray, ray_t),
        }
    }
}

impl<T> Hittable<T> for &[GeometricPrimitive<T>]
where
    T: Copy,
    Sphere<T>: Hittable<T>,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>> {
        let mut result = None;
        let mut closest = ray_t.max;
        for object in self.iter() {
            let ray_t = Interval::new(ray_t.min, closest);
            if let Some(hit) = object.hit(ray, ray_t) {
                closest = hit.t;
                result = Some(hit);
            }
        }
        result
    }
}
