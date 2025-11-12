use std::sync::Arc;

use crate::core::hittable::{HitRecord, Hittable};
use crate::core::material::{Isotropic, Material};
use crate::core::ray::Ray;
use crate::core::texture::{SolidColor, Texture};
use crate::utils::aabb::Aabb;
use crate::utils::color::Color;
use crate::utils::interval::Interval;
use crate::utils::vec3::Vec3;

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub neg_inv_density: f64,
    pub phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(tex)),
        }
    }

    pub fn new_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self::new(boundary, density, Arc::new(SolidColor::new(albedo)))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Find the entry and exit points with the boundary
        let mut rec1 = match self.boundary.hit(r, &Interval::universe()) {
            Some(rec) => rec,
            None => return None,
        };
        let mut rec2 = match self
            .boundary
            .hit(r, &Interval::new(rec1.t + 0.0001, f64::INFINITY))
        {
            Some(rec) => rec,
            None => return None,
        };

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (rand::random::<f64>()).ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        let mut rec = HitRecord::new(p, t, (0.0, 0.0), self.phase_function.clone());
        // Arbitrary normal and front_face for volume events
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
