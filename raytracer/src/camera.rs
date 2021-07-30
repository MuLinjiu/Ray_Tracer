use std::f64::consts::PI;

use crate::rtweekend::random_double2;
use crate::vec3::Vec3;
use crate::{ray::Ray, vec3::random_in_unit_disk};
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
    _time0: f64,
    _time1: f64,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        //const ASPECT_RATIO:f64 = 16.0 / 9.0;
        let viewpoint_height: f64 = 2.0 * h;
        let viewpoint_width: f64 = aspect_ratio * viewpoint_height;

        let w_ = Vec3::unit(lookfrom - lookat);
        let u_ = Vec3::unit(Vec3::cross(vup, w_));
        let v_ = Vec3::cross(w_, u_);

        let _focal_length: f64 = 1.0;
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
            _time0: time0,
            _time1: time1,
        }
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            random_double2(self._time0, self._time1),
        )
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
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
