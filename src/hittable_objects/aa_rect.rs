use std::sync::Arc;

use crate::{aabb::Aabb, vec3::Vec3, ray::Ray, materials::Material, vec2::Vec2};

use super::{Hittable, HitRecord};

// Could use macros to generate all the versions or use a single function and pass the arguments.
// Not using Vec2 for min and max because it might be confusing for other planes.
pub struct XyRect {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z: f32,
    pub material: Arc<dyn Material>
}

impl XyRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, z: f32, material: Arc<dyn Material> ) -> Self {
        Self { x_min: x0, x_max: x1, y_min: y0, y_max: y1, z, material }
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let plane_hit_point = ray.at(t);
        let (x, y) = (plane_hit_point.x, plane_hit_point.y);

        if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
            return None;
        }

        Some(HitRecord::new(
            plane_hit_point, 
            ray, 
            Vec3::new(0.0, 0.0, 1.0), 
            t, 
            Vec2::new((x - self.x_min) / (self.x_max - self.x_min), (y - self.y_min) / (self.y_max - self.y_min)),
            self.material.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x_min, self.y_min, self.z - 0.0001), 
            Vec3::new(self.x_max, self.y_max, self.z + 0.0001)))
    }
}

pub struct XzRect {
    pub x_min: f32,
    pub x_max: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub y: f32,
    pub material: Arc<dyn Material>
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, y: f32, material: Arc<dyn Material> ) -> Self {
        Self { x_min: x0, x_max: x1, z_min: z0, z_max: z1, y, material }
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.y - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }

        let plane_hit_point = ray.at(t);
        let (x, z) = (plane_hit_point.x, plane_hit_point.z);

        if x < self.x_min || x > self.x_max || z < self.z_min || z > self.z_max {
            return None;
        }

        Some(HitRecord::new(
            plane_hit_point, 
            ray, 
            Vec3::new(0.0, 1.0, 0.0), 
            t, 
            Vec2::new((x - self.x_min) / (self.x_max - self.x_min), (z - self.z_min) / (self.z_max - self.z_min)),
            self.material.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x_min, self.y - 0.0001, self.z_min), 
            Vec3::new(self.x_max, self.y + 0.0001, self.z_max)))
    }
}

pub struct YzRect {
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub x: f32,
    pub material: Arc<dyn Material>
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, x: f32, material: Arc<dyn Material> ) -> Self {
        Self { y_min: y0, y_max: y1, z_min: z0, z_max: z1, x, material }
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.x - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }

        let plane_hit_point = ray.at(t);
        let (y, z) = (plane_hit_point.y, plane_hit_point.z);

        if y < self.y_min || y > self.y_max || z < self.z_min || z > self.z_max {
            return None;
        }

        Some(HitRecord::new(
            plane_hit_point, 
            ray, 
            Vec3::new(1.0, 0.0,  0.0), 
            t, 
            Vec2::new((y - self.y_min) / (self.y_max - self.y_min), (z - self.z_min) / (self.z_max - self.z_min)),
            self.material.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x - 0.0001, self.y_min,  self.z_min), 
            Vec3::new(self.x + 0.0001, self.y_max,  self.z_max)))
    }
}