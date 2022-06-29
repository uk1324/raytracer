use crate::hittable_objects::HitRecord;
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Vec3
}

impl ScatterRecord {
    pub fn new(ray: &Ray, attenuation: Vec3) -> ScatterRecord {
        ScatterRecord{ ray: *ray, attenuation: attenuation}
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}