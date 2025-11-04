use std::sync::Arc;

use super::material::Material;
use super::ray::Ray;
use crate::utils::aabb::Aabb;
use crate::utils::interval::Interval;
use crate::utils::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, material: Arc<dyn Material>) -> Self {
        Self {
            p,
            normal: Vec3::default(),
            t,
            front_face: false,
            material,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
