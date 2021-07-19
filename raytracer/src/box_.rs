use std::sync::Arc;

use crate::{Vec3, aabb::AABB, aarec::{xy_rect, xz_rect, yz_rect}, hittable::{self, Hittable}, hittable_list::{self, Hittable_list}, materia::material};

pub struct Box_{
    box_min:Vec3,
    box_max:Vec3,
    sides:Hittable_list,
}

impl Box_{
    pub fn new(p0:Vec3,p1:Vec3,ptr:Arc<dyn material>) -> Self{
        let mut s = Hittable_list::new();
        s.add(Arc::new(xy_rect::new(p0.x,p1.x,p0.y,p1.y,p1.z,ptr.clone())));
        s.add(Arc::new(xy_rect::new(p0.x,p1.x,p0.y,p1.y,p0.z,ptr.clone())));

        s.add(Arc::new(xz_rect::new(p0.x,p1.x,p0.z,p1.z,p1.y,ptr.clone())));
        s.add(Arc::new(xz_rect::new(p0.x,p1.x,p0.z,p1.z,p0.y,ptr.clone())));

        s.add(Arc::new(yz_rect::new(p0.y,p1.y,p0.z,p1.z,p1.x,ptr.clone())));
        s.add(Arc::new(yz_rect::new(p0.y,p1.y,p0.z,p1.z,p0.x,ptr.clone())));

        Self{
            box_min:p0,
            box_max:p1,
            sides:s,
        }
    }
}

impl Hittable for Box_{
    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64) -> Option<hittable::hit_record> {
        return self.sides.hit(r, t_min, t_max);
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}