use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct AABB {
    pub minmum: Vec3,
    pub maxmum: Vec3,
}

pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

pub fn fmax(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    } else {
        return b;
    }
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            minmum: a,
            maxmum: b,
        }
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Vec3::new(
            fmin(box0.minmum.x, box1.minmum.x),
            fmin(box0.minmum.y, box1.minmum.y),
            fmin(box0.minmum.z, box1.minmum.z),
        );
        let big = Vec3::new(
            fmax(box0.maxmum.x, box1.maxmum.x),
            fmax(box0.maxmum.y, box1.maxmum.y),
            fmax(box0.maxmum.z, box1.maxmum.z),
        );
        Self {
            minmum: small,
            maxmum: big,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir.get(a);
            let mut t0 = (self.minmum.get(a) - r.orig.get(a)) * inv_d;
            let mut t1 = (self.maxmum.get(a) - r.orig.get(a)) * inv_d;
            if inv_d < 0.0 {
                let mid = t0;
                t0 = t1;
                t1 = mid;
            }
            //println!("{},{}\n",t0,t1);
            let ans1 = fmax(t0, t_min);
            let ans2 = fmin(t1, t_max);
            //println!("{},{}\t",t1,t_max);
            if ans2 <= ans1 {
                //if ans2 != ans1 {println!("{},{}\n",ans2,ans1);}
                return false;
            }
            // if a == 0 {
            //     let mut t0 = fmin(
            //         (self.minmum.x - r.orig.x) / r.dir.x,
            //         (self.maxmum.x - r.orig.x) / r.dir.x,
            //     );
            //     let mut t1 = fmax(
            //         (self.minmum.x - r.orig.x) / r.dir.x,
            //         (self.maxmum.x - r.orig.x) / r.dir.x,
            //     );
            //     t_min = fmax(t0, t_min);
            //     t_max = fmin(t1, t_max);
            //     if t_max <= t_min {
            //         println!("1\n");
            //         return false;
            //     }
            // } else if a == 1 {
            //     let mut t0 = fmin(
            //         (self.minmum.y - r.orig.y) / r.dir.y,
            //         (self.maxmum.y - r.orig.y) / r.dir.y,
            //     );
            //     let mut t1 = fmax(
            //         (self.minmum.y - r.orig.y) / r.dir.y,
            //         (self.maxmum.y - r.orig.y) / r.dir.y,
            //     );
            //     t_min = fmax(t0, t_min);
            //     t_max = fmin(t1, t_max);
            //     if t_max <= t_min {
            //         println!("2\n");
            //         return false;
            //     }
            // } else if a == 2 {
            //     let mut t0 = fmin(
            //         (self.minmum.z - r.orig.z) / r.dir.z,
            //         (self.maxmum.z - r.orig.z) / r.dir.z,
            //     );
            //     let mut t1 = fmax(
            //         (self.minmum.z - r.orig.z) / r.dir.z,
            //         (self.maxmum.z - r.orig.z) / r.dir.z,
            //     );
            //     t_min = fmax(t0, t_min);
            //     t_max = fmin(t1, t_max);
            //     if t_max <= t_min {
            //         println!("3\n");
            //         return false;
            //     }
            // }
        }
        //println!("2\n");
        true
    }
}
