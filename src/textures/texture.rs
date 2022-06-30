use crate::{vec3::Vec3, vec2::Vec2};

pub trait Texture {
    fn color(&self, uv: Vec2, hit_point: Vec3) -> Vec3;
}