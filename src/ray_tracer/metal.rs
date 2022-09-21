use crate::{Color, Material, Ray};

pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal { albedo: color }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: crate::Ray, rec: crate::HitRecord) -> crate::MaterialRecord {
        let reflected = ray.direction().unit_vector().reflect(rec.normal.unwrap());
        let scattered = Ray::new(rec.p.unwrap(), reflected);

        return crate::MaterialRecord {
            attenuation: self.albedo,
            scattered: Some(scattered),
            scatter: scattered.direction().dot(rec.normal.unwrap()) > 0.0,
        };
    }
}
