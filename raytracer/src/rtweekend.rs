#[allow(clippy::float_cmp)]
use rand::{Rng, random};
pub fn random_double(min:f64,max:f64) -> f64{
    let secret_number = rand::thread_rng().gen_range(min..max);
    secret_number / (max - min)
}

pub fn random_double2(min:f64,max:f64) -> f64{
    let secret_number = rand::thread_rng().gen_range(min..max);
    secret_number
}