use core::cmp::PartialOrd;
use std::ops::{Add, Mul, Neg};

use crate::floatops::Constants;
use crate::Interval;

use super::Point3;
use super::Ray;
use super::Vector3;

pub struct HitRecord<T> {
    pub point: Point3<T>,
    pub normal: Vector3<T>,
    pub t: T,
    pub front_face: bool,
}

pub trait Hittable<T> {
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T>>;
}

impl<T> HitRecord<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Neg<Output = T> + PartialOrd + Constants,
{
    pub fn new(ray: &Ray<T>, point: Point3<T>, outward_normal: Vector3<T>, t: T) -> HitRecord<T> {
        let zero = <T as Constants>::zero();
        let front_face = ray.direction().dot(outward_normal) < zero;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}

impl<T> Hittable<T> for &[Box<dyn Hittable<T>>]
where
    T: Copy,
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
