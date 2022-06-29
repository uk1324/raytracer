use crate::hittable_objects::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList{ objects: Vec::new() }
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
}