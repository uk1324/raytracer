use crate::{perlin::Perlin, vec2::Vec2, vec3::Vec3};

use super::Texture;

pub struct NoiseTexture {
    pub perlin: Perlin
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self { perlin: Perlin::new() }
    }
}

impl Texture for NoiseTexture {
    fn color(&self, _: Vec2, hit_point: Vec3) -> Vec3 {
        self.perlin.get(hit_point) * Vec3::new_all(1.0)
    }
}