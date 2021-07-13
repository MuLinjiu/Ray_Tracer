#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
//use std::intrinsics::ceilf32;

// use std::intrinsics::discriminant_value;

use std::{f32::INFINITY, vec};

use hittable::hit_record;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use ray::Ray;

use crate::{hittable::Hittable, hittable_list::Hittable_list, sphere::Sphere};

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

// fn hit1(r:Ray,t_min:f64,t_max:f64,rec:&hit_record,world:&Vec<Sphere>) -> bool{
//     let mut temp_rec = hit_record::new();
//         let mut hit_anything = false;
//         let mut closest_so_far = t_max;
//         for object in world.iter()
//         {
//             if object.hit(r, t_min, closest_so_far, temp_rec) {
//                 hit_anything = true;
//                 closest_so_far = temp_rec.t;
//                 rec = &temp_rec;
//             }
//         }
//         return hit_anything;
// }

fn color(r:&Ray,world:&dyn Hittable) -> Vec3{
    //let rec = hit_record::new();
    // if world.hit(r,0.0,9999999999.0,&rec){
    // }
    let t = world.hit(&r,0.0,INFINITY as f64);
    if let Some(rec_) = t{
        //println!("{}\n",rec_.normal.x);
        return (rec_.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }else{
        let unit_direction = Vec3::unit(r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5,0.7,1.0) * t
    }
    
}
// fn color1(r:&Ray,s1:Sphere) -> Vec3{
//     //let rec = hit_record::new();
//     // if world.hit(r,0.0,9999999999.0,&rec){
//     // }
//     let t = s1.hit(&r,0.0,INFINITY as f64);
//     if let Some(rec_) = t{
//         //println!("{}\n",rec_.normal.x);
//         // let n = Vec3::unit(rec_.p - Vec3::new(0.0,0.0,-1.0));
//         //  return Vec3::new(n.x + 1.0,n.y + 1.0,n.z + 1.0) * 0.5;
//         return Vec3::new(rec_.normal.x + 1.0,rec_.normal.y + 1.0,rec_.normal.z + 1.0) * 0.5;
//     }else{
//         let unit_direction = Vec3::unit(r.dir);
//         let t = 0.5 * (unit_direction.y + 1.0);
//         Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5,0.7,1.0) * t
//     }
    
// }
fn main() {
    const ASPECT_RATIO:f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let mut world:Hittable_list = Hittable_list::new();
    let s1 = Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5));
    let s2 = Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0));
    world.add(s1);
    world.add(s2);

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
            let pixel_color = color(&r,&world);
            //let pixel_color = color1(&r,s1);

            
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
