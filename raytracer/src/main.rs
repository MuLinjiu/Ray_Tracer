#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod rtweekend;
mod camera;

use rand::{Rng, random};
use std::{f32::INFINITY, vec};
use camera::clamp;
use hittable::hit_record;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use ray::Ray;
use crate::{camera::Camera, hittable::Hittable, hittable_list::Hittable_list, rtweekend::random_double, sphere::Sphere};

fn hit_sphere(center:Vec3, radius:f64, r:Ray) -> f64{
    let oc:Vec3 = r.orig - center;
    let a: f64 = Vec3::dot(r.dir,r.dir);
    let half_b: f64 = Vec3::dot(r.dir,oc);
    let c: f64 = Vec3::len(oc) - radius * radius;
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


fn color(r:&Ray,world:&dyn Hittable) -> Vec3{
    let t = world.hit(&r,0.0,INFINITY as f64);
    if let Some(rec_) = t{
        return (rec_.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }else{
        let unit_direction = Vec3::unit(r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5,0.7,1.0) * t
    }
    
}

fn main() {
    const ASPECT_RATIO:f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL:i32 = 100;

    let mut world:Hittable_list = Hittable_list::new();
    let s1 = Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5));
    let s2 = Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0));
    world.add(s1);
    world.add(s2);

    let cam = Camera::new();

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let bar = ProgressBar::new(IMAGE_WIDTH as u64);
    
    for j in 0..IMAGE_WIDTH{
        for i in 0..IMAGE_HEIGHT {
            let pixel = img.get_pixel_mut(j as u32, i as u32);
            let mut pixel_color = Vec3::zero();
            for s in 0..SAMPLES_PER_PIXEL{
                let u =  (((j as f64 + random_double(0.0,100.0)) as f64) / (IMAGE_WIDTH as f64 - 1.0)) as f64;
                let v =  (((IMAGE_HEIGHT - i) as f64 - random_double(0.0,100.0)as f64) / (IMAGE_HEIGHT as f64 - 1.0)) as f64;
                let r  =cam.get_ray(u, v);
                pixel_color += color(&r,&world);
            }
            let mut r = pixel_color.x ;
            let mut g = pixel_color.y ;
            let mut b = pixel_color.z;
            let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);
            r *= scale;
            g *= scale;
            b *= scale;
            let ir = (256.0 * clamp(r,0.0,0.999)) as u8;
            let ig = (256.0 * clamp(g,0.0,0.999)) as u8;
            let ib = (256.0 * clamp(b,0.0,0.999)) as u8;
            *pixel = image::Rgb([ir, ig, ib]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
    


    

}
