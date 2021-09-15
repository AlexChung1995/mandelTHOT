use crate::multi_dim_array::two_d_array::TwoDArray;

pub fn mandelbrot_iteration(
    initial_real: f64,
    initial_imaginary: f64,
    real_part: f64,
    imaginary_part: f64,
    current_iteration: u32,
    total_iterations: u32,
) -> u32 {
    /* println!("initial_real: {} initial_imaginary: {} real_part: {} imaginary_part: {} current_iteration: {} total_iteration: {}", 
                initial_real,
                initial_imaginary,
                real_part,
                imaginary_part,
                current_iteration,
                total_iterations); */
    let abs_val = complex_num_abs(initial_real, initial_imaginary);
    if current_iteration >= total_iterations {
        return total_iterations;
    } else if abs_val >= 2.0_f64 {
        return current_iteration;
    } else {
        let z_squared = complex_num_squared(initial_real, initial_imaginary);
        let new_z_real = z_squared.0 + real_part;
        let new_z_imaginary = z_squared.1 + imaginary_part;
        return mandelbrot_iteration(
            new_z_real,
            new_z_imaginary,
            real_part,
            imaginary_part,
            current_iteration + 1,
            total_iterations,
        );
    }
}

fn complex_num_squared(real_part: f64, imaginary_part: f64) -> (f64, f64) {
    (
        real_part * real_part - imaginary_part * imaginary_part,
        2.0_f64 * real_part * imaginary_part,
    )
}

fn complex_num_abs(real_part: f64, imaginary_part: f64) -> f64 {
    (real_part * real_part + imaginary_part * imaginary_part).sqrt()
}

pub fn get_mandelbrot_grid_iteration_values(
    real_min: f64,
    real_max: f64,
    imaginary_min: f64,
    imaginary_max: f64,
    mut num_pixels_vert: usize,
    mut num_pixels_horiz: usize,
) -> TwoDArray<f64> {
    if num_pixels_horiz % 2 != 0 {
        num_pixels_horiz = num_pixels_horiz + 1;
    }
    if num_pixels_vert % 2 != 0 {
        num_pixels_vert = num_pixels_vert + 1;
    }
    let step_horiz = (real_max - real_min) / (num_pixels_horiz as f64);
    let step_vert = (imaginary_max - imaginary_min) / (num_pixels_vert as f64);

    let mut pixel_img =
        TwoDArray::new(num_pixels_vert, num_pixels_horiz, -1.0_f64);

    for i in 0..(num_pixels_horiz + 1) {
        for j in 0..(num_pixels_vert + 1) {
            let real_val = real_min + step_horiz * i as f64;
            let imaginary_val = imaginary_min + step_vert * j as f64;
            let iterations_til_divergence = mandelbrot_iteration(0.0, 0.0, real_val, imaginary_val, 0, 1000_u32);
            pixel_img.set(i, j, iterations_til_divergence as f64);
        }
    }

    return pixel_img;
}
