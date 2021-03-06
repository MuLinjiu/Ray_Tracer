#[deny(clippy::float_cmp)]
mod aabb;
mod aarec;
mod box_;
mod bvh;
mod camera;
mod constant_medium;
mod hittable;
mod hittable_list;
mod materia;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rtweekend;
mod sphere;
mod texture;

#[allow(clippy::float_cmp)]
mod vec3;

use bvh::BVHNODE;
use camera::clamp;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use materia::ScatterRecord;
use pdf::{HittablePdf, MixturePdf, Pdf};
use rtweekend::random_double2;
use sphere::MovingSphere;
// use std::{f64::INFINITY, sync::{mpsc::channel, Arc}};
use std::{
    f64::INFINITY,
    sync::{mpsc::channel, Arc},
};
use texture::CheckerTexture;

use threadpool::ThreadPool;

//use crate::{aarec::{RotateY, Translate, XyRect, XzRect, YzRect}, box_::Box_, camera::Camera, constant_medium::ConstantMedium, hittable::Hittable, hittable_list::HittableList, materia::{Dielectric, DiffuseLight, Lambertian, Metal}, onb::FlipFace, rtweekend::random_double, sphere::Sphere, texture::{ImageTexture, NoiseTexture}};
use crate::{
    aarec::{RotateY, Translate, Triangle, XyRect, XzRect, YzRect},
    box_::Box_,
    camera::Camera,
    constant_medium::ConstantMedium,
    hittable::Hittable,
    hittable_list::HittableList,
    materia::{Dielectric, DiffuseLight, Lambertian, Metal},
    onb::FlipFace,
    rtweekend::random_double,
    sphere::Sphere,
    texture::{ImageTexture, NoiseTexture},
};
pub use ray::Ray;
pub use vec3::Vec3;

pub fn random_scene(world: &mut HittableList) {
    //let mut world:Hittable_list = Hittable_list::new();
    // let ground_material = Arc::new(lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere::new(
    //     Vec3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     ground_material,
    // )));

    let checker = Arc::new(CheckerTexture::new(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new1(checker)),
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
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_double2(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
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
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    //return world;
}

pub fn final_scene(world: &mut HittableList) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Vec3::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0) * 100.0 + 1.0;
            let z1 = z0 + w;

            boxes1.add(Arc::new(Box_::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    world.add(Arc::new(BVHNODE::new(
        &boxes1.objects,
        0,
        boxes1.objects.len(),
        0.0,
        1.0,
    )));

    let light = Arc::new(DiffuseLight::new1(Vec3::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new1(
        boundary,
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));

    let boundary = Arc::new(Sphere::new(
        Vec3::zero(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new1(
        boundary,
        0.0001,
        Vec3::ones(),
    )));

    let emat = Arc::new(Lambertian::new1(Arc::new(ImageTexture::new(
        "earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Arc::new(NoiseTexture::new1(0.1));
    world.add(Arc::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new1(pertext)),
    )));
    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Vec3::new(
                random_double2(0.0, 165.0),
                random_double2(0.0, 165.0),
                random_double2(0.0, 165.0),
            ),
            10.0,
            white.clone(),
        )));
    }
    let tp = Arc::new(BVHNODE::new(
        &boxes2.objects,
        0,
        boxes2.objects.len(),
        0.0,
        1.0,
    ));
    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(tp, 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));
}

