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
    pub uv: (f64, f64),
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, uv: (f64, f64), material: Arc<dyn Material>) -> Self {
        Self {
            p,
            normal: Vec3::default(),
            t,
            uv,
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

pub struct Translate {
    pub obj: Arc<dyn Hittable>,
    pub offset: Vec3,
    pub aabb: Aabb,
}

impl Translate {
    pub fn new(obj: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let aabb = obj.bounding_box() + &offset;
        Self { obj, offset, aabb }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(r.orig - self.offset, r.dir, r.ts);
        if let Some(mut rec) = self.obj.hit(&offset_r, ray_t) {
            rec.p = rec.p + self.offset;
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}

pub struct RotateY {
    pub obj: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub aabb: Aabb,
}

impl RotateY {
    pub fn new(obj: Arc<dyn Hittable>, angle_degrees: f64) -> Self {
        let radians = angle_degrees.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = obj.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            let x = if i == 1 { bbox.x.max } else { bbox.x.min };
            for j in 0..2 {
                let y = if j == 1 { bbox.y.max } else { bbox.y.min };
                for k in 0..2 {
                    let z = if k == 1 { bbox.z.max } else { bbox.z.min };
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Point3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let aabb = Aabb::from_points(min, max);
        Self {
            obj,
            sin_theta,
            cos_theta,
            aabb,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Rotate ray into object space (inverse rotation)
        let origin = Point3::new(
            self.cos_theta * r.orig.x() - self.sin_theta * r.orig.z(),
            r.orig.y(),
            self.sin_theta * r.orig.x() + self.cos_theta * r.orig.z(),
        );
        let direction = Vec3::new(
            self.cos_theta * r.dir.x() - self.sin_theta * r.dir.z(),
            r.dir.y(),
            self.sin_theta * r.dir.x() + self.cos_theta * r.dir.z(),
        );
        let rotated_r = Ray::new(origin, direction, r.ts);

        if let Some(mut rec0) = self.obj.hit(&rotated_r, ray_t) {
            // Rotate intersection back to world space
            let p = Point3::new(
                self.cos_theta * rec0.p.x() + self.sin_theta * rec0.p.z(),
                rec0.p.y(),
                -self.sin_theta * rec0.p.x() + self.cos_theta * rec0.p.z(),
            );
            let normal = Vec3::new(
                self.cos_theta * rec0.normal.x() + self.sin_theta * rec0.normal.z(),
                rec0.normal.y(),
                -self.sin_theta * rec0.normal.x() + self.cos_theta * rec0.normal.z(),
            );
            rec0.p = p;
            rec0.normal = normal;
            Some(rec0)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
