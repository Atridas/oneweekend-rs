use external::stb;
use math::*;
use noise::RandomNumberGenerator;
use one_weekend::{
    Camera, Dielectric, GeometricPrimitive, Lambertian, Material, Metal, RNGAdapter, Sphere,
};

enum MaterialIndex {
    Lambertian(usize),
    Metal(usize),
    Dielectric,
}

fn main() {
    // RNG

    let mut rng = RandomNumberGenerator::new(42);

    // Materials

    let material_ground = Lambertian::new(RGB::new(0.5, 0.5, 0.5));
    let material1 = Dielectric::new(1.5);
    let material2 = Lambertian::new(RGB::new(0.4, 0.2, 0.1));
    let material3 = Metal::new(RGB::new(0.7, 0.6, 0.5), 0.0);

    let mut materials_lambertian = Vec::new();
    let mut materials_metal = Vec::new();
    let material_dielectric = &material2;

    let mut material_indices = Vec::with_capacity(500);
    for _ in 0..500 {
        let choose_mat = rng.next_f32();
        if choose_mat < 0.8 {
            // lambertian
            material_indices.push(MaterialIndex::Lambertian(materials_lambertian.len()));
            let albedo =
                RGB::random(&mut RNGAdapter(&mut rng)) * RGB::random(&mut RNGAdapter(&mut rng));
            materials_lambertian.push(Lambertian::new(albedo));
        } else if choose_mat < 0.95 {
            material_indices.push(MaterialIndex::Metal(materials_metal.len()));
            let albedo = RGB::random_range(&mut RNGAdapter(&mut rng), 0.5, 1.0);
            let fuzz = rng.next_range_f32(0.0, 0.5);
            materials_metal.push(Metal::new(albedo, fuzz));
        } else {
            material_indices.push(MaterialIndex::Dielectric);
        }
    }

    let materials_lambertian = materials_lambertian;
    let materials_metal = materials_metal;
    let material_indices = material_indices;

    // World

    let mut world: Vec<GeometricPrimitive<f64, f32>> = Vec::new();
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        1000.0,
        &material_ground,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        &material1,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        &material2,
    )));
    world.push(GeometricPrimitive::Sphere(Sphere::new(
        Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
        &material3,
    )));

    let mut idx = 0;
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + rng.next_range_f64(0.0, 0.9),
                0.2,
                b as f64 + rng.next_range_f64(0.0, 0.9),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length_squared() > 0.9 * 0.9 {
                let material: &dyn Material<f64, f32> = match material_indices[idx] {
                    MaterialIndex::Lambertian(idx) => &materials_lambertian[idx],
                    MaterialIndex::Metal(idx) => &materials_metal[idx],
                    MaterialIndex::Dielectric => material_dielectric,
                };
                let geometry = Sphere::new(center, 0.2, material);
                world.push(GeometricPrimitive::Sphere(geometry));
                idx = idx + 1;
            }
        }
    }

    let world = &world[..];

    // Camera

    let camera = Camera::new(
        16.0 / 9.0,
        1200,
        500,
        50,
        Degrees(20.0),
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Degrees(0.6),
        10.0,
    );

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
