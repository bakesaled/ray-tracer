use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + (t * self.dir)
    }
}
