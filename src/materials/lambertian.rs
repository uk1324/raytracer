use std::rc::Rc;

use crate::materials::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::hittable_objects::HitRecord;
use crate::textures::{Texture, SolidColor};
use crate::vec2::Vec2;
use crate::vec3::{Vec3, Pt3, Color};

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>
}

fn random_point_in_unit_sphere() -> Vec3 {
    loop {
        let point = Vec3::new_random_in_range(-1.0, 1.0);
        if point.length_squared() < 1.0 {
            return point
        }
    }
}

impl Lambertian {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self{ albedo: texture }
    }

    pub fn from_color(albedo: Vec3) -> Lambertian {
        Self::new(Rc::new(SolidColor::new(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit();

        // TODO: Why?
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }
        Some(ScatterRecord::new(
            &Ray::new(hit_record.point, scatter_direction), 
            self.albedo.color(hit_record.texture_coord, hit_record.point)))
    }

    fn color_emmited(&self, _: Vec2, _: Pt3) -> Color {
        Color::all(0.0)
    }
}