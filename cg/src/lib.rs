mod ray;
pub use ray::Ray;

mod hittable;
pub use hittable::HitRecord;
pub use hittable::Hitable;

mod hittable_list;
pub use hittable_list::HittableList;

mod sphere;
pub use sphere::Sphere;


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
