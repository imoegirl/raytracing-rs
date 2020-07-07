// use std::io::prelude::*;
// use std::fs::File;

extern crate math;
use math::Vector3;

extern crate cg;
use cg::{ Ray, Hitable, HitRecord, HittableList, Sphere, Camera };

// extern crate pbr;
// use pbr::ProgressBar;

extern crate image;

// use std::thread;
use std::time::Instant;


fn main(){
    let now = Instant::now();
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 300;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: f64 = 100.0;
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let max_depth: u32 = 50;
    
    // create rendering things
    let mut hittable_list = HittableList::new();
    let sphere1 = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0);
    hittable_list.add(Box::new(sphere1));
    hittable_list.add(Box::new(sphere2));

    // let mut pb = ProgressBar::new((image_width * image_height) as u64);
    // pb.format("╢▌▌░╟");

    let camera = Camera::new(aspect_ratio,viewport_height, focal_length);

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // j is row index, and i is col index
    // for(i, j, pixel) in imgbuf.enumerate_pixels_mut() {
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = imgbuf.get_pixel_mut(i, j);
            // let c = 0 as u8;
            // *pixel = image::Rgb([c, c, c]);
            let x = i;
            let y = (image_height - 1) - j;

            let mut pixel_color = Vector3::zero();
            for _s in 0..samples_per_pixel as usize {
                let u = (x as f64 + math::random_double()) / (image_width -1) as f64;
                let v = (y as f64 + math::random_double()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                let ray_color = ray_color(&r, &hittable_list, max_depth);
                pixel_color += ray_color;
            }
            
            let color = get_samples_color(pixel_color, samples_per_pixel);

            let r = color.x as u8;
            let g = color.y as u8;
            let b = color.z as u8;

            *pixel = image::Rgb([r, g, b]);

            // pb.inc();
        }
    }

    imgbuf.save("generated.png").unwrap();
  
    let elapsed_sec = now.elapsed().as_secs();
    println!("Elapsed: {}", elapsed_sec);
}


fn get_samples_color(ray_color: Vector3, samples_per_pixel: f64) -> Vector3 {
    let mut r = ray_color.x;
    let mut g = ray_color.y;
    let mut b = ray_color.z;

    let scale = 1.0 / samples_per_pixel;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    let max_value = 256.0;
    let color_min = 0.0;
    let color_max = 0.999;

    r = max_value * math::clamp(r, color_min, color_max);
    g = max_value * math::clamp(g, color_min, color_max);
    b = max_value * math::clamp(b, color_min, color_max);

    Vector3::new(r, g, b)
}


fn ray_color(r: &Ray, world: &dyn Hitable, depth: u32) -> Vector3 {
    if depth <= 0 {
        return Vector3::zero();
    }

    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, std::f64::INFINITY, &mut rec) {
        // return (rec.normal + Vector3::one()) * 0.5
        // let target = rec.p + rec.normal + Vector3::random_in_unit_sphere();
        // let target = rec.p + rec.normal + Vector3::random_unit_vector(); 
        let target = rec.p + Vector3::random_in_hemisphere(rec.normal);
        let ray = Ray::new(rec.p, target - rec.p);
        return ray_color(&ray, world, depth - 1) * 0.5;
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);

    return Vector3::one() * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;

}

// fn ray_color(r: &Ray) -> Vector3 {
//     let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, r);

//     if t > 0.0 {
//         let vn = (r.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalized();
//         Vector3::new(vn.x + 1.0, vn.y + 1.0, vn.z + 1.0) * 0.5
//     }else{
//         let unit_direction = r.direction.normalized();
//         let t = 0.5 * (unit_direction.y + 1.0);
//         Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
//     }
// }

#[allow(dead_code)]
fn hit_sphere(center: Vector3, radius: f64, r: &Ray) -> f64 {
    // let oc = r.origin - center;
    // let a = Vector3::dot(r.direction, r.direction);
    // let b = Vector3::dot(oc, r.direction) * 2.0;
    // let c = Vector3::dot(oc, oc) - radius * radius;
    // let discriminant = b * b - 4.0 * a * c;
    // if discriminant < 0.0 {
    //      -1.0
    // } else {
    //     (-b - discriminant.sqrt()) / (2.0 * a)
    // }

    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = Vector3::dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
         -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }

}