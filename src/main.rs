use std::io::prelude::*;
use std::fs::File;

extern crate math;
use math::vector::Vector3;

extern crate pbr;
use pbr::ProgressBar;


fn main() -> std::io::Result<()>{
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut buffer = File::create("image.ppm")?;
    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    buffer.write_all(header.as_str().as_bytes())?;

    let mut pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    pb.format("╢▌▌░╟");

    let mut j: i32 = IMAGE_HEIGHT - 1;
    while j >= 0 {
        let mut i = 0;
        while i < IMAGE_WIDTH {
            let r: f64 = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g: f64 = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b: f64 = 0.25;

            let color = Vector3::new(r,g,b);
            write_color(&mut buffer, &color)?;

            i += 1;
        }
        pb.inc();
        j -= 1;
    }

    // pb.finish_print("Done");
  
    Ok(())
}

fn write_color(buffer: &mut File, v: &Vector3) -> std::io::Result<()> {
    buffer.write_all(format!("{} {} {}\n", 
    v.x * 255.999,
    v.y * 255.999,
    v.z * 255.999).as_str().as_bytes())?;
    Ok(())
}