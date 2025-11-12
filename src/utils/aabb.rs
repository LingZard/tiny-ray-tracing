use std::ops::Add;

use crate::core::ray::Ray;
use crate::utils::interval::Interval;
use crate::utils::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimum();
        aabb
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let (xmin, xmax) = (a[0].min(b[0]), a[0].max(b[0]));
        let (ymin, ymax) = (a[1].min(b[1]), a[1].max(b[1]));
        let (zmin, zmax) = (a[2].min(b[2]), a[2].max(b[2]));

        let mut aabb = Self {
            x: Interval::new(xmin, xmax),
            y: Interval::new(ymin, ymax),
            z: Interval::new(zmin, zmax),
        };
        aabb.pad_to_minimum();
        aabb
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Aabb>,
    {
        let mut it = iter.into_iter();
        if let Some(first) = it.next() {
            let mut acc = first;
            for b in it {
                acc = acc.merge(&b);
            }
            acc
        } else {
            Aabb::default()
        }
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn universe() -> Self {
        Self {
            x: Interval::universe(),
            y: Interval::universe(),
            z: Interval::universe(),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let mut t_range = ray_t;

        for (axis, ax) in [&self.x, &self.y, &self.z].into_iter().enumerate() {
            // adinv: 避免出现 0.0 / 0.0 = NaN, 这里保证了如果 r.dir[axis] == 0.0, adinv = +-infinity
            let adinv = 1.0 / r.dir[axis];

            let t0 = (ax.min - r.orig[axis]) * adinv;
            let t1 = (ax.max - r.orig[axis]) * adinv;

            let t_near = t0.min(t1);
            let t_far = t0.max(t1);

            t_range.min = t_range.min.max(t_near);
            t_range.max = t_range.max.min(t_far);

            if t_range.max <= t_range.min {
                return false;
            }
        }

        true
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            x: self.x.merge(&other.x),
            y: self.y.merge(&other.y),
            z: self.z.merge(&other.z),
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() && self.x.size() > self.z.size() {
            0
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    pub fn pad_to_minimum(&mut self) {
        let delta = 0.0001;
        for axis in [&mut self.x, &mut self.y, &mut self.z] {
            if axis.size() < delta {
                axis.expand(delta);
            }
        }
    }
}

impl<'a> Add<&'a Vec3> for Aabb {
    type Output = Self;

    fn add(self, other: &'a Vec3) -> Self {
        Self {
            x: self.x + &other.x(),
            y: self.y + &other.y(),
            z: self.z + &other.z(),
        }
    }
}

impl<'a> Add<&'a Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, other: &'a Aabb) -> Aabb {
        Aabb {
            x: other.x + &self.x(),
            y: other.y + &self.y(),
            z: other.z + &self.z(),
        }
    }
}
