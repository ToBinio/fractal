use std::ops::Range;
use image::{DynamicImage, GenericImage, Rgba};
use palette::{Gradient, LinSrgb};
use crate::fractal::{IMG_SIZE, SIZE};

const MAX_ITER: u32 = 500;
const MULTISAMPLE_SIZE: u32 = 1;

pub fn compute_img(x_range: Range<f64>, y_range: Range<f64>, complex_const: (f64, f64)) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(IMG_SIZE, IMG_SIZE);

    let gradient = Gradient::new(vec![
        LinSrgb::new(0.0, 0.0, 0.0),
        LinSrgb::new(0.0, 0.0, 1.0),
        LinSrgb::new(1.0, 0.0, 1.0),
        LinSrgb::new(1.0, 0.0, 0.0),
        LinSrgb::new(1.0, 1.0, 1.0),
    ]);

    // every pixel
    for x in 0..IMG_SIZE {
        for y in 0..IMG_SIZE {
            let mut iter_sum = 0;

            //multi sampling
            for x_offset in 0..MULTISAMPLE_SIZE {
                for y_offset in 0..MULTISAMPLE_SIZE {
                    let x_offset = x_offset as f64 / MULTISAMPLE_SIZE as f64;
                    let y_offset = y_offset as f64 / MULTISAMPLE_SIZE as f64;

                    let x = x as f64 + x_offset;
                    let y = y as f64 + y_offset;

                    let mut iter_count = MAX_ITER;

                    let mut normal = ((x / IMG_SIZE as f64)
                        * (x_range.end - x_range.start).abs()
                        + x_range.start)
                        / SIZE
                        * 2.0;

                    let mut imaginary = ((y / IMG_SIZE as f64)
                        * (y_range.end - y_range.start).abs()
                        + y_range.start)
                        / SIZE
                        * 2.0;

                    for i in 0..MAX_ITER {
                        let mut temp_normal = normal.powi(2);
                        let mut temp_imaginary = normal * imaginary * 2.0;
                        temp_normal += -imaginary.powi(2);

                        temp_normal += complex_const.0;
                        temp_imaginary += complex_const.1;

                        normal = temp_normal;
                        imaginary = temp_imaginary;

                        let size = normal * normal + imaginary * imaginary;
                        if size > 4.0 {
                            iter_count = i;
                            break;
                        }
                    }

                    iter_sum += iter_count;
                }
            }

            let iter_avg = iter_sum as f64 / MULTISAMPLE_SIZE.pow(2) as f64;
            let color = gradient.get(iter_avg / MAX_ITER as f64);

            DynamicImage::put_pixel(
                &mut image,
                x,
                y,
                Rgba([
                    (color.red * 255.0) as u8,
                    (color.green * 255.0) as u8,
                    (color.blue * 255.0) as u8,
                    255,
                ]),
            );
        }
    }

    image
}