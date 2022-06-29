use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::materials::Material;
use std::option::Option;
use std::rc::Rc;

// #[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(point: Vec3, ray: &Ray, outward_normal: Vec3, t: f32, material: &Rc<dyn Material>) -> HitRecord {
        let is_front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if is_front_face { outward_normal } else { -outward_normal };
        HitRecord{ point, normal, t, is_front_face, material: Rc::clone(material) }
    }
    // pub fn new(point: &Vec3, normal: &Vec3, t: f32) -> HitRecord {
    //     HitRecord{ point: *point, normal: *normal, t}
    // }

    // pub fn set_face_normal(&self, ray: &Ray, outward_normal: &Vec3) {
    //     self.is_front_face = Vec3::dot(&ray.direction, &outward_normal) > 0.0;
    //     self.normal = if self.is_front_face { *outward_normal } else { -*outward_normal };
    // }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    // Optional because for example infinite planes don't have an AABB
    fn bounding_box(&self) -> Option<Aabb>;
}