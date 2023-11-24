use external::stb;
use math::*;
use noise::RandomNumberGenerator;
use one_weekend::{Camera, GeometricPrimitive, Lambertian, Metal, Sphere};

fn main() {
    // Materials

    let material_ground = Lambertian::new(RGB::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(RGB::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(RGB::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(RGB::new(0.8, 0.6, 0.2), 1.0);

    // World

    let mut world: Vec<GeometricPrimitive<f64, f32>> = Vec::new();
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        100.0,
        &material_ground,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        &material_center,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        &material_left,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        &material_right,
    )));

    let world = &world[..];

    // Camera

    let camera = Camera::new(16.0 / 9.0, 400, 100, 50);

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
