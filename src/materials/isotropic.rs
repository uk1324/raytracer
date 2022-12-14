use std::sync::Arc;

use crate::{textures::Texture, vec3::{Color, Pt3, Vec3}, vec2::Vec2, hittable_objects::HitRecord, ray::Ray};

use super::{Material, ScatterRecord};

pub struct Isotropic {
    albedo: Arc<dyn Texture> 
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new(
            &Ray::new(hit_record.point, Vec3::random_in_unit_sphere()), 
            self.albedo.color(hit_record.texture_coord, hit_record.point)))
    }

    fn color_emmited(&self, uv: Vec2, hit_point: Pt3) -> Color {
        Color::all(0.0)
    }
}