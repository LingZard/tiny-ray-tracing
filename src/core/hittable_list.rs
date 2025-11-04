use std::sync::Arc;

use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use crate::utils::aabb::Aabb;
use crate::utils::interval::Interval;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    pub aabb: Aabb,
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            aabb: Aabb::default(),
        }
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.aabb = self.aabb.merge(&object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;
        let mut temp_interval = Interval::new(ray_t.min, closest_so_far);

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, &temp_interval) {
                closest_so_far = temp_rec.t;
                temp_interval.max = closest_so_far;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb
    }
}
