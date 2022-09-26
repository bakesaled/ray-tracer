use ray_tracer::{
    Camera, Color, Dielectric, HitRecord, Hittable, HittableList, Lambertian, Metal, Ray, Sphere,
    Vec3,
};
use std::{f64::INFINITY, rc::Rc};

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::new();

    // If we've exceeded the ray bounce limit, gather no more light.
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        if let Some(in_mat_rec) = rec.material.clone() {
            let mat_rec = in_mat_rec.scatter(r, rec);
            if mat_rec.scatter {
                return mat_rec.attenuation
                    * ray_color(mat_rec.scattered.unwrap(), world, depth - 1);
            }
        }
        return Color::new(0.0, 0.0, 0.0);
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
    const MAX_DEPTH: i32 = 50;

    // World
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let world = HittableList {
        objects: vec![
            Rc::new(Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground,
            )),
            Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)),
            Rc::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                material_left.clone(),
            )),
            Rc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left)),
            Rc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
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
                let u = (i as f64 + ray_tracer::random()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + ray_tracer::random()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            println!("{}", pixel_color.to_color_string(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");
}
