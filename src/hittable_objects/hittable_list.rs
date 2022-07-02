use std::rc::Rc;
use crate::hittable_objects::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::aabb::Aabb;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{ objects: Vec::new() }
    }

    pub fn from_vec(objects: Vec<Rc<dyn Hittable>>) -> HittableList {
        HittableList{ objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit_record: Option<HitRecord> = None;
        let mut closest_hit_t = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_hit_t) {
                if hit_record.t < closest_hit_t {
                    closest_hit_t = hit_record.t;
                    closest_hit_record = Some(hit_record);
                }
            }
        }

        closest_hit_record
    }

    fn bounding_box(&self) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None
        }

        let mut total = match self.objects[0].bounding_box() {
            Some(bb) => bb,
            None => return None,
        };

        for object in (&self.objects).into_iter().skip(1) {
            match object.bounding_box() {
                Some(bb) => total = total.combined(&bb),
                None => return None
            }
        }
        Some(total)
    }
}