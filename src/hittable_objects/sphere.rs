use std::f32::consts::{PI, TAU /* = 2 * PI */};
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable_objects::{Hittable, HitRecord};
use crate::materials::Material;
use crate::vec2::Vec2;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Material>
}

/*
The parametric equation for a sphere is 
y = cos(theta)
x = cos(phi) * sin(theta)
z = sin(phi) * sin(theta)
where phi is the angle in range <0, TAU> around the y axis
and theta is the angle in range <0, PI> between [0, -1, 0] and [0, 1, 0]

y = cos(theta) becuase it describes the x position of the half circle between the up and down vectors.
sin(theta) could be thought of as a line parallel to the xy plane going straight from the center of the sphere.

x = cos(phi) * sin(theta) becuase cos(phi) describes the x position on the circle that lies on the xy plane scaled by sin(theta)
sin(phi) describes the position y on the circle and it is also scaled by sin(theta)

This raytracer uses a different coorinate system so
y = -cos(theta)
x = -cos(phi) * sin(theta)
z = sin(phi) * sin(theta)
*/

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Sphere{ center: center, radius, material }
    }

    fn texture_coord(point_on_unit_sphere_centered_at_origin: Vec3) -> Vec2 {
        let p = &point_on_unit_sphere_centered_at_origin;

        let theta = f32::acos(-p.y);
        // atan can be used becuase x and z are both scaled by sin(theta)
        let phi = f32::atan2(-p.z, p.x) + PI;

        Vec2::new(phi / (TAU), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // The equation for the sphere centered at the origin is x^2 + y^2 + z^2 = r^2.
        // If this equation is true this means the vector `v = (x, y, z)` lies on the sphere.
        // If the sphere is at position `center` then (v.x - center.x)^2 + (v.y - center.y)^2 + (v.z - center.z)^2 = r^2.
        // This can be written with the dot prodcut as dot(v - center, v - center) = r^2
        // dot(ray.origin + t * ray.direction - center, ray.origin + t * ray.direction - center) = r^2
        // oc = ray.origin - center
        // dot(oc + t * ray.direction, oc + t * ray.direction) = r^2
        // dot(oc, oc) + 2 * t * dot(ray.direction, oc) + t^2 * dot(ray.direction, ray.direction) - r^2 = 0
        // a = dot(ray.direction, ray.direction)
        // b = 2 * dot(ray.direction, oc)
        // c = dot(oc, oc) - r^2
        // discriminant = b^2 - 4.0 * a * c
        // roots = (-b +- sqrt(discriminant)) / (2.0 * a)
        // To get b, dot(ray.direction, oc) has to be multiplied by 2. Because of this 3 multiplications can be removed.
        // b = 2h
        // (-(2 * h) +- sqrt((2 * h)^2 - 4.0 * a * c)) / (2.0 * a)
        // (-(2 * h) +- sqrt(4 * h^2 - 4.0 * a * c)) / (2.0 * a)
        // (-(2 * h) +- 2 * sqrt(h^2 - a * c)) / (2.0 * a)
        // (-h +- sqrt(h^2 - * a * c)) / a
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let half_b = Vec3::dot(oc, ray.direction);
        let c= Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        }

        let sqrt_discriminant = discriminant.sqrt();

        let root = (-half_b - sqrt_discriminant) / a;

        if (root < t_min) || (root > t_max) {
            return None
        }

        let point = ray.at(root);
        // Normal is the ray from the center of the sphere to the point.
        // Normalizing it by dividing by radius.
        let outward_normal = (point - self.center) / self.radius;
        
        let uv = Self::texture_coord(outward_normal);

        Some(HitRecord::new(point, ray, outward_normal, root, uv, self.material.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::all(self.radius), 
            self.center + Vec3::all(self.radius)))
    }
}