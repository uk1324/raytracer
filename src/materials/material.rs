use crate::hittable_objects::HitRecord;
use crate::vec2::Vec2;
use crate::vec3::{Vec3, Color, Pt3};
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Vec3
}

impl ScatterRecord {
    pub fn new(ray: &Ray, attenuation: Vec3) -> ScatterRecord {
        ScatterRecord{ ray: *ray, attenuation }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
    // There is no way to provide a overridable default implementation.
    fn color_emmited(&self, uv: Vec2, hit_point: Pt3) -> Color;
}