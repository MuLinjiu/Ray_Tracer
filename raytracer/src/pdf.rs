use std::{f64::consts::PI, sync::Arc};

//use crate::{Vec3, hittable::Hittable, materia::random_cosine_direction, onb::Onb, rtweekend::random_double};
use crate::{
    hittable::Hittable, materia::random_cosine_direction, onb::Onb, rtweekend::random_double, Vec3,
};
pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = Vec3::dot(direction.unit(), self.uvw.w);

        if cosine <= 0.0 {
            return 0.0;
        } else {
            return cosine / PI;
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local1(&random_cosine_direction())
    }
}

pub struct HittablePdf {
    pub o: Vec3,
    pub ptr: Arc<dyn Hittable>,
}

impl HittablePdf {
    pub fn new(p: Arc<dyn Hittable>, origin: Vec3) -> Self {
        Self { ptr: p, o: origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        return self.ptr.pdf_value(&self.o, direction);
    }

    fn generate(&self) -> Vec3 {
        return self.ptr.random(self.o);
    }
}
pub struct MixturePdf {
    p0: Arc<dyn Pdf>,
    p1: Arc<dyn Pdf>,
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self { p0, p1 }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        // if 0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction) == 0.0{
        //     println!()
        // }
        return 0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction);
    }

    fn generate(&self) -> Vec3 {
        if random_double(0.0, 100.0) < 0.5 {
            return self.p0.generate();
        } else {
            self.p1.generate()
        }
    }
}
pub struct NonePdf {
    pub val: f64,
}

impl NonePdf {
    pub fn new() -> Self {
        Self { val: 0.0 }
    }
}

impl Pdf for NonePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        return 0.0;
    }
    fn generate(&self) -> Vec3 {
        return Vec3::zero();
    }
}
