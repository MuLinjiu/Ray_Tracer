#[allow(clippy::float_cmp)]

use crate::vec3::Vec3;
use crate::ray::Ray;
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