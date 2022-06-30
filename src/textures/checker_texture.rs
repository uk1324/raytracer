use std::rc::Rc;

use crate::{vec2::Vec2, vec3::Vec3};

use super::Texture;

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self{ odd, even }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, uv: Vec2, hit_point: Vec3) -> Vec3 {
        // Using sin becuase it changes sign cyclically.
        if (f32::sin(10.0 * hit_point.x) * f32::sin(10.0 * hit_point.y) * f32::sin(10.0 * hit_point.z)) > 0.0 {
            self.odd.color(uv, hit_point)
        } else {
            self.even.color(uv, hit_point)
        }
    }
}