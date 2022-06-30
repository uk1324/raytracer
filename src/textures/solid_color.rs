use crate::{vec3::Vec3, vec2::Vec2};

use super::Texture;

pub struct SolidColor {
    pub color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn color(&self, _: Vec2, _: Vec3) -> Vec3 {
        self.color
    }
}