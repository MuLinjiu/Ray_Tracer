
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::onb::Onb;
use crate::rtweekend::random_double;
use crate::vec3::Vec3;
use crate::{
    hittable::{HitRecord, Hittable},
    materia::{Material, Metal},
    ray::Ray,
};

pub struct Sphere {
    pub radius: f64,
    pub center: Vec3,
    pub mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
    fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let temptheta = (-p.z) / p.x;
        let mut phi = (temptheta).atan();
        phi = phi + PI;
        *u = *&mut (phi / (2.0 * PI));
        *v = *&mut (theta / PI);
    }
}

impl Hittable for Sphere {
    fn pdf_value(&self, _o: &Vec3, _v: &Vec3) -> f64 {
            if let Some(_rec) = self.hit(&Ray::new(*_o, *_v, 0.0), 0.0001, INFINITY) {
                let cos_theta_max =
                    (1.0 - self.radius * self.radius / (self.center - *_o).squared_length()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            return 1.0 / solid_angle;
        }
        0.0
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = HitRecord::new(
            Vec3::zero(),
            Vec3::zero(),
            0.0,
            Arc::new(Metal::new(Vec3::zero(), 0.0)),
            false,
        );
        let oc: Vec3 = r.orig - self.center;
        let a: f64 = Vec3::len_squared(r.dir);
        let half_b: f64 = Vec3::dot(r.dir, oc);
        let c: f64 = Vec3::len_squared(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return Some(rec);
            }
            let t = (-half_b + root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return Some(rec);
            }
        }
        return None;
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let output = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        return Some(output);
    }

    fn random(&self, _o: Vec3) -> Vec3 {
        let direction = self.center - _o;
        let distance_squared = direction.len_squared();
        let uvw = Onb::build_from_w(&direction);
        return uvw.local1(&random_to_sphere(self.radius, distance_squared));

    }
}

pub fn random_to_sphere(radius: f64, distance_squred: f64) -> Vec3 {
    let r1 = random_double(0.0, 100.0);
    let r2 = random_double(0.0, 100.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squred).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    return Vec3::new(x, y, z);
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: Vec3,
        cen1: Vec3,
        _time0: f64,
        _time1: f64,
        r: f64,
        m: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0: Vec3::new(cen0.x, cen0.y, cen0.z),
            center1: Vec3::new(cen1.x, cen1.y, cen1.z),
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        return self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0);
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = HitRecord::new(
            Vec3::zero(),
            Vec3::zero(),
            0.0,
            Arc::new(Metal::new(Vec3::zero(), 0.0)),
            false,
        );
        let oc: Vec3 = r.orig - self.center(r.time);
        let a: f64 = Vec3::len_squared(r.dir);
        let half_b: f64 = Vec3::dot(r.dir, oc);
        let c: f64 = Vec3::len_squared(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal = (rec.p - self.center(r.time)) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return Some(rec);
            }
            let t = (-half_b + root) / a;
            if t > t_min && t < t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal = (rec.p - self.center(r.time)) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return Some(rec);
            }
        }
        return None;
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let output_box = AABB::surrounding_box(box0, box1);
        return Some(output_box);
        //改成option
    }
}
