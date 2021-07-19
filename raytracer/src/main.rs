mod aabb;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod materia;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;
mod aarec;
mod box_;
mod constant_medium;
#[allow(clippy::float_cmp)]
mod vec3;

use bvh::BVHNODE;
use camera::clamp;
use hittable::hit_record;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{random, Rng};
use rtweekend::random_double2;
use sphere::moving_sphere;
use std::{f32::INFINITY, mem::zeroed, rc::Rc, sync::Arc, vec};
use texture::checker_texture;
use vec3::random_in_unit_sphere;

use crate::{aarec::{rotate_y, translate, xy_rect, xz_rect, yz_rect}, box_::Box_, camera::Camera, constant_medium::Constant_medium, hittable::Hittable, hittable_list::Hittable_list, materia::{dielectric, diffuse_light, lambertian, metal}, rtweekend::random_double, sphere::Sphere, texture::{image_texture, noise_texture}};
pub use ray::Ray;
pub use vec3::Vec3;

pub fn random_scene(world: &mut Hittable_list) {
    //let mut world:Hittable_list = Hittable_list::new();
    // let ground_material = Arc::new(lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere::new(
    //     Vec3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     ground_material,
    // )));

    let checker = Arc::new(checker_texture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(lambertian::new1(checker)),
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
                    let center2 = center + Vec3::new(0.0, random_double2(0.0, 0.5), 0.0);
                    world.add(Arc::new(moving_sphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
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

pub fn final_scene(world: &mut Hittable_list){
    let mut boxes1 = Hittable_list::new();
    let ground = Arc::new(lambertian::new(Vec3::new(0.48,0.83,0.53)));
    let boxes_per_side = 20;

    for i in 0..boxes_per_side{
        for j in 0..boxes_per_side{
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double2(1.0, 101.0);
            let z1 = z0 + w;

            world.add(Arc::new(Box_::new(Vec3::new(x0,y0,z0),Vec3::new(x1,y1,z1),ground.clone())));

        }
    }
    //world.add(Arc::new(BVHNODE::new(boxes1.objects.clone(),boxes1.objects.len(),0.0,1.0)));

    let light = Arc::new(diffuse_light::new1(Vec3::new(7.0,7.0,7.0)));
    world.add(Arc::new(xz_rect::new(123.0,423.0,147.0,412.0,554.0,light)));

    let center1 = Vec3::new(400.0,400.0,200.0);
    let center2 = center1 + Vec3::new(30.0,0.0,0.0);
    let moving_sphere_material = Arc::new(lambertian::new(Vec3::new(0.7,0.3,0.1)));
    world.add(Arc::new(moving_sphere::new(center1,center2,0.0,1.0,50.0,moving_sphere_material)));

    world.add(Arc::new(Sphere::new(Vec3::new(260.0,150.0,45.0),50.0,Arc::new(dielectric::new(1.5)))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0,150.0,145.0),50.0,Arc::new(metal::new(Vec3::new(0.8,0.8,0.9),1.0)))));

    let boundary = Arc::new(Sphere::new(Vec3::new(360.0,150.0,145.0),70.0,Arc::new(dielectric::new(1.5))));
    world.add(boundary.clone());
    world.add(Arc::new(Constant_medium::new1(boundary.clone(),0.2,Vec3::new(0.2,0.4,0.9))));
    let boundary = Arc::new(Sphere::new(Vec3::zero(),5000.0,Arc::new(dielectric::new(1.5))));
    world.add(Arc::new(Constant_medium::new1(boundary,0.0001,Vec3::ones())));

    let emat = Arc::new(lambertian::new1(Arc::new(image_texture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(Vec3::new(400.0,200.0,400.0),100.0,emat)));

    let pertext = Arc::new(noise_texture::new1(0.1));
    world.add(Arc::new(Sphere::new(Vec3::new(220.0,280.0,300.0),80.0,Arc::new(lambertian::new1(pertext)))));

    let white = Arc::new(lambertian::new(Vec3::new(0.73,0.73,0.73)));
    let ns = 1000;
    for j in 0..ns{
        world.add(Arc::new(Sphere::new(Vec3::new(random_double2(0.0,165.0),random_double2(0.0,165.0),random_double2(0.0,165.0)),10.0,white.clone())));
    }

}


pub fn color(r: &Ray, background:&Vec3,world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    let t = world.hit(&r, 0.001, INFINITY as f64);
    if let Some(rec_) = t {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let mut attenuation = Vec3::zero();
        let mut emitted = rec_.mat_ptr.emitted(rec_.u.clone(),rec_.v.clone(),&rec_.p.clone());
        //println!("{},{},{}\n",emitted.x,emitted.y,emitted.z);
        if !rec_
            .mat_ptr
            .scatter(r, &rec_, &mut attenuation, &mut scattered)
        {
            //println!("{},{},{}\n",attenuation.x,attenuation.y,attenuation.z);
            //return Vec3::cross(color(&scattered, world, depth - 1),attenuation);
            // let t = color(&scattered, background,world, depth - 1);
            // return Vec3::new(
            //     t.x * attenuation.x,
            //     t.y * attenuation.y,
            //     t.z * attenuation.z,
            // );
            return emitted;
            //return color(&scattered, background,world, depth - 1);
        }
        let t = color(&scattered, background, world, depth - 1);
        return emitted + Vec3::new(t.x * attenuation.x,t.y * attenuation.y,t.z * attenuation.z);
        //return emitted + color(&scattered, background, world, depth - 1) * attenuation;
    } else {
        // let unit_direction = Vec3::unit(r.dir);
        // let t = 0.5 * (unit_direction.y + 1.0);
        // Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        return background.clone();
    }
}

fn main() {
    const MAX_DEPTH: i32 = 50;
    //const ASPECT_RATIO: f64 = 16.0 / 9.0;
    //5
    const ASPECT_RATIO: f64 = 1.0;
    //const IMAGE_WIDTH: i32 = 400;
    //5
    //const IMAGE_WIDTH: i32 = 600;
    //6
    const IMAGE_WIDTH: i32 = 800;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    let mut lookfrom = Vec3::new(12.0, 2.0, 3.0);
    //let lookfrom = Vec3::new(15.0, 0.0, 12.0);
    let mut lookat = Vec3::new(0.0, 0.0, 0.0);
    let mut vup = Vec3::new(0.0, 1.0, 0.0);
    let mut dist_to_focus = 10.0;
    let mut aperture = 0.1;
    let mut world: Hittable_list = Hittable_list::new();
    let mut vfov = 20.0;
    let mut background = Vec3::zero();
    let x = 6;
    if x == 0 {
        random_scene(&mut world);
        background = Vec3::new(0.7,0.8,1.0);
    } else if x == 1 {
        let checker = Arc::new(checker_texture::new(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        ));
        aperture = 0.0;
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            Arc::new(lambertian::new1(checker.clone())),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            Arc::new(lambertian::new1(checker)),
        )));
        background = Vec3::new(0.7,0.8,1.0);
    } else if x == 2 {
        let pertext = Arc::new(noise_texture::new1(4.0));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(lambertian::new1(pertext.clone())),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::new(lambertian::new1(pertext)),
        )));
        lookfrom.x = 13.0;
        background = Vec3::new(0.7,0.8,1.0);
    } else if x == 3 {
        lookfrom.x = 13.0;
        let earth_texture = Arc::new(image_texture::new("earthmap.jpg"));
        //let earth_texture = Arc::new(image_texture::new("tjm.jpg"));
        let erath_surface = Arc::new(lambertian::new1(earth_texture));
        let golbe = Arc::new(Sphere::new(Vec3::zero(), 2.0, erath_surface));
        world.add(golbe);
        background = Vec3::new(0.7,0.8,1.0);
    }else if x == 4{
        background = Vec3::zero();
        lookfrom.x = 26.0;
        lookfrom.y = 3.0;
        lookfrom.z = 6.0;
        lookat.y = 2.0;
        let pertext = Arc::new(noise_texture::new1(4.0));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(lambertian::new1(pertext.clone())),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::new(lambertian::new1(pertext)),
        )));

        let difflight = Arc::new(diffuse_light::new1(Vec3::new(4.0,4.0,4.0)));
        world.add(Arc::new(xy_rect::new(3.0,5.0,1.0,3.0,-2.0,difflight)));

    }else if x == 5 {
        let red = Arc::new(lambertian::new(Vec3::new(0.65,0.05,0.05)));
        let white = Arc::new(lambertian::new(Vec3::new(0.73,0.73,0.73)));
        let green = Arc::new(lambertian::new(Vec3::new(0.12,0.45,0.15)));
        let light = Arc::new(diffuse_light::new1(Vec3::new(15.0,15.0,15.0)));

        world.add(Arc::new(yz_rect::new(0.0,555.0,0.0,555.0,555.0,green)));
        world.add(Arc::new(yz_rect::new(0.0,555.0,0.0,555.0,0.0,red)));
        world.add(Arc::new(xz_rect::new(213.0,343.0,227.0,332.0,554.0,light)));
        world.add(Arc::new(xz_rect::new(0.0,555.0,0.0,555.0,0.0,white.clone())));
        world.add(Arc::new(xz_rect::new(0.0,555.0,0.0,555.0,555.0,white.clone())));
        world.add(Arc::new(xy_rect::new(0.0,555.0,0.0,555.0,555.0,white.clone())));

        // world.add(Arc::new(Box_::new(Vec3::new(130.0,0.0,655.0),Vec3::new(295.0,165.0,230.0),white.clone())));
        // world.add(Arc::new(Box_::new(Vec3::new(265.0,0.0,295.0),Vec3::new(430.0,330.0,460.0),white.clone())));
        
        let box1 = Arc::new(Box_::new(Vec3::zero(),Vec3::new(165.0,330.0,165.0),white.clone()));
        let box1_ = Arc::new(rotate_y::new(box1,15.0));
        let box1__ = Arc::new(translate::new(box1_,Vec3::new(265.0,0.0,295.0)));
        world.add(Arc::new(Constant_medium::new1(box1__,0.01,Vec3::zero())));

        let box2 = Arc::new(Box_::new(Vec3::zero(),Vec3::new(165.0,165.0,165.0),white.clone()));
        let box2_ = Arc::new(rotate_y::new(box2,-18.0));
        let box2__ = Arc::new(translate::new(box2_,Vec3::new(130.0,0.0,65.0)));
        world.add(Arc::new(Constant_medium::new1(box2__,0.01,Vec3::ones())));

        background = Vec3::zero();
        lookfrom.x = 278.0;
        lookfrom.y = 278.0;
        lookfrom.z = -800.0;

        lookat.x = 278.0;
        lookat.y = 278.0;
        lookat.z = 0.0;
        vfov = 40.0;
    }else if x == 6{
        final_scene(&mut world);
        lookfrom = Vec3::new(478.0, 278.0, -600.0);
        lookat = Vec3::new(278.0,278.0,0.0);
        vfov = 40.0;
        background = Vec3::zero();
    }
    //random_scene(&mut world);

    // let checker =  Arc::new(checker_texture::new(Vec3::new(0.2,0.3,0.1),Vec3::new(0.9,0.9,0.9)));
    // aperture = 0.0;
    // world.add(Arc::new(Sphere::new(
    //     Vec3::new(0.0, -10.0, 0.0),
    //     10.0,
    //     Arc::new(lambertian::new1(checker.clone())),
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Vec3::new(0.0, 10.0, 0.0),
    //     10.0,
    //     Arc::new(lambertian::new1(checker)),
    // )));
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

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
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
                pixel_color += color(&r, &background,&world, MAX_DEPTH);
            }
            let mut r = pixel_color.x;
            let mut g = pixel_color.y;
            let mut b = pixel_color.z;
        
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
