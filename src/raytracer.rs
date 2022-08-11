use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;
use std::{fs::File};
use std::io::{prelude::*, BufWriter};
use std::path::Path;

use crate::vec3::{Vec3, Color};
use crate::ray::Ray;
use crate::camera::Camera;
use crate::materials::*;
use crate::hittable_objects::*;

use rand::Rng;

pub struct Scene {
    pub objects: Arc<dyn Hittable>,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vertical_fov: f32,
    pub aperture: f32,
    pub background_color: Color,
    pub focus_distance: f32,
}

pub fn run_raytracer(out_path: &Path, scene: Scene, aspect_ratio: f32) {
    let mut file = BufWriter::new(match File::create(&out_path) {
        Err(why) => {
            println!("couldn't create file {}: {}", out_path.display(), why);
            std::process::exit(-1);
        },
        Ok(file) => file
    });

    let image_width: usize = 400;
    let image_height: usize = ((image_width as f32) / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_bounces: usize = 50;

    let scene = Arc::new(scene);

    let camera = Arc::new(Camera::new(
        scene.look_from, 
        scene.look_at, 
        scene.vertical_fov, 
        aspect_ratio, 
        scene.aperture, 
        scene.focus_distance));
        
    let start = Instant::now(); 

    let thread_count = num_cpus::get();
    let rows_per_thread = image_height / thread_count;
    let last_thread_rows = rows_per_thread + image_height % thread_count;

    // Rust doesn't have any std utlity to share ownership of a vector between slices so each thead has to allocate it's own buffer.
    // Everything shared between threads has to use Arc because the shared data might go out of scope.
    // This can happen here even though the threads are joined right after they are created. The main thread might panic due to 
    // an out of memory or other error which starts unwinding and frees the resources.
    let mut threads = Vec::new();
    let finished_rows = Arc::new(AtomicUsize::new(0));

    for i in 0..(thread_count - 1) {
        let finished_rows = finished_rows.clone();
        let camera = camera.clone();
        let scene = scene.clone();
        threads.push(std::thread::spawn(move || {
            run(
                image_width, 
                image_height, 
                i * rows_per_thread, 
                rows_per_thread, 
                samples_per_pixel, 
                max_bounces, 
                &camera, 
                &scene, 
                &finished_rows)
        }));   
    }

    let pixels = run(
        image_width, 
        image_height, 
        (thread_count - 1) * rows_per_thread, 
        last_thread_rows, 
        samples_per_pixel, 
        max_bounces, 
        &camera, 
        &scene, 
        &finished_rows);

    write!(file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();
    for thread in threads {
        let result = thread.join().unwrap();
        output_pixels(&mut file, &result);
    }
    output_pixels(&mut file, &pixels);

    println!("took {}ms", start.elapsed().as_millis());
}

fn run(
    image_width: usize, 
    image_height: usize, 
    starting_y: usize,
    slice_height: usize, 
    samples_per_pixel: usize, 
    max_bounces: usize, 
    camera: &Arc<Camera>, 
    scene: &Arc<Scene>, 
    finished_rows: &Arc<AtomicUsize>
) -> Vec<u8> {
    let mut pixels =  vec![0u8; slice_height * image_width * 3];
    for y_i in 0..slice_height {
        let y = y_i as f32 + starting_y as f32;
        for x_i in 0..image_width {
            let x = x_i as f32;
            let mut color = Color::all(0.0);
            for _ in 0..samples_per_pixel {
                // TODO: Try euler integration.
                let u: f32 = (x + rand::thread_rng().gen::<f32>()) / ((image_width - 1) as f32);
                let v: f32 = (y + rand::thread_rng().gen::<f32>()) / ((image_height - 1) as f32);
                color += ray_color(&camera.ray(u, v), &scene.objects, scene.background_color, max_bounces);
            }
            // Because eyes can't prercive as many dark colors the values need to be gamma corrected.
            // Without the gamma correction things are a lot darker because a lot more bits are allocated to storing darker colors.
            // Applying gamma correction by using sqrt. gamma = 2 so f(x) = x^(1/gamma) = sqrt(x)
            let color = (color / (samples_per_pixel as f32)).applied(f32::sqrt);
            let location = (y_i * image_width + x_i) * 3;
            pixels[location] = (color.x * 255.0) as u8;
            pixels[location + 1] = (color.y * 255.0) as u8;
            pixels[location + 2] = (color.z * 255.0) as u8;
        }
        let finished = finished_rows.fetch_add(1, Ordering::Relaxed);
        println!("{:.2}%", ((finished as f32 / image_height as f32)) * 100.0);
    }
    pixels
}

fn ray_color(ray: &Ray, hittable: &Arc<dyn Hittable>, background_color: Color, bounces_left: usize) -> Color {
    if bounces_left <= 0 {
        return Color::all(0.0);
    }

    // Because of floating point rounding a ray might end up inside the sphere instead of on it which causes the ray to hit the sphere again.
    // To fix this hits very near 0 are ignored.
    const EPSILON: f32 = 0.001;
    match &hittable.hit(ray, EPSILON, f32::INFINITY) {
        None => background_color,
        Some(record @ HitRecord { point, texture_coord, material, .. }) => {
            let emmited = material.color_emmited(*texture_coord, *point);
            match material.scatter(ray, &record) {
                None => emmited,
                Some(ScatterRecord { ray, attenuation }) => 
                    emmited + attenuation * ray_color(&ray, hittable, background_color, bounces_left - 1)
            }
        }
    }
}

fn output_pixels(file: &mut BufWriter<File>, pixels: &[u8]) {
    for i in (0..pixels.len()).step_by(3) {
        let (r, g, b) = (pixels[i], pixels[i + 1], pixels[i + 2]);
        write!(file, "{} {} {}\n", r, g, b).unwrap();
    }
}