use rand::Rng;
use ray_tracer::{Camera, Color, HitRecord, Hittable, HittableList, Ray, Sphere, Vec3};
use std::{f64::INFINITY, rc::Rc};

/// Generates a random number between 0 and 1.
fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// Generates a random number within a specified range.
fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal.unwrap() + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: usize = 100; // TODO: Is this the optimal sample size?

    // World
    let world = HittableList {
        objects: vec![
            Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            println!("{}", pixel_color.to_color_string(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
