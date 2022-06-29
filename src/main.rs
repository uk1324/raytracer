mod vec3;
mod ray;
mod camera;
mod materials;
mod hittable_objects;
mod random;

mod raytracer;
fn main() {
    raytracer::run_raytracer("out.ppm");
}