pub fn color(
    r: &Ray,
    background: &Vec3,
    world: &dyn Hittable,
    lights: &Arc<HittableList>,
    depth: i32,
) -> Vec3 {
    if depth <= 0 {
        //println!("qq");
        return Vec3::zero();
    }
    let t = world.hit(&r, 0.0001, INFINITY);
    if let Some(rec_) = t {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), 0.0);
        let mut attenuation = Vec3::zero();
        //let mut pdf = 0.0;
        let mut srec = ScatterRecord::new();
        let emitted = rec_
            .mat_ptr
            .emitted(r, &rec_, rec_.u, rec_.v, &rec_.p.clone());
        //if emitted.x != 0.0{println!("{},{},{}\n",emitted.x,emitted.y,emitted.z);}
        if !rec_
            .mat_ptr
            .scatter(r, &rec_, &mut attenuation, &mut scattered, &mut srec)
        {
            //println!("{},{},{}\n",attenuation.x,attenuation.y,attenuation.z);
            //return Vec3::cross(color(&scattered, world, depth - 1),attenuation);
            // let t = color(&scattered, background,world, depth - 1);
            // return Vec3::new(
            //     t.x * attenuation.x,
            //     t.y * attenuation.y,
            //     t.z * attenuation.z,
            // );
            //println!("1\n");
            //println!("{},{},{}",emitted.x,emitted.y,emitted.z);
            return emitted;
            //return color(&scattered, background,world, depth - 1);
        }
        if srec.is_specular {
            let t = color(
                &srec.specular_ray,
                background,
                world,
                &lights.clone(),
                depth - 1,
            );
            return Vec3::new(
                srec.attenuation.x * t.x,
                srec.attenuation.y * t.y,
                srec.attenuation.z * t.z,
            );
        }
        // let on_light = Vec3::new(random_double2(213.0,343.0), 554.0, random_double2(227.0,332.0));
        // let mut to_light = on_light - rec_.p;
        // let distance_squared = to_light.len_squared();
        // to_light = to_light.unit();

        // if Vec3::dot(to_light,rec_.normal) < 0.0{
        //     return emitted;
        // }

        // let light_area = ((343 - 213) * (332 - 227)) as f64;
        // let light_cosine = to_light.y.abs();
        // if light_cosine < 0.000001 {
        //     return emitted;
        // }
        // //let pdf = rec_.mat_ptr.get_pdf_value(&rec_, &mut scattered);
        // let pdf = distance_squared / (light_cosine * light_area);
        // //scattered = Ray::new()
        // scattered.orig = rec_.p;
        // scattered.dir = to_light;
        // scattered.time = r.time;

        // let light_pdf = HittablePdf::new(lights.clone(),rec_.p);
        // scattered.orig = rec_.p;
        // scattered.dir = light_pdf.generate();
        // scattered.time = r.time;
        // let pdf_value = light_pdf.value(&scattered.dir);

        //  let p0 = Arc::new(HittablePdf::new(lights.clone(),rec_.p));
        // let p1 = Arc::new(CosinePdf::new(&rec_.normal));
        // let mixed_pdf = MixturePdf::new(p0,p1);
        // scattered.orig = rec_.p;
        // scattered.dir = mixed_pdf.generate();
        // scattered.time = r.time;
        // let pdf_value = mixed_pdf.value(&scattered.dir);

        let light_ptr = Arc::new(HittablePdf::new(lights.clone(), rec_.p));
        let p = MixturePdf::new(light_ptr, srec.pdf_ptr);
        scattered.orig = rec_.p;
        scattered.dir = p.generate();
        scattered.time = r.time;
        let pdf_value = p.value(&scattered.dir);
        // if pdf_value != 0.0 {
        //     println!("{}",pdf_value);
        // }

        //println!("2\n");
        let t = color(&scattered, background, world, &lights.clone(), depth - 1);
        //println!("{},{},{}",srec.attenuation.x,srec.attenuation.y,srec.attenuation.z);
        // if rec_.mat_ptr.scattering_pdf(r, &rec_, &mut scattered) == 0.0 {
        //     println!("{}",rec_.mat_ptr.scattering_pdf(r, &rec_, &mut scattered));
        // }
        emitted
            + Vec3::new(
                t.x * srec.attenuation.x,
                t.y * srec.attenuation.y,
                t.z * srec.attenuation.z,
            ) * rec_.mat_ptr.scattering_pdf(r, &rec_, &mut scattered)
                / pdf_value
    } else {
        // let unit_direction = Vec3::unit(r.dir);
        // let t = 0.5 * (unit_direction.y + 1.0);
        // Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        //println!("3\n");
        *background
    }
}
#[allow(clippy::many_single_char_names)]
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
    const SAMPLES_PER_PIXEL: i32 = 2000;

    let mut lookfrom = Vec3::new(12.0, 2.0, 3.0);
    //let lookfrom = Vec3::new(15.0, 0.0, 12.0);

    let mut lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let mut aperture = 0.0;
    let mut world: HittableList = HittableList::new();
    let mut vfov = 20.0;
    let mut background = Vec3::zero();

    //let light = Arc::new(DiffuseLight::new1(Vec3::new(7.0, 7.0, 7.0)));
    // world.add(Arc::new(XzRect::new(
    //     123.0, 423.0, 147.0, 412.0, 554.0, light,
    // )));
    //let lights:Arc<dyn Hittable> = Arc::new(XzRect::new(213.0,343.0,227.0,332.0,554.0,Arc::new(Metal::new(Vec3::zero(),0.0))));
    let mut lights = HittableList::new();
    lights.add(Arc::new(XzRect::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::new(Metal::new(Vec3::zero(), 0.0)),
    )));
    // lights.add(Arc::new(XzRect::new(
    //             213.0,
    //             343.0,
    //             227.0,
    //             332.0,
    //             554.0,
    //             Arc::new(Metal::new(Vec3::zero(), 0.0)),
    //         )));
    //lights.add(Arc::new(Sphere::new(Vec3::new(190.0,90.0,190.0),90.0,Arc::new(Metal::new(Vec3::zero(),0.0)))));
    //let light = Arc::new(DiffuseLight::new1(Vec3::new(7.0, 7.0, 7.0)));
    // lights.add(Arc::new(XzRect::new(
    //     123.0, 423.0, 147.0, 412.0, 554.0, light,
    // )));
    let light_ = Arc::new(DiffuseLight::new1(Vec3::new(7.0, 7.0, 7.0)));
    lights.add(Arc::new(Sphere::new(
        Vec3::new(3.0, 3.0, 3.0),
        3.0,
        light_.clone(),
    )));
    // lights.add(Arc::new(XzRect::new(
    //     123.0, 423.0, 147.0, 412.0, 554.0, light_,
    // )));
    //lights.add(Arc::new(Sphere::new(Vec3::zero(),1.0,light_.clone())));
    //lights.add(Arc::new(XzRect::new(213.0,343.0,227.0,332.0,554.0,Arc::new(Metal::new(Vec3::zero(),0.0)))));
    //  lights.add(Arc::new(Sphere::new(Vec3::new(190.0,90.0,190.0),90.0,Arc::new(Metal::new(Vec3::zero(),0.0)))));
    let x = 8;
    if x == 0 {
        random_scene(&mut world);
        background = Vec3::new(0.7, 0.8, 1.0);
    } else if x == 1 {
        let checker = Arc::new(CheckerTexture::new(
            Vec3::new(0.2, 0.3, 0.1),
            Vec3::new(0.9, 0.9, 0.9),
        ));
        aperture = 0.0;
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            Arc::new(Lambertian::new1(checker.clone())),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            Arc::new(Lambertian::new1(checker)),
        )));
        background = Vec3::new(0.7, 0.8, 1.0);
    } else if x == 2 {
        let pertext = Arc::new(NoiseTexture::new1(4.0));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new1(pertext.clone())),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::new(Lambertian::new1(pertext)),
        )));
        lookfrom.x = 13.0;
        background = Vec3::new(0.7, 0.8, 1.0);
    } else if x == 3 {
        //lookfrom.x = 13.0;
        aperture = 0.0;
        lookfrom = Vec3::new(20.0, 20.0, 20.0);
        let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
        //let earth_texture = Arc::new(image_texture::new("tjm.jpg"));
        let erath_surface = Arc::new(Lambertian::new1(earth_texture));
        let golbe = Arc::new(Sphere::new(Vec3::zero(), 2.0, erath_surface));
        world.add(golbe);
        background = Vec3::new(0.7, 0.8, 1.0);
    } else if x == 4 {
        background = Vec3::zero();
        lookfrom.x = 26.0;
        lookfrom.y = 3.0;
        lookfrom.z = 6.0;
        lookat.y = 2.0;
        let pertext = Arc::new(NoiseTexture::new1(4.0));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new1(pertext.clone())),
        )));
        world.add(Arc::new(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            Arc::new(Lambertian::new1(pertext)),
        )));

        let difflight = Arc::new(DiffuseLight::new1(Vec3::new(4.0, 4.0, 4.0)));
        world.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    } else if x == 5 {
        let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new1(Vec3::new(15.0, 15.0, 15.0)));

        world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
        world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
        world.add(Arc::new(FlipFace::new(Arc::new(XzRect::new(
            123.0, 443.0, 127.0, 442.0, 554.0, light,
        )))));
        world.add(Arc::new(XzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        world.add(Arc::new(XzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        world.add(Arc::new(XyRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));

        // world.add(Arc::new(Box_::new(
        //     Vec3::new(130.0, 0.0, 655.0),
        //     Vec3::new(295.0, 165.0, 230.0),
        //     red.clone(),
        // )));
        // world.add(Arc::new(Box_::new(
        //     Vec3::new(265.0, 0.0, 295.0),
        //     Vec3::new(430.0, 330.0, 460.0),
        //     red.clone(),
        // )));
        let aluminum = Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));
        let box1 = Arc::new(Box_::new(
            Vec3::zero(),
            Vec3::new(165.0, 330.0, 165.0),
            aluminum,
        ));
        let box1_ = Arc::new(RotateY::new(box1, 15.0));
        let box1__ = Arc::new(Translate::new(box1_, Vec3::new(265.0, 0.0, 295.0)));
        //world.add(Arc::new(ConstantMedium::new1(box1__,0.01,Vec3::zero())));
        world.add(box1__);

        // let glass = Arc::new(Dielectric::new(1.5));
        // world.add(Arc::new(Sphere::new(Vec3::new(190.0,90.0,190.0),90.0,glass)));
        let box2 = Arc::new(Box_::new(
            Vec3::zero(),
            Vec3::new(165.0, 165.0, 165.0),
            white,
        ));
        let box2_ = Arc::new(RotateY::new(box2, -18.0));
        let box2__ = Arc::new(Translate::new(box2_, Vec3::new(130.0, 0.0, 65.0)));
        world.add(box2__);
        //world.add(Arc::new(ConstantMedium::new1(box2__,0.01,Vec3::ones())));

        background = Vec3::zero();
        lookfrom.x = 278.0;
        lookfrom.y = 278.0;
        lookfrom.z = -800.0;

        lookat.x = 278.0;
        lookat.y = 278.0;
        lookat.z = 0.0;
        vfov = 40.0;
        aperture = 0.0
    } else if x == 6 {
        final_scene(&mut world);
        aperture = 0.0;
        lookfrom = Vec3::new(478.0, 278.0, -600.0);
        lookat = Vec3::new(278.0, 278.0, 0.0);
        vfov = 40.0;
        background = Vec3::zero();
    } else if x == 7 {
        background = Vec3::zero();
        lookfrom.x = 278.0;
        lookfrom.y = 278.0;
        lookfrom.z = -800.0;

        lookat.x = 278.0;
        lookat.y = 278.0;
        lookat.z = 0.0;
        vfov = 40.0;
        aperture = 0.0;

        let red = Arc::new(Lambertian::new(Vec3::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::new(Vec3::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::new(Vec3::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new1(Vec3::new(15.0, 15.0, 15.0)));

        world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
        world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
        world.add(Arc::new(FlipFace::new(Arc::new(XzRect::new(
            123.0, 443.0, 127.0, 442.0, 554.0, light,
        )))));
        world.add(Arc::new(XzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        world.add(Arc::new(XzRect::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        world.add(Arc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

        let cornell_box = tobj::load_obj(
            "bunny_1k.obj",
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(cornell_box.is_ok());
        let rate = 500.0;
        let (models, _materials) = cornell_box.expect("Failed to load OBJ file");
        for (_i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            let mut boxes2 = HittableList { objects: vec![] };
            for v in 0..mesh.indices.len() / 3 {
                let x1 = mesh.indices[3 * v];
                let x2 = mesh.indices[3 * v + 1];
                let x3 = mesh.indices[3 * v + 2];
                let triange = Triangle::new(
                    rate * mesh.positions[(3 * x1) as usize] as f64,
                    rate * mesh.positions[(3 * x1 + 1) as usize] as f64,
                    rate * mesh.positions[(3 * x1 + 2) as usize] as f64,
                    rate * mesh.positions[(3 * x2) as usize] as f64,
                    rate * mesh.positions[(3 * x2 + 1) as usize] as f64,
                    rate * mesh.positions[(3 * x2 + 2) as usize] as f64,
                    rate * mesh.positions[(3 * x3) as usize] as f64,
                    rate * mesh.positions[(3 * x3 + 1) as usize] as f64,
                    rate * mesh.positions[(3 * x3 + 2) as usize] as f64,
                    Arc::new(Lambertian::new(Vec3::new(0.99, 0.83, 0.0))),
                );
                boxes2.add(Arc::new(triange));
            }
            //let allin = Arc::new(BVHNODE::new(&boxes2.objects, 0.0 as usize, boxes2.objects.len(), 0.0, 1.0))
            let allin = Translate::new(
                Arc::new(RotateY::new(
                    Arc::new(BVHNODE::new(
                        &boxes2.objects,
                        0.0 as usize,
                        boxes2.objects.len(),
                        0.0,
                        1.0,
                    )),
                    180.0,
                )),
                Vec3::new(290.0, -27.0, 190.0),
            );
            world.add(Arc::new(allin));
        }
    } else if x == 8 {
        aperture = 0.0;
        lookfrom = Vec3::new(-2.0, 0.0, -2.0);
        lookat = Vec3::new(-10.0, 0.0, -10.0);
        // world.add(Arc::new(Sphere::new(Vec3::new(100.0,100.0,0.0), 120.0,light_.clone())));
        // world.add(Arc::new(Sphere::new(Vec3::new(-100.0,100.0,0.0), 120.0,light_.clone())));
        // world.add(Arc::new(Sphere::new(Vec3::new(100.0,-100.0,0.0), 120.0,light_.clone())));
        // world.add(Arc::new(Sphere::new(Vec3::new(-100.0,-100.0,100.0), 120.0,light_.clone())));
        // let r = 40.0;
        // let sita = PI / 6.0;
        let venus_texture = Arc::new(ImageTexture::new("earth.jpg"));
        let venus_surface = Arc::new(Lambertian::new1(venus_texture));
        let venus = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, venus_surface));

        world.add(venus);

        let bo = Arc::new(Dielectric::new(1.5));
        //let bb = Arc::new(bo);
        let bbb = Box_::new(Vec3::new(-2.0, -2.0, -2.0), Vec3::new(2.0, 2.0, 2.0), bo);
        //let bbb1 = Box_::new(Vec3::new(-2.0, -2.0, -2.0), Vec3::new(2.0, 2.0, 2.0), bo);
        world.add(Arc::new(bbb));
        //world.add(Arc::new(RotateY::new(Arc::new(bbb1),45.0)));
        //world.add(sunnn);

        // let r = 80.0;
        // let sita = PI / 2.0;
        // let mercury_texture = Arc::new(ImageTexture::new("mercury.jpg"));
        // let mercury_surface = Arc::new(Lambertian::new1(mercury_texture));
        // let mercury = Arc::new(Sphere::new(Vec3::new(r * sita.cos(), r * sita.sin(), 0.0), 30.0, mercury_surface));
        // world.add(mercury);
        //let light_ = Arc::new(DiffuseLight::new1(Vec3::new(15.0, 15.0, 15.0)));
        let bb = Arc::new(Sphere::new(Vec3::new(10.0, 0.0, 10.0), 5.0, light_));
        world.add(bb);
        //world.add(Arc::new(RotateY::new(bb.clone(),45.0)));
        //world.add(Arc::new(Sphere::new(Vec3::new(10.0,-3.0,10.0),5.0,light_.clone())));

        //world.add(Arc::new(Triangle::new(10.0,3.0,10.0,10.0,-3.0,10.0,4.0,4.0,0.0,light_.clone())));

        //world.add(Arc::new())

        //world.add(Arc::new(Sphere::new(Vec3::new(0.0,0.0,10.0),6.0,light_.clone())));
        // world.add(Arc::new(Sphere::new(Vec3::new(0.0,0.0,10.0),6.0,light_.clone())));
        // world.add(Arc::new(Sphere::new(Vec3::new(0.0,0.0,-10.0),6.0,light_.clone())));

        let al = Arc::new(Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0));
        //let glass = Arc::new(Dielectric::new(1.5));
        //let x = Arc::new(Lambertian::new(Vec3::new(0.8,0.7,0.9)));
        //let s1 = Arc::new(YzRect::new(-1000.0,1000.0,-50.0,50.0,50.0 * (3.0 as f64).sqrt(),al.clone()));
        let box1 = Arc::new(Box_::new(
            Vec3::new(-10.0, -10.0, -10.0),
            Vec3::new(10.0, 10.0, 10.0),
            al,
        ));
        world.add(box1.clone());
        let box2 = RotateY::new(box1, 45.0);
        //let box3 = RotateY::new(box1.clone(),60.0);
        //world.add(Arc::new(box3));
        world.add(Arc::new(box2));
        background = Vec3::new(0.7, 0.8, 1.0);
    }
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

    let (tx, rx) = channel();
    let n_jobs = 32;
    let n_workers = 16;
    let pool = ThreadPool::new(n_workers);

    let mut results: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    let bar = ProgressBar::new(n_jobs as u64);

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ = world.clone();
        let light = lights.clone();
        let lightss = Arc::new(light.clone());
        //let lights_ptr = lights.clone();
        pool.execute(move || {
            let row_begin = IMAGE_HEIGHT as usize * i as usize / n_jobs;
            let row_end = IMAGE_HEIGHT as usize * (i as usize + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, render_height as u32);
            for x in 0..IMAGE_WIDTH {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..SAMPLES_PER_PIXEL {
                        let u = (x as f64 + random_double(0.0, 100.0)) / ((IMAGE_WIDTH - 1) as f64);
                        let v = ((IMAGE_HEIGHT as u32 - y) as f64 - random_double(0.0, 100.0))
                            / ((IMAGE_HEIGHT - 1) as f64);
                        let r = cam.get_ray(u, v);
                        pixel_color += color(
                            &r,
                            &background,
                            &world_,
                            //&Arc::new(light.clone()),
                            &lightss,
                            MAX_DEPTH,
                        );
                        //println!("{},{},{}\n",pixel_color.x,pixel_color.y,pixel_color.z);
                    }
                    let mut r = pixel_color.x;
                    let mut g = pixel_color.y;
                    let mut b = pixel_color.z;
                    //println!("{},{},{}\n",r,b,g);
                    let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);
                    r = (r * scale).sqrt();
                    g = (g * scale).sqrt();
                    b = (b * scale).sqrt();
                    //println!("{},{},{}\n",r,b,g);
                    // if r != r {
                    //     r = 0.0;
                    // }
                    // if g != g {
                    //     g = 0.0;
                    // }
                    // if b != b {
                    //     b = 0.0;
                    // }

                    let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
                    let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
                    let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
                    let pixel = img.get_pixel_mut(x as u32, img_y as u32);
                    *pixel = image::Rgb([ir, ig, ib]);
                    // let r = (r / SAMPLES_PER_PIXEL as f64).sqrt();
                    // let g = (g / SAMPLES_PER_PIXEL as f64).sqrt();
                    // let b = (b / SAMPLES_PER_PIXEL as f64).sqrt();
                    // let pixel = img.get_pixel_mut(x as u32, img_y as u32);
                    // *pixel = image::Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..IMAGE_WIDTH {
                let row = row as u32;
                let idx = idx as u32;
                *results.get_pixel_mut(col as u32, row) = *data.get_pixel(col as u32, idx);
            }
        }
        bar.inc(1);
    }

    results.save("output/test.png").unwrap();
    bar.finish();

    // let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    // let bar = ProgressBar::new(IMAGE_WIDTH as u64);

    // for j in 0..IMAGE_WIDTH {
    //     for i in 0..IMAGE_HEIGHT {
    //         let pixel = img.get_pixel_mut(j as u32, i as u32);
    //         let mut pixel_color = Vec3::zero();
    //         for _s in 0..SAMPLES_PER_PIXEL {
    //             let u = (((j as f64 + random_double(0.0, 100.0)) as f64)
    //                 / (IMAGE_WIDTH as f64 - 1.0)) as f64;
    //             let v = (((IMAGE_HEIGHT - i) as f64 - random_double(0.0, 100.0) as f64)
    //                 / (IMAGE_HEIGHT as f64 - 1.0)) as f64;
    //             let r = cam.get_ray(u, v);
    //             pixel_color += color(&r, &background,&world, MAX_DEPTH);
    //         }
    //         let mut r = pixel_color.x;
    //         let mut g = pixel_color.y;
    //         let mut b = pixel_color.z;

    //         let scale = 1.0 / (SAMPLES_PER_PIXEL as f64);
    //         r = (r * scale).sqrt();
    //         g = (g * scale).sqrt();
    //         b = (b * scale).sqrt();
    //         //println!("{},{},{}\n",r,b,g);
    //         let ir = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    //         let ig = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    //         let ib = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    //         *pixel = image::Rgb([ir, ig, ib]);
    //     }
    //     bar.inc(1);
    // }

    // img.save("output/test.png").unwrap();
    // bar.finish();
}
