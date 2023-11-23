use stb;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    let mut data = Vec::with_capacity(image_width * image_height * 3);

    for j in 0..image_height {
        eprint!("\rScanlines remaining {} ", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (r * 255.999) as u8;
            let ig = (g * 255.999) as u8;
            let ib = (b * 255.999) as u8;

            data.push(ir);
            data.push(ig);
            data.push(ib);
        }
    }

    eprint!("\rWriting image            ");

    stb::write_png(
        "image.png",
        image_width as u32,
        image_height as u32,
        3,
        &data,
    )
    .unwrap();
    eprint!("\rDONE            ");
}
