use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    // Doesn't have to be normalized.
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray{ origin: origin, direction: direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}