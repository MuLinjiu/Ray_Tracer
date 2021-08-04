use std::{f64::INFINITY, sync::Arc};

//use crate::{Ray, Vec3, aabb::{fmax, fmin, AABB}, camera::degrees_to_radians, hittable::{self, HitRecord, Hittable}, materia::Material, rtweekend::random_double2};

use crate::{
    aabb::{fmax, fmin, AABB},
    camera::degrees_to_radians,
    hittable::{self, HitRecord, Hittable},
    materia::Material,
    rtweekend::random_double2,
    Ray, Vec3,
};
pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl XyRect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat.clone(),
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl Hittable for XyRect {
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        //let rec;
        if let Some(rec) = self.hit(&Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.y1 - self.y0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.len();
            return distance_squared / (cosine * area);
        }
        0.0
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Vec3::new(
            random_double2(self.x0, self.x1),
            random_double2(self.y0, self.y1),
            self.k,
        );
        random_point - o
    }
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord::new(Vec3::zero(), Vec3::zero(), 0.0, self.mp.clone(), false);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time11: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl XzRect {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat.clone(),
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl Hittable for XzRect {
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        //let rec;
        if let Some(rec) = self.hit(&Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.len();
            return distance_squared / (cosine * area);
        }
        0.0
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Vec3::new(
            random_double2(self.x0, self.x1),
            self.k,
            random_double2(self.z0, self.z1),
        );
        random_point - o
    }

    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(Vec3::zero(), Vec3::zero(), 0.0, self.mp.clone(), false);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn Material>,
}

impl YzRect {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: mat.clone(),
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl Hittable for YzRect {
    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        //let rec;
        if let Some(rec) = self.hit(&Ray::new(*o, *v, 0.0), 0.001, INFINITY) {
            let area = (self.y1 - self.y0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.squared_length();
            let cosine = Vec3::dot(*v, rec.normal).abs() / v.len();
            return distance_squared / (cosine * area);
        }
        0.0
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Vec3::new(
            self.k,
            random_double2(self.y0, self.y1),
            random_double2(self.z0, self.z1),
        );
        random_point - o
    }

    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new(Vec3::zero(), Vec3::zero(), 0.0, self.mp.clone(), false);
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time11: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

pub struct Translate {
    offset: Vec3,
    ptr: Arc<dyn Hittable>,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: p.clone(),
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir, r.time);
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if let Some(output_box) = self.ptr.bounding_box(time0, time1) {
            return Some(AABB::new(
                output_box.minmum + self.offset,
                output_box.maxmum + self.offset,
            ));
        }
        None
    }

    fn pdf_value(&self, _o: &Vec3, _v: &Vec3) -> f64 {
        self.ptr.pdf_value(&(*_o - self.offset), _v)
    }

    fn random(&self, _o: Vec3) -> Vec3 {
        self.ptr.random(_o - self.offset)
    }
}
pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hash_box: bool,
    pub bbox: AABB,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sintheta = radians.sin();
        let costheta = radians.cos();
        let mut flag = false;
        let hashbox_ = p.bounding_box(0.0, 1.0);
        let mut bbox = AABB::new(Vec3::zero(), Vec3::zero());
        if let Some(bbbox) = hashbox_ {
            bbox = bbbox;
            flag = true;
        }

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = -min;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maxmum.x + (1 - i) as f64 * bbox.minmum.x;
                    let y = j as f64 * bbox.maxmum.y + (1 - j) as f64 * bbox.minmum.y;
                    let z = k as f64 * bbox.maxmum.z + (1 - k) as f64 * bbox.minmum.z;

                    let newx = costheta * x + sintheta * z;
                    let newz = -sintheta * x + costheta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        if c == 0 {
                            min.x = fmin(min.x, tester.x);
                            max.x = fmax(max.x, tester.x);
                        } else if c == 1 {
                            min.y = fmin(min.y, tester.y);
                            max.y = fmax(max.y, tester.y);
                        } else if c == 2 {
                            min.z = fmin(min.z, tester.z);
                            max.z = fmax(max.z, tester.z);
                        }
                    }
                }
            }
        }
        Self {
            ptr: p.clone(),
            sin_theta: sintheta,
            cos_theta: costheta,
            hash_box: flag,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;

        origin.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        origin.z = self.cos_theta * r.orig.z + self.sin_theta * r.orig.x;

        direction.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        direction.z = self.cos_theta * r.dir.z + self.sin_theta * r.dir.x;

        let rotate_r = Ray::new(origin, direction, r.time);

        if let Some(mut rec) = self.ptr.hit(&rotate_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = self.cos_theta * rec.p.z - self.sin_theta * rec.p.x;

            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = self.cos_theta * rec.normal.z - self.sin_theta * rec.normal.x;

            rec.p.x = p.x;
            rec.p.z = p.z;

            rec.normal.x = normal.x;
            rec.normal.z = normal.z;

            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, _time0: f64, _time11: f64) -> Option<AABB> {
        let output_box = self.bbox.clone();
        Some(output_box)
    }
}

