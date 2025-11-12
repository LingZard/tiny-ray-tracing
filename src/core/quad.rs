use std::sync::Arc;

use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use crate::core::hittable_list::HittableList;
use crate::utils::aabb::Aabb;
use crate::utils::interval::Interval;
use crate::utils::vec3::{Point3, Vec3};

pub struct Quad {
    pub p0: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub material: Arc<dyn Material>,
    pub aabb: Aabb,
    pub normal: Vec3,
    pub d: f64,
}

impl Quad {
    pub fn new(p0: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let normal = u.cross(&v).unit_vector();
        let d = normal.dot(&p0);
        let mut quad = Self {
            p0,
            u,
            v,
            material,
            aabb: Aabb::default(),
            normal,
            d,
        };
        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let bbox_diag1 = Aabb::from_points(self.p0, self.p0 + self.u + self.v);
        let bbox_diag2 = Aabb::from_points(self.p0 + self.v, self.p0 + self.u);
        self.aabb = bbox_diag1.merge(&bbox_diag2);
    }

    fn is_interior(&self, alpha: f64, beta: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(alpha) || !unit_interval.contains(beta) {
            return false;
        }

        return true;
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.dir);
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&r.orig)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let mut hit_record = HitRecord::new(intersection, t, (0.0, 0.0), self.material.clone());

        let qp = intersection - self.p0;
        let alpha =
            (self.v.cross(&qp)).dot(&self.normal) / (self.v.cross(&self.u)).dot(&self.normal);
        let beta =
            (qp.cross(&self.u)).dot(&self.normal) / (self.v.cross(&self.u)).dot(&self.normal);

        if !self.is_interior(alpha, beta) {
            return None;
        }

        hit_record.uv = (alpha, beta);
        hit_record.set_face_normal(r, &self.normal);
        Some(hit_record)
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}

pub fn make_box(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let mut sides = HittableList::new();
    let min = Point3::new(p0.x().min(p1.x()), p0.y().min(p1.y()), p0.z().min(p1.z()));
    let max = Point3::new(p0.x().max(p1.x()), p0.y().max(p1.y()), p0.z().max(p1.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    // front
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        material.clone(),
    )));
    // right
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        material.clone(),
    )));
    // back
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        material.clone(),
    )));
    // left
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        material.clone(),
    )));
    // top
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        material.clone(),
    )));
    // bottom
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        material,
    )));

    Arc::new(sides)
}
