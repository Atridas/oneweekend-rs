use math::*;

pub struct Camera {
    center: Point3<f64>,
    pixel00_loc: Point3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>,
    image_width: u32,
    image_height: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Camera {
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
        }
    }

    pub fn get_image_width(&self) -> u32 {
        self.image_width
    }
    pub fn get_image_height(&self) -> u32 {
        self.image_height
    }

    pub fn render<T: Hittable<f64>>(&self, world: &T) -> Vec<RGB<f32>> {
        let mut data = Vec::with_capacity((self.image_width * self.image_height * 3) as usize);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let rgb: RGB<f32> = Camera::ray_color(&r, world);

                data.push(rgb);
            }
        }
        data
    }

    fn ray_color<T: Hittable<f64>>(ray: &Ray<f64>, world: &T) -> RGB<f32> {
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
}