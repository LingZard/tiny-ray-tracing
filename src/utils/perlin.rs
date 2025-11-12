use crate::utils::vec3::{Point3, Vec3};

pub struct Perlin {
    pub randvec: [Vec3; Self::POINT_COUNT],
    pub perm_x: [u32; Self::POINT_COUNT],
    pub perm_y: [u32; Self::POINT_COUNT],
    pub perm_z: [u32; Self::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut randvec = [Vec3::new(0.0, 0.0, 0.0); Self::POINT_COUNT];
        for i in 0..Self::POINT_COUNT {
            randvec[i] = Vec3::random(-1.0, 1.0).unit_vector();
        }

        let mut perm_x = [0_u32; Self::POINT_COUNT];
        let mut perm_y = [0_u32; Self::POINT_COUNT];
        let mut perm_z = [0_u32; Self::POINT_COUNT];
        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idx = (self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize;
                    c[di][dj][dk] = self.randvec[idx];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        return accum.abs();
    }

    fn perlin_generate_perm(p: &mut [u32; Self::POINT_COUNT]) {
        for i in 0..Self::POINT_COUNT {
            p[i] = i as u32;
        }

        Self::shuffle_in_place(p, Self::POINT_COUNT);
    }

    fn shuffle_in_place(p: &mut [u32; Self::POINT_COUNT], n: usize) {
        for i in (0..n).rev() {
            let target = rand::random::<u32>() % (i as u32 + 1);
            p.swap(i, target as usize);
        }
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermite smoothing per author (can be replaced with fade 6t^5-15t^4+10t^3)
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0_f64;
        for i in 0..2 {
            let iu = i as f64;
            let sx = iu * uu + (1.0 - iu) * (1.0 - uu);
            for j in 0..2 {
                let jv = j as f64;
                let sy = jv * vv + (1.0 - jv) * (1.0 - vv);
                for k in 0..2 {
                    let kw = k as f64;
                    let sz = kw * ww + (1.0 - kw) * (1.0 - ww);
                    let weight_v = Vec3::new(u - iu, v - jv, w - kw);
                    accum += sx * sy * sz * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }
}
