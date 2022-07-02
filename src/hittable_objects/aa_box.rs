use std::rc::Rc;

use crate::{vec3::Pt3, materials::Material, ray::Ray, aabb::Aabb};

use super::{HittableList, XyRect, XzRect, YzRect, Hittable, HitRecord};

pub struct AaBox {
    // TODO just store the AABB
    pub min: Pt3,
    pub max: Pt3,
    pub sides: HittableList
}

impl AaBox {
    pub fn new(min: Pt3, max: Pt3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        // sides.objects.push(value)
        sides.objects.push(Rc::new(XyRect::new(min.x, max.x, min.y, max.y, max.z, material.clone())));
        sides.objects.push(Rc::new(XyRect::new(min.x, max.x, min.y, max.y, min.z, material.clone())));
        sides.objects.push(Rc::new(XzRect::new(min.x, max.x, min.z, max.z, max.y, material.clone())));
        sides.objects.push(Rc::new(XzRect::new(min.x, max.x, min.z, max.z, min.y, material.clone())));
        sides.objects.push(Rc::new(YzRect::new(min.y, max.y, min.z, max.z, max.x, material.clone())));
        sides.objects.push(Rc::new(YzRect::new(min.y, max.y, min.z, max.z, min.x, material)));
        AaBox{ min, max, sides }
    }
}

impl Hittable for AaBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}