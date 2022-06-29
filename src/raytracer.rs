use std::{fs::File, rc::Rc};
use std::io::prelude::*;
use std::path::Path;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::camera::Camera;
use crate::{materials::*};
use crate::hittable_objects::*;

use rand::Rng;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.objects.push(Box::new(Sphere::new(&Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f32 = rand::thread_rng().gen();
            let center = Vec3::new((a as f32) + 0.9 * rand::thread_rng().gen::<f32>(), 0.2, b as f32 + 0.9 * rand::thread_rng().gen::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> =
                    if choose_material < 0.8 {
                        let albedo = Vec3::new_random() * Vec3::new_random();
                        Rc::new(Lambertian::new(albedo))
                    } else if choose_material < 0.95 {
                        let albedo = Vec3::new_random_in_range(0.5, 1.0);
                        let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                        Rc::new(Metal::new(&albedo, fuzz))
                    } else {
                        Rc::new(Dielectric::new(1.5))
                    };
                world.objects.push(Box::new(Sphere::new(&center, 0.2, sphere_material)));
            }
        }
    }

    // Could juse use the same name for all materials
    let material1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.objects.push(Box::new(Sphere::new(&Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.objects.push(Box::new(Sphere::new(&Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3: Rc<dyn Material> = Rc::new(Metal::new(&Vec3::new(0.7, 0.5, 0.5), 0.0));
    world.objects.push(Box::new(Sphere::new(&Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn write_color(file: &mut File, color: Vec3, samples_per_pixel: i32) {
    // Because the human eye can't prercive as many dark colors the values need to be gamma corrected.
    // Without the gamma correction things are a lot darker because a lot more bits are allocated to storing darker colors.
    // Applying gamma correction by using sqrt. Gamma = 2 so f(x) = x^(1/Gamma) = sqrt(x)
    let color: Vec3 = (color / (samples_per_pixel as f32)).applied(f32::sqrt);

    file.write_fmt(format_args!(
        "{} {} {}\n",
        (color.x.clamp(0.0, 0.999) * 255.0) as i32,
        (color.y.clamp(0.0, 0.999) * 255.0) as i32,
        (color.z.clamp(0.0, 0.999) * 255.0) as i32
    )).unwrap();
}

fn ray_color(ray: &Ray, hittable: &Box<dyn Hittable>, bounces_left: i32) -> Vec3 {
    if bounces_left <= 0 {
        return Vec3::new_all(0.0);
    }

    // To account for floating point precision errors.
    const EPSILON: f32 = 0.001;
    match hittable.hit(ray, EPSILON, f32::INFINITY) {
        None => (),
        Some(hit_record) => {
            match hit_record.material.scatter(ray, &hit_record) {
                None => return Vec3::new_all(0.0),
                Some(scatter_record) => 
                    return scatter_record.attenuation * ray_color(&scatter_record.ray, hittable, bounces_left - 1)
            };
        }
    }
    
    let t = 0.5 * (ray.direction.normalized().y + 1.0);
    ((1.0 - t) * Vec3::new_all(1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
}

pub fn run_raytracer(out_path: &str) {
    let path = Path::new(out_path);
    
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create file {}: {}", path.display(), why),
        Ok(file) => file
    };

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32= 200;
    let image_height: i32 = ((image_width as f32) / aspect_ratio) as i32;
    let samples_per_pixel = 1;
    let max_bounces: i32 = 50;

    let mut world= HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(&Vec3::new(0.8, 0.6, 0.2), 0.0));

    world.objects.push(Box::new(Sphere::new(&Vec3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.objects.push(Box::new(Sphere::new(&Vec3::new( 0.0, 0.0, -1.0),  0.5, material_center)));
    world.objects.push(Box::new(Sphere::new(&Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.objects.push(Box::new(Sphere::new(&Vec3::new( 1.0,  0.0, -1.0),0.5, material_right)));

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(look_from, look_at, up, f32::to_radians(20.0), aspect_ratio, aperture, dist_to_focus);

    let hittable: Box<dyn Hittable> = Box::new(random_scene());

    file.write_fmt(format_args!("P3\n{} {}\n255\n", image_width, image_height)).unwrap();

    for y in (0..image_height).rev() {
        println!("{:.2}%", (1.0 - ((y as f32) / (image_height as f32))) * 100.0);
        for x in 0..image_width {
            let mut color = Vec3::new_all(0.0);
            for _ in 0..samples_per_pixel {
                // Sample random points around the pixel.
                let u: f32 = ((x as f32) + rand::thread_rng().gen::<f32>()) / ((image_width - 1) as f32);
                let v: f32 = ((y as f32) + rand::thread_rng().gen::<f32>()) / ((image_height - 1) as f32);
                color += ray_color(&camera.ray(u, v), &hittable, max_bounces);
            }

            write_color(&mut file, color, samples_per_pixel);
        }
    }
    println!("done");
}