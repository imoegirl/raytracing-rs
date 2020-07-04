// use std::io::prelude::*;
// use std::fs::File;

extern crate math;
use math::Vector3;

extern crate cg;
use cg::Ray;

// extern crate pbr;
// use pbr::ProgressBar;

extern crate image;


fn main() -> std::io::Result<()>{

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 1920;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    
    let origin = Vector3::zero();
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    dbg!(&lower_left_corner);

    // let mut pb = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64);
    // pb.format("╢▌▌░╟");

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // j is row index, and i is col index
    for(i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        // let r: u8 = ((x as f64) / ((image_width - 1) as f64) * 255.999) as u8;
        // let g: u8 = ((y as f64) / ((image_height - 1) as f64) * 255.999) as u8;
        // let b: u8 = (0.25 * 255.999) as u8;
        // *pixel = image::Rgb([r, g, b]);
        // pb.inc();

        // revert
        let j = (image_height - 1) - j;

        let u = i as f64 / (image_width -1) as f64;
        let v = j as f64 / (image_height - 1) as f64;
        let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
        let color = ray_color(&r) * 255.999;
        let r = color.x as u8;
        let g = color.y as u8;
        let b = color.z as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("generated.png").unwrap();

  
    Ok(())
}

fn ray_color(r: &Ray) -> Vector3 {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let vn = (r.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalized();
        Vector3::new(vn.x + 1.0, vn.y + 1.0, vn.z + 1.0) * 0.5
    }else{
        let unit_direction = r.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}

fn hit_sphere(center: Vector3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = Vector3::dot(r.direction, r.direction);
    let b = Vector3::dot(oc, r.direction) * 2.0;
    let c = Vector3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
         -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}