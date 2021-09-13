mod image;

fn main() {
    mandelbrot_iteration(0.0_f64, 0.0_f64, -1.0_f64, 0.0_f64, 0, 20);
    println!("Hello, world!");
    let mut height: usize = 5;
    let mut width: usize = 5;
    // let pixel_img = image::twodarray::TwoDArray::new(height, width, -1_i32);
    // println!("{:?}", pixel_img.data);
    get_mandelbrot_grid_iteration_values(0.0_f64, 4.0_f64, 0.0_f64, 4.0_f64, height, width);
}

fn mandelbrot_iteration(initial_real: f64, initial_imaginary: f64, real_part: f64, imaginary_part: f64, current_iteration: u32, total_iterations: u32) -> u32 {
    println!("initial_real: {} initial_imaginary: {} real_part: {} imaginary_part: {} current_iteration: {} total_iteration: {}", 
                initial_real,
                initial_imaginary,
                real_part,
                imaginary_part,
                current_iteration,
                total_iterations);
    if current_iteration >= total_iterations {
        return total_iterations;
    }
    else if complex_num_abs(real_part, imaginary_part) > 2.0_f64 {
        return current_iteration
    }
    else {
        let z_squared = complex_num_squared(initial_real, initial_imaginary);
        let new_z_real = z_squared.0 + real_part;
        let new_z_imaginary = z_squared.1 + imaginary_part;
        return mandelbrot_iteration(new_z_real, new_z_imaginary, real_part, imaginary_part, current_iteration + 1, total_iterations);
    }
}

fn complex_num_squared(real_part: f64, imaginary_part: f64) -> (f64, f64) {
    return (real_part * real_part - imaginary_part * imaginary_part, 2.0_f64 * real_part * imaginary_part);
}

fn complex_num_abs(real_part: f64, imaginary_part: f64) -> f64 {
    return (real_part * real_part + imaginary_part * imaginary_part).sqrt();
}

fn get_mandelbrot_grid_iteration_values(real_min: f64, real_max: f64,
                                        imaginary_min: f64, imaginary_max: f64,
                                        mut num_pixels_vert: usize, mut num_pixels_horiz: usize) {
    println!("lets go!");
    // const num_pixels_odd_horiz: usize = 5;
    if num_pixels_horiz % 2 != 0 {
        num_pixels_horiz = num_pixels_horiz + 1;
    }
    // const num_pixels_odd_vert: usize = 5;
    if num_pixels_vert % 2 != 0 {
        num_pixels_vert = num_pixels_vert + 1;
    }
    let step_horiz = (real_max - real_min) / (num_pixels_horiz as f64);
    let step_vert = (imaginary_max - imaginary_min) / (num_pixels_vert as f64);

    let center_real_calc = real_min + step_horiz * ((num_pixels_horiz / 2) as f64);
    let center_imaginary_calc = imaginary_min + step_vert * ((num_pixels_vert / 2) as f64);
    println!("center_real_calc: {} center_imaginary_calc: {}", center_real_calc, center_imaginary_calc);

    let mut pixel_img = image::twodarray::TwoDArray::new(num_pixels_vert, num_pixels_horiz, -1.0_f64);

    for i in 0..num_pixels_horiz {
        for j in 0..num_pixels_vert {
            pixel_img.set(i, j, (i + j) as f64);
        }
    }
    println!("{:?}", pixel_img.data);

    return;
}