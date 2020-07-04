mod ray;
pub use ray::Ray;

// use ray::Ray;

// extern crate math;
// use math::vector::Vector3;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let origin = Vector3::zero();
//         let direction = Vector3::new(1.0, 1.0, 0.0);
//         let ray = Ray::new(origin, direction);

//         assert_eq!(ray.origin, Vector3::new(0.0, 0.0, 0.0));
//         assert_eq!(ray.direction, Vector3::new(1.0, 1.0, 0.0));
//         assert_eq!(ray.at(5.0), Vector3::new(5.0, 5.0, 0.0));
//         assert_eq!(ray.direction, Vector3::new(1.0, 1.0, 0.0));
//     }
// }
