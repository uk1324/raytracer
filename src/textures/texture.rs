use crate::{vec3::{Pt3, Color}, vec2::Vec2};

pub trait Texture where Self: Send + Sync {
    fn color(&self, uv: Vec2, hit_point: Pt3) -> Color;
}