// pub struct RotateZ {
//     pub ptr: Arc<dyn Hittable>,
//     pub sin_theta: f64,
//     pub cos_theta: f64,
//     pub hash_box: bool,
//     pub bbox: AABB,
// }

// impl RotateZ {
//     pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
//         let radians = degrees_to_radians(angle);
//         let sintheta = radians.sin();
//         let costheta = radians.cos();
//         let mut flag = false;
//         let hashbox_ = p.bounding_box(0.0, 1.0);
//         let mut bbox = AABB::new(Vec3::zero(), Vec3::zero());
//         if let Some(bbbox) = hashbox_ {
//             bbox = bbbox;
//             flag = true;
//         }

//         let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
//         let mut max = -min;

//         for i in 0..2 {
//             for j in 0..2 {
//                 for k in 0..2 {
//                     let x = i as f64 * bbox.maxmum.x + (1 - i) as f64 * bbox.minmum.x;
//                     let y = j as f64 * bbox.maxmum.y + (1 - j) as f64 * bbox.minmum.y;
//                     let z = k as f64 * bbox.maxmum.z + (1 - k) as f64 * bbox.minmum.z;

//                     let newx = costheta * x + sintheta * y;
//                     let newy = -sintheta * x + costheta * y;

//                     let tester = Vec3::new(newx, newy, z);

//                     for c in 0..3 {
//                         if c == 0 {
//                             min.x = fmin(min.x, tester.x);
//                             max.x = fmax(max.x, tester.x);
//                         } else if c == 1 {
//                             min.y = fmin(min.y, tester.y);
//                             max.y = fmax(max.y, tester.y);
//                         } else if c == 2 {
//                             min.z = fmin(min.z, tester.z);
//                             max.z = fmax(max.z, tester.z);
//                         }
//                     }
//                 }
//             }
//         }
//         Self {
//             ptr: p.clone(),
//             sin_theta: sintheta,
//             cos_theta: costheta,
//             hash_box: flag,
//             bbox,
//         }
//     }
// }

// impl Hittable for RotateZ {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         let mut origin = r.orig;
//         let mut direction = r.dir;

//         origin.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.y;
//         origin.y = self.cos_theta * r.orig.y + self.sin_theta * r.orig.x;

//         direction.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.y;
//         direction.y = self.cos_theta * r.dir.y + self.sin_theta * r.dir.x;

//         let rotate_r = Ray::new(origin, direction, r.time);

//         if let Some(mut rec) = self.ptr.hit(&rotate_r, t_min, t_max) {
//             let mut p = rec.p;
//             let mut normal = rec.normal;

//             p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.y;
//             p.y = self.cos_theta * rec.p.y - self.sin_theta * rec.p.x;

//             normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.y;
//             normal.y = self.cos_theta * rec.normal.y - self.sin_theta * rec.normal.x;

//             rec.p.x = p.x;
//             rec.p.y = p.y;

//             rec.normal.x = normal.x;
//             rec.normal.y = normal.y;

//             return Some(rec);
//         }
//         None
//     }

//     fn bounding_box(&self, _time0: f64, _time11: f64) -> Option<AABB> {
//         let output_box = self.bbox.clone();
//         Some(output_box)
//     }
// }

// pub struct RotateX {
//     pub ptr: Arc<dyn Hittable>,
//     pub sin_theta: f64,
//     pub cos_theta: f64,
//     pub hash_box: bool,
//     pub bbox: AABB,
// }

// impl RotateX {
//     pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
//         let radians = degrees_to_radians(angle);
//         let sintheta = radians.sin();
//         let costheta = radians.cos();
//         let mut flag = false;
//         let hashbox_ = p.bounding_box(0.0, 1.0);
//         let mut bbox = AABB::new(Vec3::zero(), Vec3::zero());
//         if let Some(bbbox) = hashbox_ {
//             bbox = bbbox;
//             flag = true;
//         }

//         let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
//         let mut max = -min;

//         for i in 0..2 {
//             for j in 0..2 {
//                 for k in 0..2 {
//                     let x = i as f64 * bbox.maxmum.x + (1 - i) as f64 * bbox.minmum.x;
//                     let y = j as f64 * bbox.maxmum.y + (1 - j) as f64 * bbox.minmum.y;
//                     let z = k as f64 * bbox.maxmum.z + (1 - k) as f64 * bbox.minmum.z;

//                     let newy = costheta * y + sintheta * z;
//                     let newz = -sintheta * y + costheta * z;

//                     let tester = Vec3::new(x, newy, newz);

//                     for c in 0..3 {
//                         if c == 0 {
//                             min.x = fmin(min.x, tester.x);
//                             max.x = fmax(max.x, tester.x);
//                         } else if c == 1 {
//                             min.y = fmin(min.y, tester.y);
//                             max.y = fmax(max.y, tester.y);
//                         } else if c == 2 {
//                             min.z = fmin(min.z, tester.z);
//                             max.z = fmax(max.z, tester.z);
//                         }
//                     }
//                 }
//             }
//         }
//         Self {
//             ptr: p.clone(),
//             sin_theta: sintheta,
//             cos_theta: costheta,
//             hash_box: flag,
//             bbox,
//         }
//     }
// }

