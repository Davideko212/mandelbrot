use std::os::raw::c_float;
use num_complex::{Complex, ComplexFloat};
use image::{ImageBuffer, Rgb};

const WIDTH: i32 = 6000;
const HEIGHT: i32 = 4000;
const MAX_ITER: f32 = 80f32;

// Plot window
const RE_START: i32 = -2;
const RE_END: i32 = 1;
const IM_START: i32 = -1;
const IM_END: i32 = 1;

fn main() {
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH as u32, HEIGHT as u32);

    for x in 0i64..i64::from(WIDTH) {
        for y in 0i64..i64::from(HEIGHT) {
            let c = Complex::new(
                (RE_START as f32 + (x as f32 / WIDTH as f32) * (RE_END - RE_START) as c_float),
                (IM_START as f32 + (y as f32 / HEIGHT as f32) * (IM_END - IM_START) as c_float));

            let m = mandelbrot(c);

            let color = 255 - (m * 255.0 / MAX_ITER) as u8;
            image.put_pixel(x as u32, y as u32, Rgb::from([color, color, color]));
        }
    }

    image.save("output.png").unwrap();
}

fn mandelbrot(c: Complex<c_float>) -> f32 {
    let mut z: Complex<f32> = Complex::new(0f32, 0f32);
    let mut n = 0f32;

    while z.abs() <= 2f32 && n < MAX_ITER {
        z = z*z + c;
        n += 1f32;
    }

    n
}
