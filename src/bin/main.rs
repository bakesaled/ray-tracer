use ray_tracer::{
    random_in_range, vec3_random, vec3_random_in_range, Camera, Color, Dielectric, HitRecord,
    Hittable, HittableList, Lambertian, Material, Metal, Point3, Ray, Sphere, Vec3,
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

fn random_scene() -> HittableList<Sphere> {
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let mut world = HittableList {
        objects: vec![Rc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        ))],
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = ray_tracer::random();
            let center = Point3::new(
                a as f64 + 0.9 * ray_tracer::random(),
                0.2,
                b as f64 + 0.9 * ray_tracer::random(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3_random() * vec3_random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    // world
                    //     .objects
                    //     .push(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3_random_in_range(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    // world
                    //     .objects
                    //     .push(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    // world
                    //     .objects
                    //     .push(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
                world
                    .objects
                    .push(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.objects.push(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: usize = 100; // TODO: Is this the optimal sample size?
    const MAX_DEPTH: i32 = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

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
