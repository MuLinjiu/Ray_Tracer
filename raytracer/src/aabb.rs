
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct AABB{
    minmum:Vec3,
    maxmum:Vec3,
}


pub fn fmin(a:f64,b:f64) -> f64{
    if a < b {
        return a;
    }else {
        return b;
    }
}

pub fn fmax(a:f64,b:f64) -> f64{
    if a > b {
        return a;
    }
    else {
        return b;
    }
}

impl AABB{

    pub fn new(a:Vec3,b:Vec3) -> Self{
        Self{
            minmum:a,
            maxmum:b,
        }
    }

    pub fn surrounding_box(box0:Self, box1:Self) -> Self{
        let small = Vec3::new(fmin(box0.minmum.x,box1.minmum.x),fmin(box0.minmum.y,box1.minmum.y),fmin(box0.minmum.z,box1.minmum.z));
        let big = Vec3::new(fmax(box0.minmum.x,box1.minmum.x),fmax(box0.minmum.y,box1.minmum.y),fmax(box0.minmum.z,box1.minmum.z));
            Self{
                minmum:small,
                maxmum:big,
            }
    }

    pub fn hit(&self,r:Ray,mut t_min:f64,mut t_max:f64) -> bool{
        for a in 0..2{
            if a == 0{
                let mut t0 = fmin((self.minmum.x - r.orig.x) / r.dir.x,(self.maxmum.x - r.orig.x) / r.dir.x);
                let mut t1 = fmax((self.minmum.x - r.orig.x) / r.dir.x,(self.maxmum.x - r.orig.x) / r.dir.x);
                t_min = fmax(t0, t_min);
                t_max = fmin(t1,t_max);
                if t_max <= t_min{
                    return false;
                }
            }else if a == 1{
                let mut t0 = fmin((self.minmum.y - r.orig.y) / r.dir.y,(self.maxmum.y - r.orig.y) / r.dir.y);
                let mut t1 = fmax((self.minmum.y - r.orig.y) / r.dir.y,(self.maxmum.y - r.orig.y) / r.dir.y);
                t_min = fmax(t0, t_min);
                t_max = fmin(t1,t_max);
                if t_max <= t_min{
                    return false;
                }
            }else if a == 2{
                let mut t0 = fmin((self.minmum.z - r.orig.z) / r.dir.z,(self.maxmum.z - r.orig.z) / r.dir.z);
                let mut t1 = fmax((self.minmum.z - r.orig.z) / r.dir.z,(self.maxmum.z - r.orig.z) / r.dir.z);
                t_min = fmax(t0, t_min);
                t_max = fmin(t1,t_max);
                if t_max <= t_min{
                    return false;
                }
            }
        }
        true
    }
}