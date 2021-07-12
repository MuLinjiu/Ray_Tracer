#[allow(clippy::float_cmp)]
mod vec3;
mod ray;
//use std::intrinsics::ceilf32;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub use vec3::Vec3;
pub use ray::Ray;
fn color(r:Ray,) ->Vec3{
    Vec3 unit_direction = unit(r.dir);
}
fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    let mut img: RgbImage = ImageBuffer::new(256, 256);
    let bar = ProgressBar::new(256);

    for x in (0..256).rev(){
        for y in (0..256).rev() {
            let pixel = img.get_pixel_mut(x, y);
            let x1 = x as f32;
            let y1 = (y) as f32;
            let r = (x1 / 256.) as f64;
			let g = (y1 / 256.) as f64;
			let b = 0.25;
			let ir = (255.999 * r) as u8;
			let ig = (255.999 * g) as u8;
			let ib = (255.999 * b) as u8;
            *pixel = image::Rgb([ir, ig, ib]);
        }
        bar.inc(1);
    }



    img.save("output/test.png").unwrap();
    bar.finish();
}
