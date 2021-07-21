use crate::{
    rtweekend::{random_double2, random_int},
    Vec3,
};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    //pub ranfloat: [f64;POINT_COUNT],
    pub ranvec: [Vec3; POINT_COUNT],
    pub perm_x: [i32; POINT_COUNT],
    pub perm_y: [i32; POINT_COUNT],
    pub perm_z: [i32; POINT_COUNT],
}
impl Perlin {
    pub fn new() -> Self {
        //let mut ran_fl = [0.0;POINT_COUNT];
        let mut ranv = [Vec3::zero(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            //ran_fl[i] = random_double(0.0, 100.0);
            ranv[i] = Vec3::unit(Vec3::new(
                random_double2(-1.0, 1.0),
                random_double2(-1.0, 1.0),
                random_double2(-1.0, 1.0),
            ))
        }
        let mut x = [0; POINT_COUNT];
        Perlin::perlin_generate_perm(&mut x);
        let mut y = [0; POINT_COUNT];
        Perlin::perlin_generate_perm(&mut y);
        let mut z = [0; POINT_COUNT];
        Perlin::perlin_generate_perm(&mut z);

        Self {
            ranvec: ranv,
            perm_x: x,
            perm_y: y,
            perm_z: z,
        }
    }

    pub fn perlin_generate_perm(p: &mut [i32; POINT_COUNT]) {
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        Perlin::permute(p, POINT_COUNT);
    }

    pub fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
        for i in n - 1..0 {
            let target = random_int(0, i as i32);
            let tmp = p[i];
            p[i] = p[target as usize];
            p[target as usize] = tmp;
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] =
                        self.ranvec[(self.perm_x[((i as i32 + di as i32) & 255) as usize] as i32
                            ^ self.perm_y[((j as i32 + dj as i32) & 255) as usize] as i32
                            ^ self.perm_z[((k as i32 + dk as i32) & 255) as usize] as i32)
                            as usize]
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut acc = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * Vec3::dot(
                            c[i][j][k],
                            Vec3::new(u - i as f64, v - j as f64, w - k as f64),
                        );
                }
            }
        }
        acc
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut tmp_p = p;
        let mut weight = 1.0;
        for _i in 0..depth {
            accum += weight * Perlin::noise(&self, tmp_p.clone());
            weight *= 0.5;
            tmp_p = tmp_p * 2.0;
        }
        accum.abs()
    }
}
