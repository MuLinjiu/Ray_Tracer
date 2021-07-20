use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{hit_record, Hittable},
    vec3::Vec3,
    Ray,
};

#[allow(clippy::float_cmp)]
#[derive(Clone)]
pub struct Hittable_list {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl Hittable_list {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        &self.objects.push(object);
    }
}

impl Hittable for Hittable_list {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<hit_record> {
        let mut rec = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            let t = object.hit(r, t_min, closest_so_far);
            if let Some(rec_) = t {
                let t = rec_.t.clone();
                rec = Some(rec_);
                closest_so_far = t;
            }
        }
        rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output = AABB::new(Vec3::zero(), Vec3::zero());
        let tempbox = AABB::new(Vec3::zero(), Vec3::zero());
        let mut first_box = true;
        for object in self.objects.iter() {
            if let Option::Some(tempbox) = object.bounding_box(time0, time1) {
                if first_box {
                    output = tempbox;
                } else {
                    output = AABB::surrounding_box(output, tempbox);
                }
                first_box = false;
            } else {
                return None;
            }
        }
        Some(output)
    }
}
