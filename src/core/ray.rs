use crate::utils::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub ts: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, timestamp: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            ts: timestamp,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        Vec3::new(
            self.orig.e[0] + t * self.dir.e[0],
            self.orig.e[1] + t * self.dir.e[1],
            self.orig.e[2] + t * self.dir.e[2],
        )
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Point3::default(),
            dir: Vec3::default(),
            ts: 0.0,
        }
    }
}
