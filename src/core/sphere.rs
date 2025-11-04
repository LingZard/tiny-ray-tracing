use std::sync::Arc;

use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use crate::utils::aabb::Aabb;
use crate::utils::interval::Interval;
use crate::utils::vec3::*;

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub material: Arc<dyn Material>,
    pub aabb: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(center, Vec3::default(), 0.0),
            radius: radius.max(0.0),
            material,
            aabb: Aabb::from_points(center - &rvec, center + &rvec),
        }
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center0 - &rvec, center0 + &rvec);
        let box2 = Aabb::from_points(center1 - &rvec, center1 + &rvec);
        Self {
            center: Ray::new(center0, center1 - center0, 0.0),
            radius: radius.max(0.0),
            material,
            aabb: box1.merge(&box2),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.ts);
        let oc = current_center - &r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - current_center) / self.radius;
        let mut rec = HitRecord::new(p, t, self.material.clone());
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
