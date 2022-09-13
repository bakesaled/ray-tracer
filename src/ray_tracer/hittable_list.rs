use std::rc::Rc;

use crate::{HitRecord, Hittable};

pub struct HittableList<T: Hittable> {
    pub objects: Vec<Rc<T>>,
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, ray: crate::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t().unwrap();
                rec.set_rec(&temp_rec);
            }
        }

        hit_anything
    }
}
