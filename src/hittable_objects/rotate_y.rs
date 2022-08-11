use std::sync::Arc;

use crate::{aabb::Aabb, vec3::{Pt3, Vec3}, ray::Ray};

use super::{Hittable, HitRecord};

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin: f32,
    cos: f32,
    aabb: Option<Aabb>
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        let aabb = match hittable.bounding_box() {
            Some(aabb) => aabb,
            None => return RotateY{ hittable, sin, cos, aabb: None }
        };

        let mut max = Pt3::all(f32::INFINITY);
        let mut min = Pt3::all(-f32::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (i, j, k) = (i as f32, j as f32, k as f32);
                    let x = i * aabb.max.x + (1.0 - i) * aabb.min.x;
                    let y = j * aabb.max.y + (1.0 - j) * aabb.min.y;
                    let z = k * aabb.max.z + (1.0 - k)* aabb.min.z;

                    let new_x = cos * x + sin * z;
                    let new_z = -cos * x + cos * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f32::min(min[c], tester[c]);
                        max[c] = f32::max(max[c], tester[c]);
                    }
                }
            }
        }

        RotateY{ hittable, sin, cos, aabb: Some(aabb) }
    }
}

impl Hittable for RotateY {
    fn hit(&self, Ray{ origin, direction }: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = Vec3::new(
            self.cos * origin.x - self.sin * origin.z,
            origin.y,
            self.sin * origin.x + self.cos * origin.z);

        let direction = Vec3::new(
            self.cos * direction.x - self.sin * direction.z,
            direction.y,
            self.sin * direction.x + self.cos * direction.z);

        let ray = Ray::new(origin, direction);

        let HitRecord { point, normal, t, texture_coord, material, .. } 
            = self.hittable.hit(&ray, t_min, t_max)?;

        let point = Vec3::new(
            self.cos * point.x + self.sin * point.z,
            point.y,
            -self.sin * point.x + self.cos * point.z);

        let normal = Vec3::new(
            self.cos * normal.x + self.sin * normal.z,
            normal.y,
            -self.sin * normal.x + self.cos * normal.z).normalized();

        Some(HitRecord::new(point, &ray, normal, t, texture_coord, material))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        self.aabb
    }
}