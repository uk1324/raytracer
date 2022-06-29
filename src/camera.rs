use rand::Rng;

use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand::thread_rng().gen_range(-1.0..1.0), rand::thread_rng().gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up: Vec3, fov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32) -> Camera {
        let h = f32::tan(fov / 2.0);

        let viewport_height: f32 = 2.0 * h;
        let viewport_width: f32 = aspect_ratio * viewport_height;
    
        let w = (look_from - look_at).normalized();
        let u = Vec3::cross(up, w).normalized();
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (focus_distance * w);

        Camera {origin: origin, lower_left_corner: lower_left_corner, vertical: vertical, horizontal: horizontal,
            lens_radius: aperture / 2.0, w: w, u: u, v: v
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = (self.u * rd.x) + (self.v * rd.y);

        Ray::new(self.origin + offset, (self.lower_left_corner + (u * self.horizontal)) + (v * self.vertical) - self.origin - offset)
    }
}