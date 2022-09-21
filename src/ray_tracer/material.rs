use crate::{Color, HitRecord, Ray};

pub struct MaterialRecord {
    pub attenuation: Color,
    pub scattered: Option<Ray>,
    pub scatter: bool,
}

pub trait Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> MaterialRecord;
}
