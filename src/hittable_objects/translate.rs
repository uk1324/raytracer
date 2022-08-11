use std::sync::Arc;

use crate::{vec3::Vec3, aabb::Aabb, ray::Ray};

use super::{Hittable, HitRecord};

pub struct Translate {
    pub hittable: Arc<dyn Hittable>,
    pub translation: Vec3
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, translation: Vec3) -> Self {
        Self{ hittable, translation }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let ray = Ray::new(ray.origin - self.translation, ray.direction);
        match self.hittable.hit(&ray, t_min, t_max) {
            Some(mut hit) => {
                hit.point += self.translation;
                Some(hit)
            }
            None => None
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        match self.hittable.bounding_box() {
            Some(aabb) => Some(Aabb::new(aabb.min + self.translation, aabb.max + self.translation)),
            None => None
        }
    }
}