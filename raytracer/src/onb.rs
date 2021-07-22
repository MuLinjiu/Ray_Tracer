use std::sync::Arc;

use crate::{Vec3, hittable::{Hittable}};


pub struct Onb{
    pub u:Vec3,
    pub v:Vec3,
    pub w:Vec3,
}   

impl Onb{
    // pub fn new() -> Self{
    //     Self{
    //         u:Vec3::zero(),
    //         v:Vec3::zero(),
    //         w:Vec3::zero(),
    //     }
    // }
    // pub fn local(&self,a:f64,b:f64,c:f64) -> Vec3{
    //     self.u * a + self.v * b + self.w * c
    // }

    pub fn local1(&self,a:&Vec3) -> Vec3{
        self.u * a.x + self.v * a.y + self.w * a.z
    }

    pub fn build_from_w(n:&Vec3) -> Self{
        let w = n.unit();
        let mut a = Vec3::new(1.0,0.0,1.0);
        if w.x.abs() > 0.9 {
            a.x = 0.0;
            a.y = 1.0;
        }
        let v = Vec3::cross(w, a).unit();
        let u = Vec3::cross(w,v);

        Self{
            u,v,w,
        }


    }
}


pub struct FlipFace{
    ptr:Arc<dyn Hittable>,
}

impl FlipFace{
    pub fn new(p:Arc<dyn Hittable>) -> Self{
        Self{ptr:p}
    }


}

impl Hittable for FlipFace{
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        //let rec:HitRecord;
        if let Some(mut rec) = self.ptr.hit(r, t_min, t_max){
            rec.front_face = !rec.front_face;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        self.ptr.bounding_box(time0, time1)
    }
}