use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    rtweekend::random_int,
    Ray, Vec3,
};

pub struct BVHNODE {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box1: AABB,
}

impl Hittable for BVHNODE {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.box1.hit(ray, t_min, t_max) {
            //println!("1\n");
            return None;
        }
        if let Some(rec1) = self.left.hit(ray, t_min, t_max) {
            if let Some(rec2) = self.right.hit(ray, t_min, rec1.t) {
                return Some(rec2);
            } else {
                return Some(rec1);
            }
        } else if let Some(rec2) = self.right.hit(ray, t_min, t_max) {
            return Some(rec2);
        }
        //println!("2\n");
        None
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.box1.clone())
    }
}

impl BVHNODE {
    pub fn new(
        src_objects: &[Arc<dyn Hittable>],
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut objects = src_objects.to_owned();
        let axis = random_int(0, 10000) % 3;
        let left;
        let right;
        let span = end - start;
        let comparetor = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        if span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if span == 2 {
            if comparetor(&objects[start], &objects[start + 1]) {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            objects.sort_by(|a, b| {
                let x = a.bounding_box(time0, time1).unwrap().minmum.get(axis);
                let y = b.bounding_box(time0, time1).unwrap().minmum.get(axis);
                x.partial_cmp(&y).unwrap()
            });
            //obj.sort_by(|a,b| comparetor(&**a,&**b));

            let mid = start + span / 2;
            left = Arc::new(BVHNODE::new(&objects, start, mid, time0, time1));
            right = Arc::new(BVHNODE::new(&objects, mid, end, time0, time1));
        }

        // let box0 = left.bounding_box(time0, time1).unwrap();
        // let box1 = right.bounding_box(time0, time1).unwrap();
        if let Some(box_left) = left.bounding_box(time0, time1) {
            if let Some(box_right) = right.bounding_box(time0, time1) {
                let _box = AABB::surrounding_box(box_left, box_right);
                return Self {
                    left,
                    right,
                    box1: _box,
                };
            }
        }
        panic!("worinima");
        // Self {
        //     left,
        //     right,
        //     box1: AABB::surrounding_box(box0.clone(), box1.clone()),
        // }
        // let left: Arc<dyn Hittable>;
        // let right: Arc<dyn Hittable>;
        // if span == 1 {
        //     left = objects.remove(0);
        //     right = left.clone();
        // } else if span == 2 {
        //     left = objects.remove(0);
        //     right = objects.remove(0);
        // } else {
        //     objects.sort_by(|a, b| {
        //         let x = a.bounding_box(time0, time1).unwrap().minmum.get(axis);
        //         let y = b.bounding_box(time0, time1).unwrap().maxmum.get(axis);
        //         x.partial_cmp(&y).unwrap()
        //     });
        //     let mid = span / 2;
        //     let (objectspre, objectssuf) = objects.split_at_mut(mid);
        //     left = Arc::new(BVHNODE::new(objectspre.to_vec(), mid, time0, time1));
        //     right = Arc::new(BVHNODE::new(objectssuf.to_vec(), span - mid, time0, time1));
        // }
        // let box0 = left.bounding_box(time0, time1).unwrap();
        // let box1 = right.bounding_box(time0, time1).unwrap();
        // Self {
        //     left: left,
        //     right: right,
        //     box1: AABB::surrounding_box(box0, box1),
        // }
    }

    pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        let box_a = AABB::new(Vec3::zero(), Vec3::zero());
        let box_b = AABB::new(Vec3::zero(), Vec3::zero());
        if let Some(_rec1) = a.bounding_box(0.0, 0.0) {
        } else {
            println!("no bounding box in bvh node constructor\n");
        }
        if let Some(_rec1) = b.bounding_box(0.0, 0.0) {
        } else {
            println!("no bounding box in bvh node constructor\n");
        }

        box_a.minmum.x < box_b.minmum.x
    }

    pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        let box_a = AABB::new(Vec3::zero(), Vec3::zero());
        let box_b = AABB::new(Vec3::zero(), Vec3::zero());
        if let Some(_rec1) = a.bounding_box(0.0, 0.0) {
        } else {
            println!("no bounding box in bvh node constructor\n");
        }
        if let Some(_rec1) = b.bounding_box(0.0, 0.0) {
        } else {
            println!("no bounding box in bvh node constructor\n");
        }

        box_a.minmum.y < box_b.minmum.y
    }
    pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> bool {
        let box_a = AABB::new(Vec3::zero(), Vec3::zero());
        let box_b = AABB::new(Vec3::zero(), Vec3::zero());
        if let Some(_rec1) = a.bounding_box(0.0, 0.0) {
        } else {
            println!("no bounding box in bvh node constructor\n");
        }
        if let Some(_rec1) = b.bounding_box(0.0, 0.0) {
        } else {
            println!("no bounding box in bvh node constructor\n");
        }

        box_a.minmum.z < box_b.minmum.z
    }
    // pub fn new(list:&Hittable_list,time0:f64,time1:f64) -> Self{
    //     Self{

    //     }
    // }
}
