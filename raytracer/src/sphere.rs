use std::{rc::Rc, sync::Arc};

#[allow(clippy::float_cmp)]

use crate::vec3::Vec3;
use crate::{hittable::{Hittable, hit_record}, materia::{material, metal}, ray::Ray};


pub struct Sphere{
    pub radius:f64,
    pub center:Vec3,
    pub mat_ptr:Arc<dyn material>,
}

impl Sphere{
    pub fn new(center:Vec3,radius:f64,mat_ptr: Arc<dyn material>) -> Self{
        Self{center,radius,mat_ptr}
    }
}


impl Hittable for Sphere{
    fn hit(&self,r:&Ray,t_min:f64,t_max:f64) -> Option<hit_record>{
        let mut rec = hit_record::new(Vec3::zero(),Vec3::zero(),0.0,Arc::new(metal::new(Vec3::zero(),0.0)),false);
        let oc:Vec3 = r.orig - self.center;
        let a: f64 = Vec3::len_squared(r.dir);
        let half_b: f64 = Vec3::dot(r.dir,oc);
        let c: f64 = Vec3::len_squared(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0{
            let root = discriminant.sqrt();
            let t = (-half_b - root) / a;
            if t > t_min && t < t_max{
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r,outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
			    return Some(rec);
            }
            let t = (-half_b + root) / a;
		    if t > t_min && t < t_max {
			    rec.t = t;
			    rec.p = r.at(t);
			    let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r,outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
			    return Some(rec);
		    }
        }
        return None;
        
    }
}
