mod vec3;
mod vec2;
mod ray;
mod aabb;
mod camera;
mod materials;
mod hittable_objects;
mod textures;
mod random;
mod perlin;

mod raytracer;
fn main() {
    raytracer::run_raytracer("out.ppm");
}
