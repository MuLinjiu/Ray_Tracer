use std::sync::Arc;

use crate::{Ray, hittable::{Hittable, hit_record}};

#[allow(clippy::float_cmp)]
pub struct Hittable_list{
    objects:Vec<Arc<dyn Hittable>>,
}

impl Hittable_list{
    pub fn new() -> Self{
            Self{
                objects : Vec::new(),
            }
    }
    pub fn add(&mut self,object:Arc<dyn Hittable>){
        &self.objects.push(object);
    }
}

impl Hittable for Hittable_list{
    fn hit(&self,r:&Ray,t_min:f64,t_max:f64) -> Option<hit_record>{
        let mut rec = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter()
        {
            let t = object.hit(r, t_min, closest_so_far);
            if let Some(rec_) = t{
                let t = rec_.t.clone();
                rec = Some(rec_);
                closest_so_far = t;
            }
        }
        rec
    }
}
