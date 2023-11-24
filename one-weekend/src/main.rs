use external::stb;
use math::*;
use noise::RandomNumberGenerator;
use one_weekend::{Camera, GeometricPrimitive, Lambertian, Sphere};

fn main() {
    // Materials

    let lambertian1 = Lambertian::new(RGB::new(0.5, 0.5, 0.5));

    // World

    let mut world: Vec<GeometricPrimitive<f64, f32>> = Vec::new();
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        &lambertian1,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        100.0,
        &lambertian1,
    )));

    let world = &world[..];

    // Camera

    let camera = Camera::new(16.0 / 9.0, 400, 10, 50);

    // RNG

    let mut rng = RandomNumberGenerator::new(42);

    // Render

    let data = camera.render(&world, &mut rng);

    eprint!("\rWriting image            ");

    stb::write_png(
        "image.png",
        camera.get_image_width(),
        camera.get_image_height(),
        3,
        &RGB::to_srgb_array(&data),
    )
    .unwrap();
    eprint!("\rDONE            \n");
}
