#[allow(clippy::float_cmp)]

use crate::vec3::Vec3;
use crate::{hittable::{Hittable, hit_record}, ray::Ray};

#[derive(Clone, Debug, PartialEq,Copy)]
pub struct Sphere{
    pub radius:f64,
    pub center:Vec3,
}

impl Sphere{
    // fn pub new(r:Ray,t_min:f64,t_max:f64,rec:hit_record){

    // }
    pub fn new(center:Vec3,radius:f64) -> Self{
        Self{center,radius}
    }
}


impl Hittable for Sphere{
    fn hit(&self,r:&Ray,t_min:f64,t_max:f64) -> Option<hit_record>{
        let mut rec = hit_record::new();
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
			    return Some(rec);
            }
            let t = (-half_b + root) / a;
		    if t > t_min && t < t_max {
			    rec.t = t;
			    rec.p = r.at(t);
			    let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r,outward_normal);
			    return Some(rec);
		    }
        }
       // println!("{} {} {}\n",r.orig.x,r.orig.y,r.orig.z);
        return None;
        
    }
}

// impl Hittable for Sphere{
//     fn hit(r:Ray,t_min:f64,t_max:f64,rec:hit_record) -> bool{
//         let oc:Vec3 = r.orig - center;
//         let a: f64 = Vec3::modlen(r.dir);
//         let half_b: f64 = Vec3::dot(r.dir,oc);
//         let c: f64 = Vec3::modlen(oc) - radius * radius;
//         let discriminant = half_b * half_b - a * c;
//         if discriminant > 0.0{
//             let root = discriminant.sqrt();
//             let t = (-half_b - root) / a;
            
//         }
//     }
// }