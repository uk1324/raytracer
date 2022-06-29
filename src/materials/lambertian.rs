use crate::materials::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::hittable_objects::HitRecord;
use crate::vec3::Vec3;
use crate::random::random_point_in_hemisphere;

pub struct Lambertian {
    pub albedo: Vec3
}

fn random_point_in_unit_sphere() -> Vec3 {
    loop {
        let point = Vec3::new_random_in_range(-1.0, 1.0);
        if point.length_squared() < 1.0 {
            return point
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{ albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.normal + random_point_in_hemisphere(hit_record.normal);
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }
        Some(ScatterRecord::new(&Ray::new(hit_record.point, scatter_direction), self.albedo))
    }
}