use std::sync::Arc;

use crate::{hittable::Hittable, Vec3};

pub struct Onb {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Onb {
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

    pub fn local1(&self, a: &Vec3) -> Vec3 {
        self.u * a.x + self.v * a.y + self.w * a.z
    }

    pub fn build_from_w(n1: &Vec3) -> Self {
        let w1 = n1.unit();
        let mut a1 = Vec3::new(1.0, 0.0, 1.0);
        if w1.x.abs() > 0.9 {
            a1.x = 0.0;
            a1.y = 1.0;
        }
        let v1 = Vec3::cross(w1, a1).unit();
        let u1 = Vec3::cross(w1, v1);

        Self {
            u: u1,
            v: v1,
            w: w1
        }
    }
}
pub struct FlipFace {
    ptr: Arc<dyn Hittable>,
}
impl FlipFace {
    pub fn new(p: Arc<dyn Hittable>) -> Self {
        Self { ptr: p }
    }
}
impl Hittable for FlipFace {
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::HitRecord> {
        //let rec:HitRecord;
        if let Some(mut rec) = self.ptr.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            return Some(rec);
        }
        None
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        self.ptr.bounding_box(time0, time1)
    }
}
