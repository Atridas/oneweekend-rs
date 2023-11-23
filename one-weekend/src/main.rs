use math::*;
use one_weekend::Camera;
use stb;

fn main() {
    // World
    let mut world: Vec<GeometricPrimitive<f64>> = Vec::new();
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        100.0,
    )));

    let world = &world[..];

    // Camera

    let camera = Camera::new(16.0 / 9.0, 400);

    // Render

    let data = camera.render(&world);

    eprint!("\rWriting image            ");

    stb::write_png(
        "image.png",
        camera.get_image_width(),
        camera.get_image_height(),
        3,
        &RGB::to_byte_array(&data),
    )
    .unwrap();
    eprint!("\rDONE            \n");
}
