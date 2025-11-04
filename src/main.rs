mod core;
mod utils;

use std::sync::Arc;

use crate::core::camera::Camera;
use crate::core::hittable::Hittable;
use crate::core::hittable_list::HittableList;
use crate::core::material::{Dielectric, Lambertian, Metal};
use crate::core::sphere::Sphere;
use crate::utils::bvh::BvhNode;
use crate::utils::color::Color;
use crate::utils::timer::Timer;
use crate::utils::vec3::*;

fn main() {
    // 记录: bvh随机选轴约 19.8s, 选最长轴约 17.1s, 直接渲染约61.1s，构建bvh在0.4ms这个级别
    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
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
                        Arc::new(Lambertian::new(albedo)),
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

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
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
    );

    // Render to file
    let file = std::fs::File::create("image.ppm").unwrap();
    timer.start();
    cam.render(file, &world_bvh).unwrap();
    timer.stop();
    let render_ms = timer.elapsed_ms();
    println!(
        "Render time: {:.3} ms ({:.3} s)",
        render_ms,
        render_ms / 1000.0
    );
    println!("Image saved to image.ppm");
}
