extern crate math;
use math::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray{
            origin,
            direction,
        }
    }

    pub fn zero() -> Self {
        Ray {
            origin: Vector3::zero(),
            direction: Vector3::zero(),
        }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}