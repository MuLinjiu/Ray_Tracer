#[allow(clippy::float_cmp)]

use crate::vec3::Vec3;
use crate::ray::Ray;
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct Camera{
    origin: Vec3,
    lower_left_corner:Vec3,
    horizontal:Vec3,
    vertical:Vec3,
}

impl Camera{
    pub fn new() -> Self{
        const ASPECT_RATIO:f64 = 16.0 / 9.0;
        let viewpoint_height:f64 = 2.0;
        let viewpoint_width:f64 = ASPECT_RATIO * viewpoint_height;
        let focal_length:f64 = 1.0;
        Self{
            origin : Vec3::new(0.0,0.0,0.0),
            horizontal : Vec3::new(viewpoint_width,0.0,0.0),
            vertical : Vec3::new(0.0,viewpoint_height,0.0),
            lower_left_corner : Vec3::new(0.0,0.0,0.0) - Vec3::new(viewpoint_width,0.0,0.0) / 2.0  - Vec3::new(0.0,viewpoint_height,0.0) / 2.0 -  Vec3::new(0.0,0.0,focal_length),

        }
    }

    pub fn get_ray(self,u:f64,v:f64) -> Ray{
        return Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin);
    }
}

pub fn clamp(x:f64,min:f64,max:f64) -> f64{
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}