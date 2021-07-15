use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub time:f64,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3,time:f64) -> Self {
        Self { orig, dir ,time}
    }
    pub fn zero() -> Self {
        Self::new(Vec3::zero_(), Vec3::zero(),0.0)
    }
    pub fn at(&self, x: f64) -> Vec3 {
        self.orig + self.dir * x
    }
}
