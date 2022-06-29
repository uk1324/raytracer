use crate::{vec3::Vec3, ray::Ray};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let (mut t_entry, mut t_exit) = (t_min, t_max);
        // a stands for axis
        for axis in 0..3 {
            // min[a] = ray.origin[a] + time * ray.direction[a]
            // time = (min[a] - ray.origin[a]) / ray.direction[a]
            // same for max
            let direction_inverse = 1.0 / ray.direction[axis];
            let mut t_entry_axis = (self.min[axis] - ray.origin[axis]) * direction_inverse;
            let mut t_exit_axis = (self.max[axis] - ray.origin[axis]) * direction_inverse;
            // If direction is negative then the ray will get earlier to t_exit than t_entry.
            if direction_inverse < 0.0 {
                std::mem::swap(&mut t_entry_axis, &mut t_exit_axis);
            }

            // t_entry is the most time you can move before hitting an axis
            // t_entry is the earliest exit you can make.
            t_entry = if t_entry_axis > t_entry { t_entry_axis } else { t_entry };
            t_exit = if t_exit_axis < t_exit { t_exit_axis } else { t_exit };
            
            // If t lies between the farthest entrance and closest exit then it is inside the box.
            if t_exit <= t_entry {
                return false;
            }
        }
        true
    }

    pub fn combined(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: Vec3::new(
                f32::min(self.min.x, other.min.x),
                f32::min(self.min.y, other.min.y),
                f32::min(self.min.z, other.min.z)),
            max: Vec3::new(
                f32::max(self.max.x, other.max.x),
                f32::max(self.max.y, other.max.y),
                f32::max(self.max.z, other.max.z)),
        }
    }
}

