extern crate math;
use math::Vector3;
use crate::Ray;

#[derive(Debug)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            p: Vector3::zero(),
            normal: Vector3::zero(),
            t: 0.0,
            front_face: false
        }
    }

    pub fn new(p: Vector3, normal: Vector3, t: f64, front_face: bool) -> Self {
        HitRecord { p, normal, t, front_face }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = Vector3::dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }

    pub fn copy_from(&mut self, from_record: &HitRecord) {
        self.p = from_record.p;
        self.normal = from_record.normal;
        self.t = from_record.t;
        self.front_face = from_record.front_face;
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p &&
        self.normal == other.normal &&
        self.t == other.t &&
        self.front_face == other.front_face
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}