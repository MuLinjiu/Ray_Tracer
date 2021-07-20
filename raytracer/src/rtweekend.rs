use rand::{random, Rng};
pub fn random_double(min: f64, max: f64) -> f64 {
    let secret_number = rand::thread_rng().gen_range(min..max);
    secret_number / (max - min)
}

pub fn random_double2(min: f64, max: f64) -> f64 {
    let secret_number = rand::thread_rng().gen_range(min..max);
    secret_number
}

pub fn random_int(min: i32, max: i32) -> i32 {
    let secret_number = rand::thread_rng().gen_range(min..max);
    secret_number
}
