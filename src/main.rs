mod core;
mod utils;

use std::sync::Arc;

use crate::core::camera::Camera;
use crate::core::hittable::{Hittable, RotateY, Translate};
use crate::core::hittable_list::HittableList;
use crate::core::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::core::quad::{Quad, make_box};
use crate::core::sphere::Sphere;
use crate::core::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::utils::bvh::BvhNode;
use crate::utils::color::Color;
use crate::utils::timer::Timer;
use crate::utils::vec3::*;

fn bouncing_spheres() {
    // 记录: bvh随机选轴约 19.8s, 选最长轴约 17.1s, 直接渲染约61.1s，构建bvh在0.4ms这个级别
    // World
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere: Arc<dyn Hittable> = if choose_mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let center1 = center + Vec3::new(0.0, rand::random::<f64>() * 0.5, 0.0);
                    Arc::new(Sphere::new_moving(
                        center,
                        center1,
                        0.2,
                        Arc::new(Lambertian::new_color(albedo)),
                    ))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz))))
                } else {
                    Arc::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))))
                };
                world.add(sphere);
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut timer = Timer::new();
    timer.start();
    let world_bvh = BvhNode::new_from_list(&mut world);
    timer.stop();
    println!("BVH build time: {} ms", timer.elapsed_ms_3dp());

    // Camera
    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
        Color::new(0.7, 0.8, 1.0),
    );

    // Render to file
    let file = std::fs::File::create("image-1.ppm").unwrap();
    timer.start();
    cam.render(file, &world_bvh).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image-1.ppm");
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::new(0.7, 0.8, 1.0),
    );

    let file = std::fs::File::create("image-2.ppm").unwrap();
    let mut timer = Timer::new();
    timer.start();
    cam.render(file, &world).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image-2.ppm");
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Point3::new(0.0, 0.0, 12.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::new(0.7, 0.8, 1.0),
    );

    let file = std::fs::File::create("image-3.ppm").unwrap();
    let mut timer = Timer::new();
    timer.start();
    cam.render(file, &*globe).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image-3.ppm");
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::new(0.7, 0.8, 1.0),
    );

    let file = std::fs::File::create("image-4.ppm").unwrap();
    let mut timer = Timer::new();
    timer.start();
    cam.render(file, &world).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image-4.ppm");
}

fn quads() {
    let mut world = HittableList::new();

    // Materials
    let left_red = Arc::new(Lambertian::new_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new_color(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let mut cam = Camera::new(
        1.0,
        400,
        100,
        50,
        80.0,
        Point3::new(0.0, 0.0, 9.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::new(0.7, 0.8, 1.0),
    );

    let file = std::fs::File::create("image-5.ppm").unwrap();
    let mut timer = Timer::new();
    timer.start();
    cam.render(file, &world).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image-5.ppm");
}

fn simple_light() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let mut cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::new(0.0, 0.0, 0.0),
    );

    let file = std::fs::File::create("image-6.ppm").unwrap();
    let mut timer = Timer::new();
    timer.start();
    cam.render(file, &world).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image-6.ppm");
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    // Walls and light
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Inner boxes
    let mut box1: Arc<dyn Hittable> = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Arc<dyn Hittable> = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::new(
        1.0,
        600,
        200,
        50,
        40.0,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
        Color::new(0.0, 0.0, 0.0),
    );

    let file = std::fs::File::create("image-7.ppm").unwrap();
    let mut timer = Timer::new();
    timer.start();
    cam.render(file, &world).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image-7.ppm");
}
fn main() {
    let scene = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(1);

    match scene {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => bouncing_spheres(),
    }
}
