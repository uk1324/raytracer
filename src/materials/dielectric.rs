use crate::materials::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec2::Vec2;
use crate::vec3::{Vec3, Pt3, Color};
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
        // Use Schlick's polynomial approximation for reflectance.
        let mut r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        r0 *= r0;
        r0 + (1.0 - r0) * f32::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        // Snells law states that
        // sin(output) = (index_of_refraction_of_input / index_of_refraction_of_output) * sin(input).
        // Air's index of refraction is near zero so depending on the side from which the ray comes the ratio is i or 1/i.
        let refraction_ratio = if hit_record.is_front_face { 1.0 / self.index_of_refraction } else { self.index_of_refraction };

        let direction = ray.direction.normalized();
        let cos = f32::min(Vec3::dot(-direction, hit_record.normal), 1.0);
        // sin^2 + cos^2 = 1
        // sin^2 = 1 - cos^2
        let sin = f32::sqrt(1.0 - cos * cos);

        // It is impossible for the sin of the angle be above 1 so when this happens the light is reflected.
        // This effect is called total internal reflection and the angle at which this starts to happen is called the cirtical angle.
        let cannot_refract = refraction_ratio * sin > 1.0;

        let direction = if cannot_refract
            // Random chance for rays to reflect.
            || (Self::reflectance(cos, refraction_ratio) > rand::thread_rng().gen()) {
            Vec3::reflect(direction, hit_record.normal)
        } else {
            Vec3::refract(direction, hit_record.normal, refraction_ratio)
        };
        Some(ScatterRecord::new(&Ray::new(hit_record.point, direction), Vec3::all(1.0)))
    }

    fn color_emmited(&self, _: Vec2, _: Pt3) -> Color {
        Color::all(0.0)
    }
}