use math::{Float, Ray, RGB};

use crate::HitRecord;

trait Material<T, U>
where
    T: Float,
    U: Float,
{
    fn scatter(
        &self,
        ray: &Ray<T>,
        hit_record: &HitRecord<T>,
        attenuation: RGB<U>,
    ) -> Option<Ray<T>>;
}
