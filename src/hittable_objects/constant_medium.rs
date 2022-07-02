use std::rc::Rc;

use rand::Rng;

use crate::{materials::Material, ray::Ray, aabb::Aabb, vec3::Vec3, vec2::Vec2};

use super::{Hittable, HitRecord};

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub phase_function: Rc<dyn Material>,
    pub density_inverse_negated: f32
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, phase_function: Rc<dyn Material>, density: f32) -> Self {
        Self{ boundary, phase_function, density_inverse_negated: -1.0 / density }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        // const bool enableDebug = false;
        // const bool debugging = enableDebug && random_double() < 0.00001;


        let mut hit1 = self.boundary.hit(ray, -f32::INFINITY, f32::INFINITY)?;
        let mut hit2 = self.boundary.hit(ray, hit1.t + 0.0001, f32::INFINITY)?;

        // if (debugging) std::cerr << "\nt_min=" << rec1.t << ", t_max=" << rec2.t << '\n';

        if hit1.t < t_min {
            hit1.t = t_min;
        }

        if hit2.t > t_max {
            hit2.t = t_max;
        }

        if hit1.t >= hit2.t {
            return None;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.density_inverse_negated * f32::ln(rand::thread_rng().gen_range(0.0..1.0));

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;
        Some(HitRecord::new(
            ray.at(t), 
            &Ray::new(Vec3::all(0.0), Vec3::all(0.0)), // Arbitrary
            Vec3::new(1.0, 0.0, 0.0), // Arbitrary
            t, 
            Vec2::all(0.0), // Arbitrary
            self.phase_function.clone()))
    }

    fn bounding_box(&self) -> Option<Aabb> {
        self.boundary.bounding_box()
    }
}