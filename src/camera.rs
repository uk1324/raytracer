use crate::vec3::{Vec3, self};
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub view_plane_lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub forward: Vec3,
    pub lens_radius: f32
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vertical_fov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32) -> Camera {
        let viewport_height: f32 = 2.0 * f32::tan(vertical_fov / 2.0);
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let forward = (look_from - look_at).normalized();
        // Can't look straight up or down becuase then where should left and up vectors be.
        // cross(x, x) creates the zero vector so it can't work.
        assert!(forward != vec3::UP && forward != vec3::DOWN);
        let right = Vec3::cross(vec3::UP, forward).normalized();
        let up = Vec3::cross(forward, right);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * right;
        let vertical = focus_distance * viewport_height * up;
        let view_plane_lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (focus_distance * forward);

        Self {origin, view_plane_lower_left_corner, vertical, horizontal, lens_radius: aperture / 2.0, forward, right, up }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        let point_on_lens = self.lens_radius * Vec3::random_in_unit_disk_on_z_plane();
        let offset = (self.right * point_on_lens.x) + (self.up * point_on_lens.y);
        let origin = self.origin + offset;
        let point_on_view_plane = (self.view_plane_lower_left_corner + (u * self.horizontal)) + (v * self.vertical);
        let direction = point_on_view_plane - origin;
        Ray::new(origin, direction)
    }
}