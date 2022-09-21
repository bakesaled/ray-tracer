use crate::{random_in_unit_vector, Color, Material, Ray};

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: crate::Ray, rec: crate::HitRecord) -> crate::MaterialRecord {
        let mut scatter_direction = rec.normal.unwrap() + random_in_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.unwrap();
        }

        return crate::MaterialRecord {
            attenuation: self.albedo,
            scattered: Some(Ray::new(rec.p.unwrap(), scatter_direction)),
            scatter: true,
        };
    }
}
