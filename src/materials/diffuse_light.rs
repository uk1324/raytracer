use std::sync::Arc;

use crate::{textures::Texture, ray::Ray, hittable_objects::HitRecord, vec3::{Color, Pt3}, vec2::Vec2};

use super::{Material, ScatterRecord};

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn color_emmited(&self, uv: Vec2, hit_point: Pt3) -> Color {
        self.emit.color(uv, hit_point)
    }
}