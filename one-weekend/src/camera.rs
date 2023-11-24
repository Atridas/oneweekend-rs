use math::*;
use noise::RandomNumberGenerator;

pub struct Camera {
    center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
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
    ) -> Camera {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let image_height = if image_height < 1usize {
            1usize
        } else {
            image_height
        };

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u * 0.5
            - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            image_width,
            image_height: image_height as u32,
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

    pub fn render<T: Hittable<f64>>(
        &self,
        world: &T,
        rng: &mut RandomNumberGenerator,
    ) -> Vec<RGB<f32>> {
        let mut data = Vec::with_capacity((self.image_width * self.image_height * 3) as usize);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {} ", self.image_height - j);
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

    fn ray_color<T: Hittable<f64>>(
        ray: &Ray<f64>,
        depth: u32,
        world: &T,
        rng: &mut RandomNumberGenerator,
    ) -> RGB<f32> {
        if depth <= 0 {
            return RGB::black();
        }
        match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(record) => {
                let direction = (record.normal
                    + Vector3::random_on_hemisphere(&mut RNGAdapter(rng), record.normal))
                .unit_vector();

                Self::ray_color(&Ray::new(record.point, direction), depth - 1, world, rng) * 0.5
            }
            None => {
                let unit_direction = Vector3::unit_vector(ray.direction());
                let a = ((unit_direction.y + 1.0) * 0.5) as f32;
                RGB::new(1.0, 1.0, 1.0) * (1.0 - a) + RGB::new(0.5, 0.7, 1.0) * a
            }
        }
    }

    fn pixel_sample_square(&self, rng: &mut RandomNumberGenerator) -> Vector3<f64> {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + rng.next_double();
        let py = -0.5 + rng.next_double();
        return (self.pixel_delta_u * px) + (self.pixel_delta_v * py);
    }

    fn get_ray(&self, i: u32, j: u32, rng: &mut RandomNumberGenerator) -> Ray<f64> {
        // Get a randomly sampled camera ray for the pixel at location i,j.

        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square(rng);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }
}

struct RNGAdapter<'a>(&'a mut RandomNumberGenerator);

impl<'a> RandomSource<f32> for RNGAdapter<'a> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f32 {
        self.0.next_float()
    }
    fn next_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.0.next_float() * (max - min)
    }
}
impl<'a> RandomSource<f64> for RNGAdapter<'a> {
    /// Generates a number between [0 and 1)
    fn next(&mut self) -> f64 {
        self.0.next_double()
    }
    fn next_range(&mut self, min: f64, max: f64) -> f64 {
        min + self.0.next_double() * (max - min)
    }
}
