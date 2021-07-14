mod camera;
mod hittable;
mod hittable_list;
mod materia;
mod ray;
mod rtweekend;
mod sphere;
#[allow(clippy::float_cmp)]
mod vec3;

use camera::clamp;
use hittable::hit_record;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{random, Rng};
use rtweekend::random_double2;
use std::{f32::INFINITY, mem::zeroed, rc::Rc, sync::Arc, vec};
use vec3::random_in_unit_sphere;

use crate::{
    camera::Camera,
    hittable::Hittable,
    hittable_list::Hittable_list,
    materia::{dielectric, lambertian, metal},
    rtweekend::random_double,
    sphere::Sphere,
};
pub use ray::Ray;
pub use vec3::Vec3;

pub fn random_scene(world: &mut Hittable_list) {
    //let mut world:Hittable_list = Hittable_list::new();
    let ground_material = Arc::new(lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(1.0, 100.0);
            let center = Vec3::new(
                a as f64 + 0.9 * random_double(1.0, 100.0),
                0.2,
                b as f64 + 0.9 * random_double(1.0, 100.0),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo1 = Vec3::random();
                    let albedo2 = Vec3::random();
                    let albedo = Vec3::new(
                        albedo1.x * albedo2.x,
                        albedo1.y * albedo2.y,
                        albedo1.z * albedo2.z,
                    );
                    let sphere_material = Arc::new(lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_(0.5, 1.0);
                    let fuzz = random_double2(0.0, 0.5);
                    let sphere_material = Arc::new(metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    //return world;
}

pub fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    let t = world.hit(&r, 0.001, INFINITY as f64);
    if let Some(rec_) = t {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero();
        if rec_
            .mat_ptr
            .scatter(r, &rec_, &mut attenuation, &mut scattered)
        {
            //println!("{},{},{}\n",attenuation.x,attenuation.y,attenuation.z);
            //return Vec3::cross(color(&scattered, world, depth - 1),attenuation);
            let t = color(&scattered, world, depth - 1);
            return Vec3::new(
                t.x * attenuation.x,
                t.y * attenuation.y,
                t.z * attenuation.z,
            );
            //return color(&scattered, world, depth - 1);
        }
        return Vec3::zero();
    } else {
        let unit_direction = Vec3::unit(r.dir);
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    const MAX_DEPTH: i32 = 50;
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    let mut world: Hittable_list = Hittable_list::new();
    random_scene(&mut world);
    // let material_ground = Arc::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0,Arc::new(lambertian::new(Vec3::new(0.8,0.8,0.0)))));
    // let material_center = Arc::new(Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5,Arc::new(lambertian::new(Vec3::new(0.1,0.2,0.5)))));
    // let material_left = Arc::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0),0.5,Arc::new(dielectric::new(1.5))));
    // let material_right = Arc::new(Sphere::new(Vec3::new(1.0,0.0,-1.0),0.5,Arc::new(metal::new(Vec3::new(0.8,0.6,0.2),0.0))));
    // let material_left2 = Arc::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0),-0.45,Arc::new(dielectric::new(1.5))));

    //  world.add(material_ground);
    // world.add(material_center);
    // world.add(material_left);
    // world.add(material_left2);
    // world.add(material_right);

    let lookfrom = Vec3::new(12.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let bar = ProgressBar::new(IMAGE_WIDTH as u64);

    for j in 0..IMAGE_WIDTH {
        for i in 0..IMAGE_HEIGHT {
            let pixel = img.get_pixel_mut(j as u32, i as u32);
            let mut pixel_color = Vec3::zero();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (((j as f64 + random_double(0.0, 100.0)) as f64)
                    / (IMAGE_WIDTH as f64 - 1.0)) as f64;
                let v = (((IMAGE_HEIGHT - i) as f64 - random_double(0.0, 100.0) as f64)
                    / (IMAGE_HEIGHT as f64 - 1.0)) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += color(&r, &world, MAX_DEPTH);
            }
            let mut r = pixel_color.x;
            let mut g = pixel_color.y;
            let mut b = pixel_color.z;
            //println!("{},{},{}\n",r,g,b);
            let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);
            r = (r * scale).sqrt();
            g = (g * scale).sqrt();
            b = (b * scale).sqrt();
            //println!("{},{},{}\n",r,b,g);
            let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
            let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
            let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
            *pixel = image::Rgb([ir, ig, ib]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
