use std::sync::Arc;

use crate::{aabb::AABB, hittable::Hittable};

pub struct BVHNODE {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box1: AABB,
}
impl BVHNODE{
    // to do 
}