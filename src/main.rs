// use std::io::prelude::*;
// use std::fs::File;

extern crate math;
use math::Vector3;

extern crate cg;
use cg::{ Ray, Hitable, HitRecord, HittableList, Sphere, Camera, Lambertain, Metal, Dielectric, Scatterable };

extern crate pbr;
use pbr::ProgressBar;

extern crate image;

// use std::thread;
use std::time::Instant;


fn main(){
    let now = Instant::now();
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 1920;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: f64 = 100.0;
    let max_depth: u32 = 50;
    
    let mut pb = ProgressBar::new((image_width * image_height) as u64);
    pb.format("╢▌▌░╟");

    let world = random_scene();

    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    // j is row index, and i is col index
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = imgbuf.get_pixel_mut(i, j);

            let x = i;
            let y = (image_height - 1) - j;

            let mut pixel_color = Vector3::zero();
            for _s in 0..samples_per_pixel as usize {
                let u = (x as f64 + math::random_double()) / (image_width -1) as f64;
                let v = (y as f64 + math::random_double()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                let ray_color = ray_color(&r, &world, max_depth);
                pixel_color += ray_color;
            }
            
            let color = get_samples_color(pixel_color, samples_per_pixel);

            let r = color.x as u8;
            let g = color.y as u8;
            let b = color.z as u8;

            *pixel = image::Rgb([r, g, b]);

            pb.inc();
        }
    }

    imgbuf.save("generated.png").unwrap();
  
    let elapsed_sec = now.elapsed().as_secs();
    println!("Elapsed: {}", elapsed_sec);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Lambertain::new(Vector3::new(0.5, 0.5, 0.5));
    world.add(
        Box::new(Sphere::new(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        ))
    );

    for a in -11..11 {
        for b in -11.. 11 {
            let choose_mat = math::random_double();
            let center = Vector3::new(
                a as f64 + 0.9 * math::random_double(),
                0.2,
                b as f64 + 0.9 * math::random_double(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat = if choose_mat < 0.8 {
                    let albedo = Vector3::random() * Vector3::random();
                    Lambertain::new(albedo)
                }
                else if choose_mat < 0.95 {
                    let albedo = Vector3::random_range(0.5, 1.0);
                    let fuzz = math::random_double_of_range(0.0, 0.5);
                    Metal::new(albedo, fuzz)
                }
                else {
                    Dielectric::new(1.5)
                };

                let sphere = Sphere::new(center, 0.2, mat);
                world.add(Box::new(sphere));
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let sphere1 = Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let mat2 = Lambertain::new(Vector3::new(0.4, 0.2, 0.1));
    let sphere2 = Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let mat3 = Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0);
    let sphere3 = Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));

    world
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
        let mut scattered: Ray = Ray::zero();
        let mut attenuation: Vector3 = Vector3::zero();
        let mat = rec.mat;
        if mat.scatter(r, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Vector3::zero();
    }

    let unit_direction = r.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);

    return Vector3::one() * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;

}

#[allow(dead_code)]
fn hit_sphere(center: Vector3, radius: f64, r: &Ray) -> f64 {
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