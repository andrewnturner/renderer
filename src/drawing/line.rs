use std::mem::swap;
use std::ops::DerefMut;

use image::{ImageBuffer, Pixel};

pub fn draw_line<P: Pixel + 'static, C: DerefMut<Target=[P::Subpixel]>>(image: &mut ImageBuffer<P, C>, colour: P, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32) {
    // We need the line to be more horizontal than vertical. If it's more vertical,
    // then we can transpose the coordinates and run the algorithm, and we just
    // transpose back at the end.
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        steep = true;
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    // We need the line to be left to right. If not we just swap the points and
    // then it will be.
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    for x in x0..x1+1 {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = ((y0 as f32 * (1.0 - t)) + (y1 as f32 * t)) as u32;

        if steep {
            image.put_pixel(y, x as u32, colour);
        } else {
            image.put_pixel(x as u32, y, colour);
        }
    }


}
