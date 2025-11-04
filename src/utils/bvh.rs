use std::sync::Arc;

use crate::core::hittable::{HitRecord, Hittable};
use crate::core::hittable_list::HittableList;
use crate::core::ray::Ray;
use crate::utils::aabb::Aabb;
use crate::utils::interval::Interval;

// TODO: upgrade to index-based tree

pub struct BvhNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new_from_list(list: &mut HittableList) -> Self {
        Self::build(&mut list.objects)
    }

    pub fn build(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let mut node = BvhNode {
            left: None,
            right: None,
            bbox: Aabb::default(),
        };

        let len = objects.len();
        if len == 0 {
            return node;
        }

        let bbox = Aabb::from_iter(objects.iter().map(|o| o.bounding_box()));

        let axis = bbox.longest_axis();
        let comparator: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> std::cmp::Ordering =
            match axis {
                0 => Self::box_x_compare,
                1 => Self::box_y_compare,
                2 => Self::box_z_compare,
                _ => unreachable!(),
            };

        if len == 1 {
            node.left = Some(objects[0].clone());
            node.right = Some(objects[0].clone());
        } else if len == 2 {
            node.left = Some(objects[0].clone());
            node.right = Some(objects[1].clone());
        } else {
            objects.sort_by(|a, b| comparator(a, b));
            let mid = len / 2;
            let (left_slice, right_slice) = objects.split_at_mut(mid);
            node.left = Some(Arc::new(BvhNode::build(left_slice)));
            node.right = Some(Arc::new(BvhNode::build(right_slice)));
        }

        let lb = node.left.as_ref().unwrap().bounding_box();
        let rb = node.right.as_ref().unwrap().bounding_box();
        node.bbox = lb.merge(&rb);
        node
    }

    fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let a_min = a.bounding_box().axis_interval(axis).min;
        let b_min = b.bounding_box().axis_interval(axis).min;
        a_min
            .partial_cmp(&b_min)
            .unwrap_or(std::cmp::Ordering::Equal)
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, Interval::new(ray_t.min, ray_t.max)) {
            return None;
        }

        let mut best: Option<HitRecord> = None;
        let mut tmax = ray_t.max;

        if let Some(left) = &self.left {
            if let Some(h) = left.hit(r, ray_t) {
                tmax = h.t;
                best = Some(h);
            }
        }

        if let Some(right) = &self.right {
            let right_interval = Interval::new(ray_t.min, tmax);
            if let Some(h) = right.hit(r, &right_interval) {
                match &best {
                    Some(b) if b.t <= h.t => {}
                    _ => best = Some(h),
                }
            }
        }

        best
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
