use crate::Hitable;
use crate::HitRecord;
use crate::Ray;
use crate::Material;

extern crate math;
use math::Vector3;

pub struct Sphere {
    pub hit_rec: HitRecord,
    pub center: Vector3,
    pub radius: f64,
    pub mat: Material,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, mat: Material) -> Self {
        Sphere { hit_rec: HitRecord::default(), center, radius, mat}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vector3::dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;

            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);

                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = self.mat;
                return true;
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = self.mat;
                return true;
            }
        }

        return false;
    }
}