use rand::Rng;
pub fn random_double(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max) / (max - min)
}

pub fn random_double2(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..max)
}
