mod colors;
mod floatops;
mod geometric_primitives;
mod hittable;
mod interval;
mod rays;
mod vectors;

pub use colors::*;
pub use geometric_primitives::*;
pub use hittable::*;
pub use interval::*;
pub use rays::*;
pub use vectors::*;

pub fn ray_color<T: Hittable<f64>>(ray: &Ray<f64>, world: &T) -> RGB<f32> {
    match world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        Some(record) => {
            let normal_colour = RGB::new(
                record.normal.x as f32,
                record.normal.y as f32,
                record.normal.z as f32,
            );
            (normal_colour + RGB::new(1.0, 1.0, 1.0)) * 0.5
        }
        None => {
            let unit_direction = Vector3::unit_vector(ray.direction());
            let a = ((unit_direction.y + 1.0) * 0.5) as f32;
            RGB::new(1.0, 1.0, 1.0) * (1.0 - a) + RGB::new(0.5, 0.7, 1.0) * a
        }
    }
}
