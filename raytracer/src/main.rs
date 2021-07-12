#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod hittable;
mod sphere;
//use std::intrinsics::ceilf32;

// use std::intrinsics::discriminant_value;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use ray::Ray;

fn hit_sphere(center:Vec3, radius:f64, r:Ray) -> f64{
    let oc:Vec3 = r.orig - center;
    let a: f64 = Vec3::dot(r.dir,r.dir);
    let half_b: f64 = Vec3::dot(r.dir,oc);
    let c: f64 = Vec3::modlen(oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    else{
        let t1 = (-half_b - discriminant.sqrt()) / a;
        if t1 > 0.0 {
            return t1;
        }
        let t2 = (-half_b + discriminant.sqrt()) / a;
        if t2 > 0.0 { 
            return t2;
        }
        return -1.0;
    }
}

fn color(r:Ray) -> Vec3{
     let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) ;
     if t > 0.0{
         let n = Vec3::unit(r.at(t) - Vec3::new(0.0,0.0,-1.0));
         return Vec3::new(n.x + 1.0,n.y + 1.0,n.z + 1.0) * 0.5;
     }
        //return Vec3::new(1.0,0.0,0.0);
    
    let unit_direction = Vec3::unit(r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5,0.7,1.0) * t
}
fn main() {
    const ASPECT_RATIO:f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let viewpoint_height:f64 = 2.0;
    let viewpoint_width:f64 = ASPECT_RATIO * viewpoint_height;
    let focal_length:f64 = 1.0;

    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewpoint_width,0.0,0.0);
    let vertical = Vec3::new(0.0,viewpoint_height,0.0);

    let lower_left_corner = origin - horizontal / 2.0  - vertical / 2.0 -  Vec3::new(0.0,0.0,focal_length); 

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let bar = ProgressBar::new(IMAGE_WIDTH as u64);

    for j in 0..IMAGE_WIDTH{
        for i in 0..IMAGE_HEIGHT {
            let pixel = img.get_pixel_mut(j as u32, i as u32);
            let u =  ((j as f64) / (IMAGE_WIDTH as f64 - 1.0)) as f64;
            let v =  (((IMAGE_HEIGHT - i) as f64) / (IMAGE_HEIGHT as f64 - 1.0)) as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical  * v - origin);
            let pixel_color = color(r);
            
			let ir = (255.999 * pixel_color.x) as u8;
			let ig = (255.999 * pixel_color.y) as u8;
			let ib = (255.999 * pixel_color.z) as u8;
            *pixel = image::Rgb([ir, ig, ib]);
        }
        bar.inc(1);
    }



    img.save("output/test.png").unwrap();
    bar.finish();
}
