mod vec3;
mod vec2;
mod ray;
mod aabb;
mod camera;
mod materials;
mod hittable_objects;
mod textures;
mod perlin;
mod raytracer;

use std::{sync::Arc, path::Path};

use hittable_objects::{HittableList, XyRect, YzRect};
use materials::Material;
use rand::Rng;
use raytracer::{Scene, run_raytracer};
use textures::{CheckerTexture, NoiseTexture};

use crate::{materials::{Lambertian, DiffuseLight, Dielectric, Metal}, hittable_objects::{Hittable, AaBox, BhvNode, XzRect, RotateY, Translate, Sphere}, vec3::{Color, Pt3, Vec3}, textures::{SolidColor, ImageTexture}};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    run_raytracer(Path::new("out.ppm"), cornell_box(), 1.0);
}

fn balls_scene() -> Scene {
    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9)))
    ))));
    world.objects.push(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    let mut balls = HittableList::new();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f32 = rand::thread_rng().gen();
            let center = Vec3::new(
                (a as f32) + 0.9 * rand::thread_rng().gen::<f32>(), 
                0.2, 
                b as f32 + 0.9 * rand::thread_rng().gen::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = 
                if choose_material < 0.8 {
                    let albedo = Vec3::new_random() * Vec3::new_random();
                    Arc::new(Lambertian::from_color(albedo))
                } else if choose_material < 0.95 {
                    let albedo = Vec3::new_random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    Arc::new(Metal::new(&albedo, fuzz))
                } else {
                    Arc::new(Dielectric::new(1.5))
                };
                balls.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    world.add(Arc::new(BhvNode::from_hittable_list(&balls)));

    let material = Arc::new(Metal::new(&Vec3::new(0.7, 0.5, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material)));
    
    let material = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material)));

    let earth_texture = Arc::new(ImageTexture::from_file(Path::new("earthmap.jpg")));
    let material: Arc<dyn Material> = Arc::new(Lambertian::new(earth_texture));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material)));
    
    Scene { 
        objects: Arc::new(world),
        look_from: Vec3::new(13.0, 2.0, 3.0), 
        look_at: Vec3::new(0.0, 0.0, 0.0),
        vertical_fov: 20.0f32.to_radians(), 
        aperture: 0.1,
        background_color: Vec3::new(0.7, 0.8, 1.0),
        focus_distance: 10.0
    }
}

fn test_scene() -> Scene {
    let mut boxes1: Vec<Arc<dyn Hittable>> = Vec::new();
    let ground = Arc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));

    const BOXES_PER_SIDE: i32 = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let (i, j) = (i as f32, j as f32);
            let w = 100.0;
            let x0 = -1000.0 + i * w;
            let z0 = -1000.0 + j * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.push(Arc::new(AaBox::new(Pt3::new(x0,y0,z0), Pt3::new(x1, y1, z1), ground.clone())));
        }    
    }

    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    objects.push(Arc::new(BhvNode::new(&boxes1, 0, boxes1.len())));

    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::all(7.0)))));
    objects.push(Arc::new(XzRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light)));

    objects.push(Arc::new(Sphere::new(Pt3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dielectric::new(0.5)))));
    objects.push(Arc::new(Sphere::new(Pt3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0)))));

    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::from_file(Path::new("earthmap.jpg")))));
    objects.push(Arc::new(Sphere::new(Pt3::new(400.0,200.0,400.0), 100.0, emat)));
    // auto pertext = make_shared<noise_texture>(0.1);
    // objects.add(make_shared<sphere>(point3(220,280,300), 80, make_shared<lambertian>(pertext)));

    let mut balls: Vec<Arc<dyn Hittable>> = Vec::new();
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    const BALL_COUNT: i32 = 1000;
    for i in 0..BALL_COUNT {
        balls.push(Arc::new(Sphere::new(Pt3::new_random_in_range(0.0,165.0), 10.0, white.clone())));
    }

    objects.push(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BhvNode::new(&balls, 0, balls.len())), 15.0), 
        ), 
            Vec3::new(-100.0, 270.0, 395.0))
    ));

    Scene { 
        objects: Arc::new(BhvNode::new(&objects, 0, objects.len())),
        // objects: Box::new(boxes1),
        look_from: Vec3::new(478.0, 278.0, -600.0), 
        look_at: Vec3::new(278.0, 278.0, 0.0),
        vertical_fov: 40.0f32.to_radians(), 
        aperture: 0.1,
        background_color: Vec3::all(0.8),
        focus_distance: 10.0
    }
}

