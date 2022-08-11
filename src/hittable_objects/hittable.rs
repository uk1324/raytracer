use crate::vec2::Vec2;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::materials::Material;
use std::option::Option;
use std::sync::Arc;

// #[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
    pub texture_coord: Vec2, // range <0, 1> going from bottom left.
    pub material: Arc<dyn Material>
}

impl HitRecord {
    pub fn new(point: Vec3, ray: &Ray, outward_normal: Vec3, t: f32, texture_coord: Vec2, material: Arc<dyn Material>) -> HitRecord {
        let is_front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if is_front_face { outward_normal } else { -outward_normal };
        HitRecord{ point, normal, t, is_front_face, texture_coord, material }
    }
}

pub trait Hittable where Self: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    // Optional because for example infinite shapes like planes don't have an AABB.
    fn bounding_box(&self) -> Option<Aabb>;
}