use std::sync::Arc;

use rand::Rng;

use crate::{aabb::Aabb, ray::Ray};

use super::{Hittable, HitRecord, HittableList};

pub struct BhvNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub aabb: Aabb
}

impl BhvNode {
    pub fn new(src_objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BhvNode {
        let mut objects = src_objects.clone();
        
        let axis = rand::thread_rng().gen_range(0..3);
        let box_compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            let box_a = a.bounding_box();
            let box_b = b.bounding_box();
        
            if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
                box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
            } else {
                panic!("both nodes must have bounding boxes")
            }
        };

        let object_span = end - start;

        let (left, right) =
        if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if box_compare(&objects[start], &objects[start + 1]).is_lt() {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            let mid = start + object_span / 2;
            objects.split_at_mut(start).1.split_at_mut(object_span).0.sort_by(box_compare);

            let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = (
                Arc::new(BhvNode::new(&objects, start, mid)),
                Arc::new(BhvNode::new(&objects, mid, end))
            );
            (left, right)
        };  

        let box_left = left.bounding_box();
        let box_right = right.bounding_box();
    
        if let (Some(box_left), Some(box_right)) = (box_left, box_right) {
            BhvNode{ left, right, aabb: box_left.combined(&box_right) }
        } else {
            panic!("both nodes must have bounding boxes")
        }
    }

    pub fn from_hittable_list(list: &HittableList) -> BhvNode {
        Self::new(&list.objects, 0, list.objects.len())
    }
}

impl Hittable for BhvNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let t_max = if let Some(hit) = &hit_left { hit.t } else { t_max };
        let hit_right = self.right.hit(ray, t_min, t_max);

        if let Some(_) = hit_right { hit_right } else { hit_left }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.aabb)
    }
}