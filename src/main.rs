use mandelbrot_set::{get_mandelbrot_grid_iteration_values, mandelbrot_iteration};

fn main() {
    mandelbrot_iteration(0.0_f64, 0.0_f64, -1.0_f64, 0.0_f64, 0, 20);
    let height: usize = 5;
    let width: usize = 5;
    // let pixel_img = image::twodarray::TwoDArray::new(height, width, -1_i32);
    // println!("{:?}", pixel_img.data);
    get_mandelbrot_grid_iteration_values(0.0_f64, 4.0_f64, 0.0_f64, 4.0_f64, height, width);
}
