use std::io::Write;

use super::hittable::Hittable;
use super::ray::Ray;
use crate::utils::color::{Color, write_color};
use crate::utils::interval::Interval;
use crate::utils::vec3::{Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,      // width / height
    pub image_width: u32,       // pixel width
    pub samples_per_pixel: u32, // blue-noise samples per pixel
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    // derived
    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::default(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,

            image_height: 1,
            pixel_samples_scale: 1.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            ..Default::default()
        }
    }

    pub fn render<W: Write>(&mut self, mut out: W, world: &dyn Hittable) -> std::io::Result<()> {
        self.initialize();

        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for s in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j, s);
                    pixel_color += &Self::ray_color(&r, self.max_depth, world);
                }
                let scaled = self.pixel_samples_scale * pixel_color;
                write_color(&mut out, &scaled)?;
            }
        }

        Ok(())
    }

    fn initialize(&mut self) {
        // compute integer height from aspect
        let mut h = (self.image_width as f64 / self.aspect_ratio) as i64;
        if h < 1 {
            h = 1;
        }
        self.image_height = h as u32;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // viewport dimensions
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = Vec3::cross(&self.vup, &self.w).unit_vector();
        self.v = Vec3::cross(&self.w, &self.u);

        // edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = -viewport_height * self.v;

        // per-pixel deltas
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // upper-left pixel center
        let viewport_upper_left =
            self.center - self.focus_dist * self.w - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        if let Some(rec) = world.hit(r, &Interval::new(0.001, f64::INFINITY)) {
            match rec.material.scatter(r, &rec) {
                Some((attenuation, scattered)) => {
                    return attenuation * Self::ray_color(&scattered, depth - 1, world);
                }
                None => return Color::default(),
            }
        }

        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u32, j: u32, sample_idx: u32) -> Ray {
        let offset = self.sample_square_blue(i, j, sample_idx);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.1) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_ts = rand::random::<f64>();
        Ray::new(ray_origin, ray_direction, ray_ts)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // return a sample point on the defocus disk
        let p = Vec3::random_in_unit_disk();
        p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v + self.center
    }

    // Returns a blue-noise-like 2D offset in [-0.5, 0.5)^2 using a small tiled mask.
    fn sample_square_blue(&self, i: u32, j: u32, sample_idx: u32) -> (f64, f64) {
        const BN8X8_A: [u8; 64] = [
            32, 0, 48, 16, 56, 24, 40, 8, 12, 44, 28, 60, 4, 36, 20, 52, 51, 19, 35, 3, 43, 11, 59,
            27, 23, 55, 7, 39, 31, 63, 15, 47, 34, 2, 50, 18, 58, 26, 42, 10, 6, 38, 22, 54, 14,
            46, 30, 62, 49, 17, 33, 1, 41, 9, 57, 25, 21, 53, 5, 37, 13, 45, 29, 61,
        ];
        const BN8X8_B: [u8; 64] = [
            0, 32, 16, 48, 8, 40, 24, 56, 44, 12, 60, 28, 36, 4, 52, 20, 19, 51, 3, 35, 11, 43, 27,
            59, 55, 23, 39, 7, 63, 31, 47, 15, 2, 34, 18, 50, 26, 58, 10, 42, 38, 6, 54, 22, 46,
            14, 62, 30, 17, 49, 1, 33, 9, 41, 25, 57, 53, 21, 37, 5, 45, 13, 61, 29,
        ];
        let ix = (i & 7) as usize;
        let jy = (j & 7) as usize;
        let base = jy * 8 + ix;
        let idx_x = ((base as u32 + sample_idx.wrapping_mul(73)) & 63) as usize;
        let idx_y = ((base as u32 + sample_idx.wrapping_mul(97)) & 63) as usize;
        let vx = BN8X8_A[idx_x] as f64;
        let vy = BN8X8_B[idx_y] as f64;
        // Map 0..63 to [-0.5, 0.5)
        let dx = (vx + 0.5) / 64.0 - 0.5;
        let dy = (vy + 0.5) / 64.0 - 0.5;
        (dx, dy)
    }
}
