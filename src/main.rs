extern crate image;

use std::collections::HashSet;

mod mandelbrot_generator;
mod multi_dim_array;

use mandelbrot_generator::simple_generator::{get_mandelbrot_grid_iteration_values, mandelbrot_iteration};

fn main() {
    //mandelbrot_iteration(0.0_f64, 0.0_f64, -1.0_f64, 0.0_f64, 0, 20);
    let height: usize = 1080;
    let width: usize = 1080;
    let iterations = mandelbrot_iteration(0.0, 0.0, 1.0, 0.0, 0, 1000);
    println!("{}", iterations);
    let pic = get_mandelbrot_grid_iteration_values(-3.0_f64, 2.0_f64, -2.0_f64, 2.0_f64, height, width);
    let mut img: image::GrayImage = image::ImageBuffer::new(height as u32, width as u32);
    let mut set: HashSet<i64> = HashSet::new();
    let mut pixel_set: HashSet<u8> = HashSet::new();
    for x in 0..pic.height {
        for y in 0..pic.width {
            let pic_val = pic.get(x, y).map_or(-1.0 ,|val| val);
            set.insert(pic_val as i64);
            let mut pixel_val = (255.0_f64 * (pic_val / 1000.0_f64 )) as u8;
            pixel_set.insert(pixel_val);
            img.put_pixel(x as u32, y as u32, image::Luma([pixel_val]));
        }
    }
    println!("{:?}", set);
    println!("{:?}", pixel_set);
    img.save("fractal.png").unwrap();
}
