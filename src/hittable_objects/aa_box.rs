use std::sync::Arc;

use crate::{vec3::Pt3, materials::Material, ray::Ray, aabb::Aabb};

use super::{HittableList, XyRect, XzRect, YzRect, Hittable, HitRecord};

pub struct AaBox {
    aabb: Aabb,
    sides: HittableList
}

impl AaBox {
    pub fn new(min: Pt3, max: Pt3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XyRect::new(min.x, max.x, min.y, max.y, max.z, material.clone())));
        sides.add(Arc::new(XyRect::new(min.x, max.x, min.y, max.y, min.z, material.clone())));
        sides.add(Arc::new(XzRect::new(min.x, max.x, min.z, max.z, max.y, material.clone())));
        sides.add(Arc::new(XzRect::new(min.x, max.x, min.z, max.z, min.y, material.clone())));
        sides.add(Arc::new(YzRect::new(min.y, max.y, min.z, max.z, max.x, material.clone())));
        sides.add(Arc::new(YzRect::new(min.y, max.y, min.z, max.z, min.x, material)));
        AaBox{ aabb: Aabb::new(min, max), sides }
    }
}

impl Hittable for AaBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.aabb)
    }
}