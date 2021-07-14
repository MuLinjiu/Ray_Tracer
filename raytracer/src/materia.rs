use std::cmp::min;

use std::time::Duration;

#[allow(clippy::float_cmp)]
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::vec3::{Vec3, random_in_unit_sphere, reflect, refract};
use crate::hittable::hit_record;

pub trait material{
    fn scatter(&self,r_in:&Ray,rec:&hit_record,attenuation:& mut Vec3,scattered:&mut Ray) -> bool;
}
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct metal{
    albedo:Vec3,
    fuzz:f64,
}

impl metal{
    pub fn new(a:Vec3,t:f64) -> Self{
        let mut t_ = t;
        if t >= 1.0 {
            t_ = 1.0;
        }
        Self{
            albedo:Vec3::new(a.x, a.y, a.z),
            fuzz:t_,
        }
    }
}

impl material for metal{
    fn scatter(&self,r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3,scattered: &mut Ray) -> bool {
        let reflected = reflect(Vec3::unit(r_in.dir), rec.normal);
        //scattered = &Ray::new(rec.p, reflected);
        scattered.orig = rec.p;
        scattered.dir = reflected + random_in_unit_sphere() * self.fuzz;
        //attenuation = &self.albedo.clone();
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return Vec3::dot(scattered.dir,rec.normal) > 0.0;
    }
}

pub struct lambertian{
    albedo:Vec3,
}

impl lambertian{
    pub fn new(a:Vec3) -> Self{
        Self{
            albedo:Vec3::new(a.x, a.y, a.z),
        }
    }
}

impl material for lambertian{
    fn scatter(&self,r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3,scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + random_in_unit_sphere();
        //scattered = &Ray::new(rec.p, scatter_direction);
        scattered.orig = rec.p;
        scattered.dir = scatter_direction;
        //attenuation = &self.albedo;
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        return true;
     }
}

pub fn schlick(cosine:f64,ref_idx:f64) -> f64{
    let mut r0:f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    return r0 + (1.0 - r0) * f64::powf(1.0 - cosine,5.0);
}
pub struct dielectric{

    ref_idx:f64,
}

impl dielectric{
    pub fn new(ri:f64) -> Self{
        Self{
            ref_idx:ri,
        }
    }
}

impl material for dielectric{
    fn scatter(&self, r_in:&Ray, rec:&hit_record, attenuation:& mut Vec3, scattered:&mut Ray) -> bool {
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        let mut etai_over_etat = 0.0;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        }else {
            etai_over_etat = self.ref_idx;
        }
        let unit_direction = Vec3::unit(r_in.dir);

        let mut cos_theta = 1.0;
        let c1 = Vec3::dot(-unit_direction, rec.normal);
        if c1 < 1.0{
            cos_theta = c1;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0{
            let reflected = reflect(unit_direction, rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            return true;
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if random_double(0.0, 100.0) < reflect_prob{
            let reflected = reflect(unit_direction, rec.normal);
            scattered.orig = rec.p;
            scattered.dir = reflected;
            return true;
        }
        //let cos_theta = Ord::min(Vec3::dot(-unit_direction, rec.normal),1.0);
        let refracted = refract(unit_direction,rec.normal,etai_over_etat );
        //scattered = Ray::new(rec.p,refracted);
        scattered.orig = rec.p;
        scattered.dir = refracted;
        return true;
    }
}