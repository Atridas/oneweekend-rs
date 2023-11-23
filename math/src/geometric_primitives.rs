use core::cmp::PartialOrd;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::floatops::Constants;
use crate::floatops::HasSqrt;
use crate::Interval;

use super::HitRecord;
use super::Hittable;
use super::Point3;
use super::Ray;
use super::Vector3;

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
    T: Copy
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Sub<Output = T>
        + PartialOrd
        + HasSqrt
        + Constants,
    Point3<T>: Sub<Output = Vector3<T>>,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < <T as Constants>::zero() {
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
