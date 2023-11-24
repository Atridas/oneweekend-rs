use math::*;
use noise::RandomNumberGenerator;
use num::traits::AsPrimitive;

use crate::{Hittable, RNGAdapter};

pub struct Camera<T> {
    center: Point3<T>,
    pixel00_loc: Point3<T>,
    pixel_delta_u: Vector3<T>,
    pixel_delta_v: Vector3<T>,
    defocus_disc_u: Vector3<T>,
    defocus_disc_v: Vector3<T>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl<T> Camera<T>
where
    T: Float + AsPrimitive<u32>,
    u32: AsPrimitive<T>,
{
    pub fn new(
        aspect_ratio: T,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: Degrees<T>,
        lookfrom: Point3<T>,
        lookat: Point3<T>,
        up: Vector3<T>,
        defocus_angle: Degrees<T>,
        focus_dist: T,
    ) -> Camera<T> {
        // Calculate the image height, and ensure that it's at least 1
        let mut image_height = (image_width.as_() / aspect_ratio).as_();
        if image_height < 1 {
            image_height = 1;
        }
        let image_height = image_height;

        // Determine viewport dimensions
        let theta = vfov.0.to_radians();
        let h = (theta * T::constant(0.5)).tan();
        let viewport_height = T::constant(2.0) * h * focus_dist;
        let viewport_width = viewport_height * (image_width.as_() / image_height.as_());

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let w = (lookfrom - lookat).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width.as_();
        let pixel_delta_v = viewport_v / image_height.as_();

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = lookfrom
            - w * focus_dist
            - viewport_u * T::constant(0.5)
            - viewport_v * T::constant(0.5);
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * T::constant(0.5);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle.0 * T::constant(0.5)).to_radians().tan();
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Camera {
            center: lookfrom,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disc_u,
            defocus_disc_v,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn get_image_width(&self) -> u32 {
        self.image_width
    }
    pub fn get_image_height(&self) -> u32 {
        self.image_height
    }

    pub fn render<U, World>(&self, world: &World, rng: &mut RandomNumberGenerator) -> Vec<RGB<U>>
    where
        T: AsPrimitive<U>,
        u32: AsPrimitive<U>,
        U: 'static + Float,
        World: Hittable<T, U>,
        for<'a> RNGAdapter<'a>: RandomSource<T>,
    {
        let mut data = Vec::with_capacity((self.image_width * self.image_height * 3) as usize);
        let mut rng = RNGAdapter(rng);

        for j in 0..self.image_height {
            eprint!(
                "\rScanlines remaining {}/{} ",
                self.image_height - j,
                self.image_height
            );
            for i in 0..self.image_width {
                let mut rgb = RGB::white();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j, &mut rng);

                    rgb += Camera::ray_color(&r, self.max_depth, world, &mut rng);
                }
                data.push(rgb / self.samples_per_pixel.as_());
            }
        }
        data
    }

    fn ray_color<U, World, RNG>(ray: &Ray<T>, depth: u32, world: &World, rng: &mut RNG) -> RGB<U>
    where
        T: AsPrimitive<U>,
        U: 'static + Float,
        World: Hittable<T, U>,
        RNG: RandomSource<T>,
    {
        assert!(ray.direction().is_unit_vector());
        if depth <= 0 {
            return RGB::black();
        }
        match world.hit(ray, Interval::new(T::constant(0.001), T::infinity())) {
            Some(hit_record) => match hit_record.material.scatter(rng, ray, &hit_record) {
                Some((scattered_ray, attenuation)) => {
                    attenuation * Self::ray_color(&scattered_ray, depth - 1, world, rng)
                }
                None => RGB::black(),
            },
            None => {
                let unit_direction = Vector3::unit_vector(ray.direction());
                let a = ((unit_direction.y + T::constant(1.0)) * T::constant(0.5)).as_();
                RGB::white() * (U::constant(1.0) - a)
                    + RGB::new(U::constant(0.5), U::constant(0.7), U::constant(1.0)) * a
            }
        }
    }

    fn pixel_sample_square<RNG: RandomSource<T>>(&self, rng: &mut RNG) -> Vector3<T> {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = T::constant(-0.5) + rng.next();
        let py = T::constant(-0.5) + rng.next();
        return (self.pixel_delta_u * px) + (self.pixel_delta_v * py);
    }

    fn get_ray<RNG: RandomSource<T>>(&self, i: u32, j: u32, rng: &mut RNG) -> Ray<T> {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.

        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i.as_()) + (self.pixel_delta_v * j.as_());
        let pixel_sample = pixel_center + self.pixel_sample_square(rng);

        let ray_origin = self.defocus_disc_sample(rng);
        let ray_direction = (pixel_sample - ray_origin).unit_vector();

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disc_sample<RNG: RandomSource<T>>(&self, rng: &mut RNG) -> Point3<T> {
        // returns a random point in the camera defocus disk
        let p = Vector3::random_in_unit_disc(rng);
        self.center + self.defocus_disc_u * p.x + self.defocus_disc_v * p.y
    }
}
