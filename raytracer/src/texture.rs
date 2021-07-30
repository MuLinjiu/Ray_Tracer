use std::{path::Path, sync::Arc};

use crate::clamp;

use image::GenericImageView;

use crate::{perlin::Perlin, Vec3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}
pub struct SolidColor {
    color_value: Vec3,
}

impl SolidColor {
    pub fn new(c: Vec3) -> Self {
        Self { color_value: c }
    }
    // fn new1(red: f64, green: f64, blue: f64) -> Self {
    //     Self {
    //         color_value: Vec3::new(red, green, blue),
    //     }
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value
    }
}

pub struct BaseColor {
    color: Vec3,
}

impl BaseColor {
    pub fn vectobase(color: Vec3) -> Self {
        Self { color }
    }
}

impl Texture for BaseColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(_even: Vec3, _odd: Vec3) -> Self {
        Self {
            even: Arc::new(BaseColor::vectobase(_even)),
            odd: Arc::new(BaseColor::vectobase(_odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scare: f64,
}

impl NoiseTexture {
    // pub fn new() -> Self {
    //     let n = Perlin::new();
    //     Self {
    //         noise: n,
    //         scare: 0.0,
    //     }
    // }
    pub fn new1(sc: f64) -> Self {
        let n = Perlin::new();
        Self {
            noise: n,
            scare: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (10.0 * self.noise.turb(*p, 7) + self.scare * p.z).sin())
    }
}

const BYTES_PER_PIXEL: i32 = 3;

pub struct ImageTexture {
    pub width: i32,
    pub height: i32,
    pub bytes_per_scanline: i32,
    pub data: image::DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let _components_per_pixel = BYTES_PER_PIXEL;
        let im = image::open(&Path::new(filename)).unwrap();
        Self {
            width: im.dimensions().0 as i32,
            height: im.dimensions().1 as i32,
            bytes_per_scanline: 0,
            data: im,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;
        if i >= self.width as u32 {
            i = (self.width - 1) as u32;
        }
        if j >= self.height as u32 {
            j = (self.height - 1) as u32;
        }
        let cs = 1.0 / 255.0;
        let pixel = self.data.get_pixel(i as u32, j as u32);
        Vec3::new(
            pixel[0] as f64 * cs,
            pixel[1] as f64 * cs,
            pixel[2] as f64 * cs,
        )
    }
}
