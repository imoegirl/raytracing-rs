mod ray;
pub use ray::Ray;

mod hittable;
pub use hittable::HitRecord;
pub use hittable::Hitable;

mod hittable_list;
pub use hittable_list::HittableList;

mod sphere;
pub use sphere::Sphere;

mod camera;
pub use camera::Camera;

mod material;
pub use material::Material;
pub use material::Lambertain;
pub use material::Metal;
pub use material::Dielectric;
pub use material::Scatterable;


extern crate math;
pub use math::Vector3;






// extern crate math;
// use math::Vector3;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let rec = Box::new(HitRecord::new(Vector3::zero(), Vector3::one(), 1.2, false));
//         let rec2 = HitRecord::new(Vector3::zero(), Vector3::one(), 1.2, false);
//         assert_eq!( rec.t , rec2.t );
//     }
// }
