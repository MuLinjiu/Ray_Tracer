use std::sync::Arc;

// use crate::{Ray, aabb::AABB, hittable::{HitRecord, Hittable}, rtweekend::random_int, vec3::Vec3};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    rtweekend::random_int,
    vec3::Vec3,
    Ray,
};

#[allow(clippy::float_cmp)]
#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        &self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn pdf_value(&self, _o: &Vec3, _v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for object in self.objects.iter() {
            //println!("{}"obje)
            sum += weight * object.pdf_value(_o, _v);
        }
        sum
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        let int_size = self.objects.len();
        //return self.objects[random_int(0, int_size as i32)as usize].random(_o);
        return self.objects[random_int(0, int_size as i32) as usize].random(_o);
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let _tempbox = AABB::new(Vec3::zero(), Vec3::zero());
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
