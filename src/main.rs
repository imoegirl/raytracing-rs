// use std::io::prelude::*;
// use std::fs::File;

// extern crate math;
// use math::vector::Vector3;

// extern crate pbr;
// use pbr::ProgressBar;

extern crate image;


fn main() -> std::io::Result<()>{
    const IMAGE_WIDTH: u32 = 51200;
    const IMAGE_HEIGHT: u32 = 51200;

    // let mut pb = ProgressBar::new((IMAGE_HEIGHT * IMAGE_WIDTH) as u64);
    // pb.format("╢▌▌░╟");

    let mut imgbuf = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for(x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r: u8 = ((x as f64) / ((IMAGE_WIDTH - 1) as f64) * 255.999) as u8;
        let g: u8 = ((y as f64) / ((IMAGE_HEIGHT - 1) as f64) * 255.999) as u8;
        let b: u8 = (0.25 * 255.999) as u8;
        *pixel = image::Rgb([r, g, b]);
        // pb.inc();
    }

    imgbuf.save("generated.png").unwrap();
  
    Ok(())
}