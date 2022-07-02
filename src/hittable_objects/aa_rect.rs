use std::rc::Rc;

use crate::{aabb::Aabb, vec3::Vec3, ray::Ray, materials::Material, vec2::Vec2};

use super::{Hittable, HitRecord};

// Could use macros to generate all the versions or use a single function and pass the arguments.
pub struct XyRect {
    // Maybe rename to xMin and yMax.
    // Not using Vec2 because it might be confusing for other planes.
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub z: f32,
    pub material: Rc<dyn Material>
}

impl XyRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, z: f32, material: Rc<dyn Material> ) -> Self {
        Self { x0, x1, y0, y1, z, material }
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

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            plane_hit_point, 
            ray, 
            Vec3::new(0.0, 0.0, 1.0), 
            t, 
            Vec2::new((x - self.x0) / (self.x1 - self.x0), (y - self.y0) / (self.y1 - self.y0)),
            self.material.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.z - 0.0001), 
            Vec3::new(self.x1, self.y1, self.z + 0.0001)))
    }
}

pub struct XzRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub y: f32,
    pub material: Rc<dyn Material>
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, y: f32, material: Rc<dyn Material> ) -> Self {
        Self { x0, x1, z0, z1, y, material }
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

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            plane_hit_point, 
            ray, 
            Vec3::new(0.0, 1.0, 0.0), 
            t, 
            Vec2::new((x - self.x0) / (self.x1 - self.x0), (z - self.z0) / (self.z1 - self.z0)),
            self.material.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y - 0.0001, self.z0), 
            Vec3::new(self.x1, self.y + 0.0001, self.z1)))
    }
}

pub struct YzRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub x: f32,
    pub material: Rc<dyn Material>
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, x: f32, material: Rc<dyn Material> ) -> Self {
        Self { y0, y1, z0, z1, x, material }
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

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            plane_hit_point, 
            ray, 
            Vec3::new(1.0, 0.0,  0.0), 
            t, 
            Vec2::new((y - self.y0) / (self.y1 - self.y0), (z - self.z0) / (self.z1 - self.z0)),
            self.material.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x - 0.0001, self.y0,  self.z0), 
            Vec3::new(self.x + 0.0001, self.y1,  self.z1)))
    }
}