fn light_scene() -> Scene {
    let mut world = HittableList::new();

    let texture = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Color::all(0.5)))));
    world.objects.push(Arc::new(Sphere::new(Pt3::new(0.0 , -1000.0, 0.0), 1000.0, texture.clone())));
    world.objects.push(Arc::new(Sphere::new(Pt3::new(0.0 , 2.0, 0.0), 2.0, texture)));

    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::all(4.0)))));
    world.objects.push(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, light)));
    // world.objects.push(Arc::new(Sphere::new(Vec3::new(3.0, 1.0, -2.0), 1.0, light)));

    Scene { 
        objects: Arc::new(world),
        look_from: Vec3::new(26.0, 3.0, 6.0), 
        look_at: Vec3::new(0.0, 2.0, 0.0),
        vertical_fov: 20.0f32.to_radians(), 
        aperture: 0.1,
        background_color: Vec3::all(0.0),
        focus_distance: 10.0
    }
}

fn simple_scene() -> HittableList {
    let mut world= HittableList::new();

    let material_ground = Arc::new(Lambertian::from_color(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from_color(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(&Vec3::new(0.8, 0.6, 0.2), 0.0));

    world.objects.push(Arc::new(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, material_ground)));
    world.objects.push(Arc::new(Sphere::new(Vec3::new( 0.0, 0.0, -1.0),  0.5, material_center)));
    world.objects.push(Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.objects.push(Arc::new(Sphere::new(Vec3::new( 1.0,  0.0, -1.0),0.5, material_right)));
    world
}

fn noise_scene() -> HittableList {
    let mut world = HittableList::new();

    let material: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new())));
    world.objects.push(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, material.clone())));
    world.objects.push(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material)));

    world
}

fn earth_scene() -> Scene {
    let texture =  ImageTexture::from_file(Path::new("earthmap.jpg"));
    let material = Arc::new(Lambertian::new(Arc::new(texture)));
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, material);

    Scene { 
        objects: Arc::new(sphere),
        look_from: Vec3::new(13.0, 2.0, 3.0), 
        look_at: Vec3::new(0.0, 0.0, 0.0),
        vertical_fov: 20.0f32.to_radians(), 
        aperture: 0.1,
        background_color: Vec3::new(0.7, 0.8, 1.0),
        focus_distance: 10.0
    }
}

fn cornell_box() -> Scene {
    let mut objects = HittableList::new();

    let red   = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::all(0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Color::all(15.0)))));

    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objects.add(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.add(Arc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));


    let box1 = Arc::new(AaBox::new(Pt3::new(0.0, 0.0, 0.0), Pt3::new(165.0, 330.0, 165.0), white.clone()));
    let box1 = Arc::new(RotateY::new(box1, 15.0f32.to_radians()));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);
    
    let box2 = Arc::new(AaBox::new(Pt3::new(0.0, 0.0, 0.0), Pt3::all(165.0), white.clone()));
    let box2 = Arc::new(RotateY::new(box2, -18.0f32.to_radians()));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);
    
    Scene { 
        objects: Arc::new(objects),
        look_from: Vec3::new(278.0, 278.0, -800.0), 
        look_at: Vec3::new(278.0, 278.0, 0.0),
        vertical_fov: 40.0f32.to_radians(), 
        aperture: 0.0,
        background_color: Vec3::all(0.0),
        focus_distance: 10.0
    }
}