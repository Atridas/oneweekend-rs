use math::Float;
use math::Interval;

use math::Point3;
use math::Ray;
use math::Vector3;

use crate::Material;

pub struct HitRecord<'a, T, U> {
    pub point: Point3<T>,
    pub normal: Vector3<T>,
    pub material: &'a dyn Material<T, U>,
    pub t: T,
    pub front_face: bool,
}

pub trait Hittable<T, U> {
    fn hit(&self, ray: &Ray<T>, ray_t: Interval<T>) -> Option<HitRecord<T, U>>;
}

impl<'a, 'b, T, U> HitRecord<'a, T, U>
where
    T: Float,
    U: Float,
{
    pub fn new(
        ray: &Ray<T>,
        point: Point3<T>,
        outward_normal: Vector3<T>,
        material: &'a dyn Material<T, U>,
        t: T,
    ) -> HitRecord<'a, T, U> {
        let zero = T::constant(0.0);
        let front_face = ray.direction().dot(outward_normal) < zero;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

impl<T, U> Hittable<T, U> for &[Box<dyn Hittable<T, U>>]
where
    T: Copy,
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
