use std::f64::consts::PI;
use std::sync::Arc;

use crate::hittable::HitRecord;

use crate::onb::Onb;
use crate::pdf::{CosinePdf, NonePdf, Pdf};
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{random_in_unit_sphere, reflect, refract, Vec3};
pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double(0.0, 100.0);
    let r2 = random_double(0.0, 100.0);
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        srec: &mut ScatterRecord,
    ) -> bool;

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
    }

    fn get_pdf_value(&self, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3;
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
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
        srec: &mut ScatterRecord,
    ) -> bool {
        let reflected = reflect(Vec3::unit(r_in.dir), rec.normal);
        srec.specular_ray.orig = rec.p;
        srec.specular_ray.dir = reflected + random_in_unit_sphere() * self.fuzz;
        srec.specular_ray.time = r_in.time;
        //scattered = &Ray::new(rec.p, reflected);
        // scattered.orig = rec.p;
        // scattered.dir = reflected + random_in_unit_sphere() * self.fuzz;
        // scattered.time = r_in.time;
        //attenuation = &self.albedo.clone();
        srec.is_specular = true;
        srec.attenuation.x = self.albedo.x;
        srec.attenuation.y = self.albedo.y;
        srec.attenuation.z = self.albedo.z;
        srec.pdf_ptr = Arc::new(NonePdf::new());
        true
        //srec.pdf_ptr = 0;
        //return Vec3::dot(scattered.dir, rec.normal) > 0.0;
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
    }

    fn get_pdf_value(&self, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
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
    fn get_pdf_value(&self, rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        let uvw = Onb::build_from_w(&rec.normal.clone());
        let direction = uvw.local1(&random_cosine_direction());
        //let scatter_direction = rec.normal.clone() + random_in_unit_sphere();
        //scattered = &Ray::new(rec.p, scatter_direction);
        //scattered.orig = rec.p;
        //scattered.dir =

        Vec3::dot(uvw.w, direction.unit()) / PI
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
        srec: &mut ScatterRecord,
    ) -> bool {
        // let uvw = Onb::build_from_w(&rec.normal.clone());
        // let direction = uvw.local1(&random_cosine_direction());
        //let scatter_direction = rec.normal.clone() + random_in_unit_sphere();
        //scattered = &Ray::new(rec.p, scatter_direction);
        // scattered.orig = rec.p;
        // scattered.dir = direction.unit();
        // scattered.time = r_in.time;

        //attenuation = &self.albedo;
        // attenuation.x = self.albedo.x;
        // attenuation.y = self.albedo.y;
        // attenuation.z = self.albedo.z;
        // attenuation.x = self.albedo.value(rec.u, rec.v, &rec.p).x;
        // attenuation.y = self.albedo.value(rec.u, rec.v, &rec.p).y;
        // attenuation.z = self.albedo.value(rec.u, rec.v, &rec.p).z;

        srec.is_specular = false;
        srec.attenuation.x = self.albedo.value(rec.u, rec.v, &rec.p).x;
        srec.attenuation.y = self.albedo.value(rec.u, rec.v, &rec.p).y;
        srec.attenuation.z = self.albedo.value(rec.u, rec.v, &rec.p).z;
        srec.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        //*pdf = Vec3::dot(rec.normal.clone(),scattered.dir.clone()) / PI;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = Vec3::dot(rec.normal, Vec3::unit(scattered.dir));
        if cosine < 0.0 {
            //println!("cao");
            0.0
        } else {
            //println!("diao");
            cosine / PI
        }
    }
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
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
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
        srec: &mut ScatterRecord,
    ) -> bool {
        srec.is_specular = true;
        srec.pdf_ptr = Arc::new(NonePdf::new());
        srec.attenuation.x = 1.0;
        srec.attenuation.y = 1.0;
        srec.attenuation.z = 1.0;
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
            srec.specular_ray.orig = rec.p;
            srec.specular_ray.dir = reflected;
            srec.specular_ray.time = r_in.time;
            // scattered.orig = rec.p;
            // scattered.dir = reflected;
            // scattered.time = r_in.time;
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if random_double(0.0, 100.0) < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            srec.specular_ray.orig = rec.p;
            srec.specular_ray.dir = reflected;
            srec.specular_ray.time = r_in.time;
            // scattered.orig = rec.p;
            // scattered.dir = reflected;
            // scattered.time = r_in.time;
            return true;
        }
        //let cos_theta = Ord::min(Vec3::dot(-unit_direction, rec.normal),1.0);
        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        //scattered = Ray::new(rec.p,refracted);
        srec.specular_ray.orig = rec.p;
        srec.specular_ray.dir = refracted;
        srec.specular_ray.time = r_in.time;
        // scattered.orig = rec.p;
        // scattered.dir = refracted;
        // scattered.time = r_in.time;
        true
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
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        if rec.front_face {
            // println!("1");
            return self.emit.value(u, v, p);
        }
        // Vec3::zero()
        self.emit.value(u, v, p)
    }
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
        _srec: &mut ScatterRecord,
    ) -> bool {
        false
    }
}

#[derive(Clone)]

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vec3,
    pub pdf_ptr: Arc<dyn Pdf>,
}

impl ScatterRecord {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            specular_ray: Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
            is_specular: true,
            attenuation: Vec3::zero(),
            pdf_ptr: Arc::new(NonePdf::new()),
        }
    }
}
