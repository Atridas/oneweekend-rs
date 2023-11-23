use math::*;
use stb;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400usize;

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
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render

    let mut data = Vec::with_capacity(image_width * image_height * 3);

    for j in 0..image_height {
        eprint!("\rScanlines remaining {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let rgb: RGB<f32> = ray_color(&r);

            data.push(rgb);
        }
    }

    eprint!("\rWriting image            ");

    stb::write_png(
        "image.png",
        image_width as u32,
        image_height as u32,
        3,
        &RGB::to_byte_array(&data),
    )
    .unwrap();
    eprint!("\rDONE            \n");
}