// impl Hittable for RotateX {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         let mut origin = r.orig;
//         let mut direction = r.dir;

//         origin.y = self.cos_theta * r.orig.y - self.sin_theta * r.orig.z;
//         origin.z = self.cos_theta * r.orig.z + self.sin_theta * r.orig.y;

//         direction.y = self.cos_theta * r.dir.y - self.sin_theta * r.dir.z;
//         direction.z = self.cos_theta * r.dir.z + self.sin_theta * r.dir.y;

//         let rotate_r = Ray::new(origin, direction, r.time);

//         if let Some(mut rec) = self.ptr.hit(&rotate_r, t_min, t_max) {
//             let mut p = rec.p;
//             let mut normal = rec.normal;

//             p.y = self.cos_theta * rec.p.y + self.sin_theta * rec.p.z;
//             p.z = self.cos_theta * rec.p.z - self.sin_theta * rec.p.y;

//             normal.y = self.cos_theta * rec.normal.y + self.sin_theta * rec.normal.z;
//             normal.z = self.cos_theta * rec.normal.z - self.sin_theta * rec.normal.y;

//             rec.p.y = p.y;
//             rec.p.z = p.z;

//             rec.normal.y = normal.y;
//             rec.normal.z = normal.z;

//             return Some(rec);
//         }
//         None
//     }

//     fn bounding_box(&self, _time0: f64, _time11: f64) -> Option<AABB> {
//         let output_box = self.bbox.clone();
//         Some(output_box)
//     }
// }

pub struct Triangle {
    pub mp: Arc<dyn Material>,
    pub a1: Vec3,
    pub a2: Vec3,
    pub a3: Vec3,
}
#[allow(clippy::too_many_arguments)]
impl Triangle {
    pub fn new(
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
        x3: f64,
        y3: f64,
        z3: f64,
        m: Arc<dyn Material>,
    ) -> Self {
        //println!("{},{},{}\t{},{},{}\t{},{},{}",x1,y1,z1,x2,y2,z2,x3,y3,z3);
        Self {
            mp: m,
            a1: Vec3::new(x1, y1, z1),
            a2: Vec3::new(x2, y2, z2),
            a3: Vec3::new(x3, y3, z3),
        }
    }
    pub fn inside(&self, x: f64, y: f64, z: f64) -> bool {
        let point = Vec3::new(x, y, z);
        let ap = self.a1 - point;
        let bp = self.a2 - point;
        let cp = self.a3 - point;
        let ab = self.a1 - self.a2;
        let ac = self.a1 - self.a3;
        let s1 = Vec3::cross(ap, bp).len() / 2.0;
        let s2 = Vec3::cross(ap, cp).len() / 2.0;
        let s3 = Vec3::cross(bp, cp).len() / 2.0;
        let s = Vec3::cross(ab, ac).len() / 2.0;

        if (s - (s1 + s2 + s3)).abs() < 0.0001 {
            return true;
        }
        false
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let dir1 = self.a2 - self.a1;
        let dir2 = self.a3 - self.a1;
        let n = Vec3::cross(dir1, dir2);
        let b_a = self.a1 - r.orig;
        let t = Vec3::dot(n, b_a) / Vec3::dot(n, r.dir);
        if t < t_min || t > t_max {
            return None;
        }
        let hit = r.at(t);
        //println!("{},{},{}",hit.x, hit.y, hit.z);
        //if Triangle::inside(self, hit.x, hit.y, hit.z) {
        if self.inside(hit.x, hit.y, hit.z) {
            let mut rec = HitRecord::new(Vec3::zero(), Vec3::zero(), 0.0, self.mp.clone(), false);
            rec.p = r.at(t);
            rec.t = t;
            let ourward_normal = n.unit();
            rec.set_face_normal(&r, ourward_normal);
            rec.mat_ptr = self.mp.clone();
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let a1 = Vec3::new(
            min3(self.a1.x, self.a2.x, self.a3.x),
            min3(self.a1.y, self.a2.y, self.a3.y),
            min3(self.a1.z, self.a2.z, self.a3.z),
        ) - Vec3::new(0.0001, 0.0001, 0.0001);
        let a2 = Vec3::new(
            max3(self.a1.x, self.a2.x, self.a3.x),
            max3(self.a1.y, self.a2.y, self.a3.y),
            max3(self.a1.z, self.a2.z, self.a3.z),
        ) + Vec3::new(0.0001, 0.0001, 0.0001);
        Some(AABB::new(a1, a2))
    }
}

pub fn max3(a: f64, b: f64, c: f64) -> f64 {
    fmax(fmax(a, b), c)
}

pub fn min3(a: f64, b: f64, c: f64) -> f64 {
    fmin(fmin(a, b), c)
}
