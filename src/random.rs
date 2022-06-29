use crate::vec3::Vec3;

pub fn random_point_in_unit_sphere() -> Vec3 {
    loop {
        let point = Vec3::new_random_in_range(-1.0, 1.0);
        if point.length_squared() < 1.0 {
            return point
        }
    }
}

pub fn random_point_in_unit_sphere_normalized() -> Vec3 {
    // This models lambertian reflection more closely.
    random_point_in_unit_sphere().normalized()
}

pub fn random_point_in_hemisphere(normal: Vec3) -> Vec3 {
    let point_in_unit_sphere = random_point_in_unit_sphere();
    // Is in the same hemisphere as normal.
    if Vec3::dot(point_in_unit_sphere, normal) > 0.0 {
        point_in_unit_sphere
    }
    else {
        -point_in_unit_sphere
    }
}