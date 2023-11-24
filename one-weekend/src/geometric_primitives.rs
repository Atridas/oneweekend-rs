use math::Float;
use math::Interval;
use math::Point3;
use math::Ray;

use crate::HitRecord;
use crate::Hittable;
use crate::Material;

#[derive(Clone, Copy)]
pub struct Sphere<'a, T, U> {
    center: Point3<T>,
    radius: T,
    material: &'a dyn Material<T, U>,
}

impl<'a, T, U> Sphere<'a, T, U> {
    pub fn new(center: Point3<T>, radius: T, material: &'a dyn Material<T, U>) -> Sphere<'a, T, U> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a, T, U> Hittable<T, U> for Sphere<'a, T, U>
where
    T: Float,
    U: Float,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<'a, T, U>> {
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
            self.material,
            root,
        ))
    }
}

pub enum GeometricPrimitive<'a, T, U> {
    Sphere(Sphere<'a, T, U>),
    Other(Box<dyn Hittable<T, U>>),
}

impl<'a, T, U> Hittable<T, U> for GeometricPrimitive<'a, T, U>
where
    Sphere<'a, T, U>: Hittable<T, U>,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T, U>> {
        match self {
            GeometricPrimitive::Sphere(s) => s.hit(ray, ray_t),
            GeometricPrimitive::Other(o) => o.hit(ray, ray_t),
        }
    }
}

impl<'a, T, U> Hittable<T, U> for &[GeometricPrimitive<'a, T, U>]
where
    T: Copy,
    Sphere<'a, T, U>: Hittable<T, U>,
{
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T, U>> {
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
