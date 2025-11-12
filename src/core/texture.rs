use std::sync::Arc;

use crate::utils::color::Color;
use crate::utils::image::RtwImage;
use crate::utils::interval::Interval;
use crate::utils::perlin::Perlin;
use crate::utils::vec3::Point3;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    pub inv_scale: f64,
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            odd,
            even,
        }
    }

    pub fn new_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(c1)),
            Arc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        // let sines = f64::sin(self.inv_scale * p.x())
        //     * f64::sin(self.inv_scale * p.y())
        //     * f64::sin(self.inv_scale * p.z());
        // if sines < 0.0 {
        //     self.odd.value(u, v, p)
        // } else {
        //     self.even.value(u, v, p)
        // }
        let x_int = (self.inv_scale * p.x()) as i32;
        let y_int = (self.inv_scale * p.y()) as i32;
        let z_int = (self.inv_scale * p.z()) as i32;
        let is_odd = (x_int + y_int + z_int) % 2 == 1;
        if is_odd {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    pub image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: impl AsRef<str>) -> Self {
        Self {
            image: RtwImage::from_file(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        if self.image.width == 0 || self.image.height == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);
        let i = (u * self.image.width as f64) as i32;
        let j = (v * self.image.height as f64) as i32;
        let pixel = self.image.pixel_data(i, j);

        Color::new(
            pixel[0] as f64 / 255.0,
            pixel[1] as f64 / 255.0,
            pixel[2] as f64 / 255.0,
        )
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        // Color::new(1.0, 1.0, 1.0) * self.noise.turbulence(&(self.scale * p), 7)
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turbulence(&p, 7)))
    }
}
