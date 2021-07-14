use crate::vec3::Vec3;
use crate::{
    ray::Ray,
    vec3::{random_in_unit_disk, random_in_unit_sphere},
};
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        //const ASPECT_RATIO:f64 = 16.0 / 9.0;
        let viewpoint_height: f64 = 2.0 * h;
        let viewpoint_width: f64 = aspect_ratio * viewpoint_height;

        let w_ = Vec3::unit(lookfrom - lookat);
        let u_ = Vec3::unit(Vec3::cross(vup, w_));
        let v_ = Vec3::cross(w_, u_);

        let focal_length: f64 = 1.0;
        Self {
            u: u_,
            v: v_,
            w: w_,
            origin: lookfrom,
            horizontal: u_ * viewpoint_width * focus_dist,
            vertical: v_ * viewpoint_height * focus_dist,
            lower_left_corner: lookfrom
                - u_ * viewpoint_width * focus_dist / 2.0
                - v_ * viewpoint_height * focus_dist / 2.0
                - w_ * focus_dist,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        );
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * 3.1415926535897932385 / 180.0;
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
