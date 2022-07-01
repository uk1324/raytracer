use crate::materials::{Material, ScatterRecord};
use crate::vec2::Vec2;
use crate::vec3::{Vec3, Pt3, Color};
use crate::ray::Ray;
use crate::hittable_objects::HitRecord;
use crate::random::random_point_in_unit_sphere;

pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(color: &Vec3, fuzz: f32) -> Metal {
        Metal{ albedo: *color, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(ray.direction, hit_record.normal);
        if Vec3::dot(reflected, hit_record.normal) > 0.0 {
            Some(ScatterRecord::new(&Ray::new(hit_record.point, reflected + (self.fuzz * random_point_in_unit_sphere())), self.albedo))
        } else {
            None
        }
    }

    fn color_emmited(&self, _: Vec2, _: Pt3) -> Color {
        Color::all(0.0)
    }
}
