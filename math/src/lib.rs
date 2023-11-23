mod colors;
mod floatops;
mod rays;
mod vectors;

pub use colors::*;
pub use rays::*;
pub use vectors::*;

pub fn ray_color(ray: &Ray<f64>) -> RGB<f32> {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return RGB::new(1.0, 0.0, 0.0);
    }

    let unit_direction = Vector3::unit_vector(ray.direction());
    let a = ((unit_direction.y + 1.0) * 0.5) as f32;
    RGB::new(1.0, 1.0, 1.0) * (1.0 - a) + RGB::new(0.5, 0.7, 1.0) * a
}

fn hit_sphere(center: Point3<f64>, radius: f64, r: &Ray<f64>) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction()) * 2.0;
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}
