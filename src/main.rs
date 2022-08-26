use ray_tracer;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = ray_tracer::Color::new(
                i as f64 / (IMAGE_WIDTH as f64 - 1_f64),
                j as f64 / (IMAGE_HEIGHT as f64 - 1_f64),
                0.25,
            );
            ray_tracer::write_color(pixel_color);
        }
    }

    eprintln!("Done.");
}
