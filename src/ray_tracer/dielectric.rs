use crate::{random, Color, Material, Ray};

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: crate::Ray, rec: crate::HitRecord) -> crate::MaterialRecord {
        let refraction_ratio = if rec.front_face.unwrap() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal.unwrap()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random() {
                unit_direction.reflect(rec.normal.unwrap())
            } else {
                unit_direction.refract(rec.normal.unwrap(), refraction_ratio)
            };

        crate::MaterialRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Some(Ray::new(rec.p.unwrap(), direction)),
            scatter: true,
        }
    }
}
