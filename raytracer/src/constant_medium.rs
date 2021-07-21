use std::{
    f64::{consts::E, INFINITY},
    sync::Arc,
};

use crate::{
    hittable::{HitRecord, Hittable},
    materia::{Material, Metal},
    rtweekend::random_double,
    texture::{SolidColor, Texture},
    vec3::random_in_unit_sphere,
    Ray, Vec3,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    // pub fn new(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
    //     Self {
    //         boundary: b,
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Arc::new(isotropic::new1(a)),
    //     }
    // }

    pub fn new1(b: Arc<dyn Hittable>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        return self.boundary.bounding_box(time0, time1);
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        // let mut rec1 = hit_record::new(
        //     Vec3::zero(),
        //     Vec3::zero(),
        //     0.0,
        //     Arc::new(metal::new(Vec3::zero(), 0.0)),
        //     false,
        // );
        let mut rec1;
        // let mut rec2 = hit_record::new(
        //     Vec3::zero(),
        //     Vec3::zero(),
        //     0.0,
        //     Arc::new(metal::new(Vec3::zero(), 0.0)),
        //     false,
        // );
        let mut rec2;
        if let Some(rec1_) = self.boundary.hit(r, -INFINITY, INFINITY) {
            rec1 = rec1_.clone();
        } else {
            return None;
        }
        if let Some(rec2_) = self.boundary.hit(r, rec1.t + 0.0001, INFINITY) {
            rec2 = rec2_.clone();
        } else {
            return None;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.len();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(0.0, 100.0).log(E);

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let mut rec = HitRecord::new(
            Vec3::zero(),
            Vec3::zero(),
            0.0,
            Arc::new(Metal::new(Vec3::zero(), 0.0)),
            false,
        );
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::zero();
        rec.normal.x = 1.0;
        rec.front_face = true;
        rec.mat_ptr = self.phase_function.clone();
        Some(rec)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(c: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }

    // pub fn new1(a: Arc<dyn Texture>) -> Self {
    //     Self { albedo: a }
    // }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &crate::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut crate::Ray,
    ) -> bool {
        //scattered = &mut Ray::new(rec.p, random_in_unit_sphere(), r_in.time);
        scattered.orig = rec.p;
        scattered.dir = random_in_unit_sphere();
        scattered.time = r_in.time;
        attenuation.x = self.albedo.value(rec.u, rec.v, &rec.p).x;
        attenuation.y = self.albedo.value(rec.u, rec.v, &rec.p).y;
        attenuation.z = self.albedo.value(rec.u, rec.v, &rec.p).z;
        true
    }

    fn emitted(&self, _r_in:&Ray,_rec:&HitRecord, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}
