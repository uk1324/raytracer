use crate::materials::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable_objects::HitRecord;
use rand::Rng;

pub struct Dielectric {
    index_of_refraction: f32
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric{ index_of_refraction }
    }

    fn reflectance(cosine: f32, index_of_refraction: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Vec3::new_all(1.0);
        let refraction_ratio = if hit_record.is_front_face { 1.0 / self.index_of_refraction } else { self.index_of_refraction };

        let direction = ray.direction.normalized();
        let cos_theta = f32::min(Vec3::dot(-direction, hit_record.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract 
            || (Dielectric::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen()) {
            Vec3::reflect(direction, hit_record.normal)
        } else {
            Vec3::refract(direction, hit_record.normal, refraction_ratio)
        };

        Some(ScatterRecord::new(&Ray::new(hit_record.point, direction), attenuation))
    }
}