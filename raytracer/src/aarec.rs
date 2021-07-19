use std::{f64::INFINITY, sync::Arc};

use crate::{
    aabb::{fmax, fmin, AABB},
    camera::degrees_to_radians,
    hittable::{self, hit_record, Hittable},
    materia::{self, material, metal},
    Ray, Vec3,
};

pub struct xy_rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Arc<dyn material>,
}

impl xy_rect {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Arc<dyn material>) -> Self {
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

impl Hittable for xy_rect {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hittable::hit_record> {
        let t = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = hit_record::new(Vec3::zero(), Vec3::zero(), 0.0, self.mp.clone(), false);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return Some(rec);
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct xz_rect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn material>,
}

impl xz_rect {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn material>) -> Self {
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

impl Hittable for xz_rect {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hittable::hit_record> {
        let t = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = hit_record::new(Vec3::zero(), Vec3::zero(), 0.0, self.mp.clone(), false);
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return Some(rec);
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct yz_rect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Arc<dyn material>,
}

impl yz_rect {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: Arc<dyn material>) -> Self {
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

impl Hittable for yz_rect {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hittable::hit_record> {
        let t = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = hit_record::new(Vec3::zero(), Vec3::zero(), 0.0, self.mp.clone(), false);
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        return Some(rec);
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

pub struct translate {
    offset: Vec3,
    ptr: Arc<dyn Hittable>,
}

impl translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: p.clone(),
            offset: displacement,
        }
    }
}

impl Hittable for translate {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hit_record> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir, r.time);
        if let Some(mut rec) = self.ptr.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            return Some(rec);
        }
        return None;
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
}

pub struct rotate_y {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hash_box: bool,
    pub bbox: AABB,
}

impl rotate_y {
    pub fn new(p: Arc<Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sintheta = radians.sin();
        let costheta = radians.cos();
        let mut flag = false;
        let hashbox_ = p.bounding_box(0.0, 1.0);
        let mut bbox = AABB::new(Vec3::zero(), Vec3::zero());
        if let Some(BBOX) = hashbox_ {
            bbox = BBOX.clone();
            flag = true;
        }

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = -min.clone();

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
            bbox: bbox,
        }
    }
}

impl Hittable for rotate_y {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<hit_record> {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let output_box = self.bbox.clone();
        Some(output_box)
    }
}
