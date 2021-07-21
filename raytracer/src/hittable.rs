use std::sync::Arc;

use crate::aabb::AABB;
use crate::materia::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub mat_ptr: Arc<dyn Material>,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(p_: Vec3, n: Vec3, t_: f64, mat: Arc<dyn Material>, f: bool) -> Self {
        Self {
            p: p_,
            normal: n,
            t: t_,
            mat_ptr: mat,
            front_face: f,
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dir, outward_normal) < 0.0;
        if self.front_face {
            self.normal.x = outward_normal.x;
            self.normal.y = outward_normal.y;
            self.normal.z = outward_normal.z;
        } else {
            self.normal.x = -outward_normal.x;
            self.normal.y = -outward_normal.y;
            self.normal.z = -outward_normal.z;
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}
