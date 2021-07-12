use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq,Copy)]
pub struct Ray{
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray{
    pub fn new(orig: Vec3,dir: Vec3) -> Self {
        Self {orig,dir}
    }
    pub fn zero() -> Self{
        Self::new(Vec3::zero(),Vec3::zero())
    }
    pub fn at(&self,x:f64) -> Vec3{
        self.orig.clone() + self.dir.clone() * x
    }
}