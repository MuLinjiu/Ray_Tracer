use std::f64::consts::PI;
use std::sync::Arc;

use crate::hittable::HitRecord;

use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{random_in_unit_sphere, reflect, refract, Vec3};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn scattering_pdf(&self,
        r_in: &Ray,
        rec: &HitRecord,
        scattered: &mut Ray,) -> f64{
            return 0.0;
        }

    fn get_pdf_value(&self,rec: &HitRecord,scattered: &mut Ray) -> f64{
        return 0.0;
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Vec3, t: f64) -> Self {
        let mut t_ = t;
        if t >= 1.0 {
            t_ = 1.0;
        }
        Self {
            albedo: Vec3::new(a.x, a.y, a.z),
            fuzz: t_,
        }
    }
}

impl Material for Metal {
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        return Vec3::zero();
    }
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(Vec3::unit(r_in.dir), rec.normal);
        //scattered = &Ray::new(rec.p, reflected);
        scattered.orig = rec.p;
        scattered.dir = reflected + random_in_unit_sphere() * self.fuzz;
        scattered.time = r_in.time;
        //attenuation = &self.albedo.clone();
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return Vec3::dot(scattered.dir, rec.normal) > 0.0;
    }
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn new1(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }

}

impl Material for Lambertian {
    fn get_pdf_value(&self,rec: &HitRecord,scattered: &mut Ray,) -> f64 {
        Vec3::dot(rec.normal.clone(),scattered.dir.clone()) / PI
    }
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal.clone() + random_in_unit_sphere();
        //scattered = &Ray::new(rec.p, scatter_direction);
        scattered.orig = rec.p;
        scattered.dir = scatter_direction;
        scattered.time = r_in.time;
        //attenuation = &self.albedo;
        // attenuation.x = self.albedo.x;
        // attenuation.y = self.albedo.y;
        // attenuation.z = self.albedo.z;
        attenuation.x = self.albedo.value(rec.u, rec.v, &rec.p).x;
        attenuation.y = self.albedo.value(rec.u, rec.v, &rec.p).y;
        attenuation.z = self.albedo.value(rec.u, rec.v, &rec.p).z;
        //*pdf = Vec3::dot(rec.normal.clone(),scattered.dir.clone()) / PI;
        return true;
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal,Vec3::unit(scattered.dir));
        if cosine < 0.0 {
            return 0.0;
        }else {
            return cosine / PI;
        }
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    return r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0);
}
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        let etai_over_etat;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        } else {
            etai_over_etat = self.ref_idx;
        }
        let unit_direction = Vec3::unit(r_in.dir);

        let mut cos_theta = 1.0;
        let c1 = Vec3::dot(-unit_direction, rec.normal);
        if c1 < 1.0 {
            cos_theta = c1;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            scattered.time = r_in.time;
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if random_double(0.0, 100.0) < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            scattered.time = r_in.time;
            return true;
        }
        //let cos_theta = Ord::min(Vec3::dot(-unit_direction, rec.normal),1.0);
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        //scattered = Ray::new(rec.p,refracted);
        scattered.orig = rec.p;
        scattered.dir = refracted;
        scattered.time = r_in.time;
        return true;
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    // pub fn new(a: Arc<dyn Texture>) -> Self {
    //     Self { emit: a.clone() }
    // }

    pub fn new1(c: Vec3) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        return self.emit.value(u, v, p);
    }
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        return false;
    }
}
