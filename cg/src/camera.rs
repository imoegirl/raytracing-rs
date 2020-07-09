use crate::{ Vector3, Ray };

extern crate math;

pub struct Camera {
    pub lookfrom: Vector3,
    pub lookat: Vector3,
    pub vup: Vector3,
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width : f64,

    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    // pub fn new( aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
    //     let viewport_width = aspect_ratio * viewport_height;
    //     let origin = Vector3::zero();
    //     let horizontal= Vector3::new(viewport_width, 0.0, 0.0);
    //     let vertical = Vector3::new(0.0, viewport_height, 0.0);
    //     let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    //     Camera {
    //         aspect_ratio,
    //         viewport_height,
    //         viewport_width,
    //         focal_length,
    //         origin,
    //         horizontal,
    //         vertical,
    //         lower_left_corner,
    //     }
    // }

    pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = math::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalized();
        let u = Vector3::cross(vup, w);
        let v = Vector3::cross(w, u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        // let focal_length = 1.0;
        // let origin = Vector3::zero();
        // let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        // let vertical = Vector3::new(0.0, viewport_height, 0.0);
        // let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

        Camera {
            lookfrom,
            lookat,
            vup,
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
        )
    }
}
