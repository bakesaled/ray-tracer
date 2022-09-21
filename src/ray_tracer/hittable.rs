use std::rc::Rc;

use crate::{Material, Point3, Ray, Vec3};

pub struct HitRecord {
    pub p: Option<Point3>,
    pub normal: Option<Vec3>,
    pub t: Option<f64>,
    pub front_face: Option<bool>,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: None,
            normal: None,
            t: None,
            front_face: None,
            material: None,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Some(ray.direction().dot(outward_normal) < 0.0);
        self.normal = if self.front_face.unwrap() {
            Some(outward_normal)
        } else {
            Some(-outward_normal)
        };
    }

    pub fn t(&self) -> Option<f64> {
        self.t
    }

    pub fn set_rec(&mut self, rec: &HitRecord) {
        self.p = rec.p.clone();
        self.t = rec.t.clone();
        self.normal = rec.normal.clone();
        self.front_face = rec.front_face.clone();
        self.material = rec.material.clone();
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
