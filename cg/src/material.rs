use crate::Ray;
use crate::Vector3;
use crate::HitRecord;
use std::fmt;

pub trait Material {
    fn scatter(r_in: &Ray, rec: &mut HitRecord, attenuation: Vector3, scattered: &Ray) -> bool;
}

impl<'a> fmt::Debug for dyn Material + 'a {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Abstract Material")
    }
}


#[derive(Debug)]
pub struct Object<'a> {
    mat: &'a dyn Material,
}
