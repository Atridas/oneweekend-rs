use math::*;
use noise::RandomNumberGenerator;

use crate::Hittable;

pub struct Camera {
    center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    defocus_disc_u: Vector3<f64>,
    defocus_disc_v: Vector3<f64>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: Degrees<f64>,
        lookfrom: Point3<f64>,
        lookat: Point3<f64>,
        up: Vector3<f64>,
        defocus_angle: Degrees<f64>,
        focus_dist: f64,
    ) -> Camera {
        // Calculate the image height, and ensure that it's at least 1
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        if image_height < 1 {
            image_height = 1;
        }
        let image_height = image_height;

        // Determine viewport dimensions
        let theta = vfov.0.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let w = (lookfrom - lookat).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = lookfrom - w * focus_dist - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (defocus_angle.0 * 0.5).to_radians().tan();
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

    pub fn render<T: Hittable<f64, f32>>(
        &self,
        world: &T,
        rng: &mut RandomNumberGenerator,
    ) -> Vec<RGB<f32>> {
        let mut data = Vec::with_capacity((self.image_width * self.image_height * 3) as usize);

        for j in 0..self.image_height {
            eprint!(
                "\rScanlines remaining {}/{} ",
                self.image_height - j,
                self.image_height
            );
            for i in 0..self.image_width {
                let mut rgb = RGB::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j, rng);

                    rgb += Camera::ray_color(&r, self.max_depth, world, rng);
                }
                data.push(rgb / self.samples_per_pixel as f32);
            }
        }
        data
    }

    fn ray_color<T: Hittable<f64, f32>>(
        ray: &Ray<f64>,
        depth: u32,
        world: &T,
        rng: &mut RandomNumberGenerator,
    ) -> RGB<f32> {
        assert!(ray.direction().is_unit_vector());
        if depth <= 0 {
            return RGB::black();
        }
        match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(hit_record) => match hit_record.material.scatter(rng, ray, &hit_record) {
                Some((scattered_ray, attenuation)) => {
                    attenuation * Self::ray_color(&scattered_ray, depth - 1, world, rng)
                }
                None => RGB::black(),
            },
            None => {
                let unit_direction = Vector3::unit_vector(ray.direction());
                let a = ((unit_direction.y + 1.0) * 0.5) as f32;
                RGB::new(1.0, 1.0, 1.0) * (1.0 - a) + RGB::new(0.5, 0.7, 1.0) * a
            }
        }
    }

    fn pixel_sample_square(&self, rng: &mut RandomNumberGenerator) -> Vector3<f64> {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + rng.next_f64();
        let py = -0.5 + rng.next_f64();
        return (self.pixel_delta_u * px) + (self.pixel_delta_v * py);
    }

    fn get_ray(&self, i: u32, j: u32, rng: &mut RandomNumberGenerator) -> Ray<f64> {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.

        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square(rng);

        let ray_origin = self.defocus_disc_sample(rng);
        let ray_direction = (pixel_sample - ray_origin).unit_vector();

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disc_sample(&self, rng: &mut RandomNumberGenerator) -> Point3<f64> {
        // returns a random point in the camera defocus disk
        let p = Vector3::random_in_unit_disc(&mut RNGAdapter::new(rng));
        self.center + self.defocus_disc_u * p.x + self.defocus_disc_v * p.y
    }
}

pub struct RNGAdapter<'a>(&'a mut RandomNumberGenerator);

impl<'a> RNGAdapter<'a> {
    pub fn new(rng: &'a mut RandomNumberGenerator) -> RNGAdapter<'a> {
        RNGAdapter(rng)
    }
}

impl RandomSource<f32> for RNGAdapter<'_> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f32 {
        self.0.next_f32()
    }
    fn next_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.0.next_f32() * (max - min)
    }
}
impl RandomSource<f64> for RNGAdapter<'_> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f64 {
        self.0.next_f64()
    }
    fn next_range(&mut self, min: f64, max: f64) -> f64 {
        min + self.0.next_f64() * (max - min)
    }
}
