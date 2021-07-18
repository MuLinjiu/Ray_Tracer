use std::sync::Arc;

use crate::Vec3;

pub trait Texture{
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}
pub struct solid_color{
    color_value:Vec3,
}

impl solid_color{
    pub fn new(c:Vec3) -> Self{
        Self{
            color_value:c,
        }
    }
    fn new1(red:f64,green:f64,blue:f64) -> Self{
        Self{
            color_value:Vec3::new(red,green,blue),
        }
    }
}

impl Texture for solid_color{
    fn value(&self,u:f64,v:f64, p:&Vec3) -> Vec3{
    return self.color_value;
    }
}

pub struct BaseColor {
    color: Vec3,
}

impl BaseColor {
    pub fn vectobase(color: Vec3) -> Self {
        Self {
            color
        }
    }
}

impl Texture for BaseColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.color
    }
}


pub  struct checker_texture{
    odd:Arc<dyn Texture>,
    even:Arc<dyn Texture>,
}

impl checker_texture{
    pub fn new(_even:Vec3,_odd:Vec3) -> Self{
        Self{
            even:Arc::new(BaseColor::vectobase(_even)),
            odd:Arc::new(BaseColor::vectobase(_odd)),
        }
    }
}

impl Texture for checker_texture{
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3{
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        }else {
            return self.even.value(u, v, p);
        }
    }
}