#[allow(clippy::float_cmp)]

use crate::vec3::Vec3;
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct hit_record{
    pub p:Vec3,
    pub normal: Vec3,
    pub t: f64,
}

