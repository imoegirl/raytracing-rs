use crate::Ray;
use crate::Vector3;
use crate::HitRecord;
// use std::fmt;
extern crate math;

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - n * 2.0 * Vector3::dot(v, n)
}

fn refrect(uv: Vector3, n: Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = Vector3::dot(-uv, n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = (1.0 - r_out_parallel.length_squared()) * n;
    r_out_parallel + r_out_perp
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0-cosine).powi(5)
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool;
}


#[derive(Clone, Copy, Debug)]
pub enum Material {
    None,
    Lambertain(Lambertain),
    Metal(Metal),
    Dielectric(Dielectric),
}


impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool{
        match *self {
            Material::None => false,
            Material::Lambertain(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
            Material::Metal(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
            Material::Dielectric(ref inner) => inner.scatter(ray_in, rec, attenuation, scattered),
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Lambertain {
    albedo: Vector3,
}

impl Lambertain {
    pub fn new(albedo: Vector3) -> Material {
        let lambertain = Lambertain { albedo };
        Material::Lambertain(lambertain)
    }
}

impl Scatterable for Lambertain {
    fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vector3::random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Material {
        let metal = Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        };

        Material::Metal(metal)
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        let refelcted = reflect(ray_in.direction.normalized(), rec.normal);
        *scattered = Ray::new(rec.p, refelcted + self.fuzz * Vector3::random_in_unit_sphere());
        *attenuation = self.albedo;
        true
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    ref_idx: f64
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Material {
        let dielectric = Self {
            ref_idx
        };
        Material::Dielectric(dielectric)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Vector3, scattered: &mut Ray) -> bool {
        *attenuation = Vector3::one();
        let mut etai_over_etat: f64 = 0.0;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        }else{
            etai_over_etat = self.ref_idx;
        }

        let unit_direction = ray_in.direction.normalized();

        let cos_theta = Vector3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if math::random_double() < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            *scattered = Ray::new(rec.p, reflected);
            return true;
        }

        let refracted = refrect(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}


// pub trait Material {
//     fn scatter(r_in: &Ray, rec: &mut HitRecord, attenuation: Vector3, scattered: &Ray) -> bool;
// }

// impl<'a> fmt::Debug for dyn Material + 'a {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Abstract Material")
//     }
// }


// #[derive(Debug)]
// pub struct Object<'a> {
//     mat: &'a dyn Material,
// }
