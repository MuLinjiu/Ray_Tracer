#[allow(clippy::float_cmp)]

use crate::vec3::Vec3;
use crate::ray::Ray;
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct hit_record{
    pub p:Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face:bool,
}

impl hit_record{
    pub fn new() -> Self{
        Self{
            p:Vec3::zero(),
            normal:Vec3::zero_(),
            t:0.0,
            front_face:false,
        }
    }
    pub fn set_face_normal(&mut self,r:&Ray,outward_normal:Vec3){
        self.front_face = Vec3::dot(r.dir,outward_normal) < 0.0;
        if self.front_face{
            self.normal.x = outward_normal.x;
            self.normal.y = outward_normal.y;
            self.normal.z = outward_normal.z;
        }else {
            self.normal.x = -outward_normal.x;
            self.normal.y = -outward_normal.y;
            self.normal.z = -outward_normal.z;
    }
}
}

pub trait Hittable{
    fn hit(&self,r:&Ray,t_min:f64,t_max:f64) -> Option<hit_record>;
}