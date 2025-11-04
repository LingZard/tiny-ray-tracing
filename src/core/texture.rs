use crate::utils::color::Color;
use crate::utils::vec3::Point3;

pub trait Texture {
    fn value(u: f64, v: f64, p: &Point3) -> Color;
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
