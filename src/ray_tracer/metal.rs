use crate::{random_in_unit_sphere, Color, Material, Ray};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Metal {
        let mut f = 1.0;
        if fuzz < 1.0 {
            f = fuzz;
        }
        Metal {
            albedo: color,
            fuzz: f,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: crate::Ray, rec: crate::HitRecord) -> crate::MaterialRecord {
        let reflected = ray.direction().unit_vector().reflect(rec.normal.unwrap());
        let scattered = Ray::new(
            rec.p.unwrap(),
            reflected + self.fuzz * random_in_unit_sphere(),
        );

        return crate::MaterialRecord {
            attenuation: self.albedo,
            scattered: Some(scattered),
            scatter: scattered.direction().dot(rec.normal.unwrap()) > 0.0,
        };
    }
}
