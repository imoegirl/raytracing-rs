mod vector;
pub use vector::Vector3;

extern crate rand;
use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = thread_rng();
    let x: f64 = rng.gen();
    x
}

pub fn random_double_of_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    let x: f64 = rng.gen_range(min, max);
    x
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    }else if x > max {
        max
    }else{
        x
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
        
        
//     }
// }
