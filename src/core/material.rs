use std::sync::Arc;

use super::hittable::HitRecord;
use super::ray::Ray;
use super::texture::{SolidColor, Texture};
use crate::utils::color::Color;
use crate::utils::vec3::{Point3, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

pub struct Lambertian {
    pub tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
    pub fn new_color(c: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(c)))
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction, r_in.ts);
        let attenuation = self.tex.value(rec.uv.0, rec.uv.1, &rec.p);
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
        let reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        let scattered = Ray::new(rec.p, reflected, r_in.ts);
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.dir.unit_vector();

        let cos_theta = (-unit_direction.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > rand::random::<f64>() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, ri)
            };
        let scattered = Ray::new(rec.p, direction, r_in.ts);
        Some((attenuation, scattered))
    }
}

pub struct DiffuseLight {
    pub tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn new_color(c: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(c)))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    pub tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
    pub fn new_color(c: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(c)))
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(rec.p, Vec3::random_unit_vector(), r_in.ts);
        let attenuation = self.tex.value(rec.uv.0, rec.uv.1, &rec.p);
        Some((attenuation, scattered))
    }
}
