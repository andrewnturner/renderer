use std::mem::swap;
use std::ops::DerefMut;

use image::{ImageBuffer, Pixel};

use crate::point::Point2;

pub fn draw_line<P: Pixel + 'static, C: DerefMut<Target = [P::Subpixel]>>(
    image: &mut ImageBuffer<P, C>,
    colour: P,
    mut p0: Point2<i32>,
    mut p1: Point2<i32>,
) {
    // We need the line to be more horizontal than vertical. If it's more vertical,
    // then we can transpose the coordinates and run the algorithm, and we just
    // transpose back at the end.
    let mut steep = false;
    if (p0.x - p1.x).abs() < (p0.y - p1.y).abs() {
        steep = true;
        p0.transpose();
        p1.transpose();
    }

    // We need the line to be left to right. If not we just swap the points and
    // then it will be.
    if p0.x > p1.x {
        swap(&mut p0, &mut p1);
    }

    for x in p0.x..p1.x + 1 {
        let t = (x - p0.x) as f32 / (p1.x - p0.x) as f32;
        let y = ((p0.y as f32 * (1.0 - t)) + (p1.y as f32 * t)) as u32;

        if steep {
                image.put_pixel(y, x as u32, colour);
        } else {
            image.put_pixel(x as u32, y, colour);
        }
    }
}

pub fn draw_triangle<P: Pixel + 'static, C: DerefMut<Target = [P::Subpixel]>>(
    image: &mut ImageBuffer<P, C>,
    colour: P,
    mut p0: Point2<i32>,
    mut p1: Point2<i32>,
    mut p2: Point2<i32>,
) {
    // First sort the points so that p0.y <= p1.y <= p2.y.
    if p0.y > p1.y {swap(&mut p0, &mut p1);}
    if p0.y > p2.y {swap(&mut p0, &mut p2);}
    if p1.y > p2.y {swap(&mut p1, &mut p2);}

    let total_height = (p2.y - p0.y) as f32;

    let lower_height = (p1.y - p0.y + 1) as f32;
    for y in p0.y..p1.y + 1 {
        let alpha = (y as f32 - p0.y as f32) / total_height;
        let beta = (y as f32 - p0.y as f32) / lower_height;

        let mut a = p0 + ((p2 - p0).to_f32() * alpha).to_i32();
        let mut b = p0 + ((p1 - p0).to_f32() * beta).to_i32();

        if a.x > b.x {swap(&mut a, &mut b);}

        for j in a.x..b.x + 1 {
            image.put_pixel(j as u32, y as u32, colour);
        }
    }

    let upper_height = (p2.y - p1.y + 1) as f32;
    for y in p1.y..p2.y + 1 {
        let alpha = (y as f32 - p0.y as f32) / total_height;
        let beta = (y as f32 - p1.y as f32) / upper_height;

        let mut a = p0 + ((p2 - p0).to_f32() * alpha).to_i32();
        let mut b = p1 + ((p2 - p1).to_f32() * beta).to_i32();

        if a.x > b.x {swap(&mut a, &mut b);}

        for j in a.x..b.x + 1 {
            image.put_pixel(j as u32, y as u32, colour);
        }
    }
}
