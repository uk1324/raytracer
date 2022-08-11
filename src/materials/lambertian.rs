use std::sync::Arc;

use crate::materials::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::hittable_objects::HitRecord;
use crate::textures::{Texture, SolidColor};
use crate::vec2::Vec2;
use crate::vec3::{Vec3, Pt3, Color};

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self{ albedo: texture }
    }

    pub fn from_color(albedo: Vec3) -> Lambertian {
        Self::new(Arc::new(SolidColor::new(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        // Generate random vector on the unit sphere centerted at the normal.
        // This generates a direction with distribution of cos(angle) so later the lambertian factor doesn't need to be applied.
        // Generating a uniformly distributed point on a sphere and then applying lambert's cosine law would probably have the same effect.
        let random_direction = hit_record.normal + Vec3::random_unit();

        // Discard really short vectors so later division by zero doesn't happen.
        // Could also loop until the generated vector is not small.
        let scatter_direction = if random_direction.is_near_zero() { hit_record.normal } else { random_direction };
        Some(ScatterRecord::new(
            &Ray::new(hit_record.point, scatter_direction), 
            self.albedo.color(hit_record.texture_coord, hit_record.point)))
    }

    fn color_emmited(&self, _: Vec2, _: Pt3) -> Color {
        Color::all(0.0)
    }
}