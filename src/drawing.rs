use std::cmp::{max, min};
use std::mem::swap;
use std::ops::DerefMut;

use array2d::Array2D;
use image::{ImageBuffer, Pixel};
use nalgebra::{Point2, Point3, Vector3};

pub fn draw_line<P: Pixel + 'static, C: DerefMut<Target = [P::Subpixel]>>(
    image: &mut ImageBuffer<P, C>,
    colour: P,
    p0: Point2<i32>,
    p1: Point2<i32>,
) {
    // We need the line to be more horizontal than vertical. If it's more vertical,
    // then we can transpose the coordinates and run the algorithm, and we just
    // transpose back at the end.
    let (steep, mut a0, mut a1) = if (p0.x - p1.x).abs() < (p0.y - p1.y).abs() {
        (true, Point2::new(p0.y, p0.x), Point2::new(p1.y, p1.x))
    } else {
        (false, p0, p1)
    };

    // We need the line to be left to right. If not we just swap the points and
    // then it will be.
    if a0.x > a1.x {
        swap(&mut a0, &mut a1);
    }

    for x in a0.x..a1.x + 1 {
        let t = (x - a0.x) as f32 / (a1.x - a0.x) as f32;
        let y = ((a0.y as f32 * (1.0 - t)) + (a1.y as f32 * t)) as u32;

        if steep {
            image.put_pixel(y, x as u32, colour);
        } else {
            image.put_pixel(x as u32, y, colour);
        }
    }
}

// Given points A, B, C, we look for numbers u, v such that
//     P = (1 - u - v)A + uB + vC.
// Then
//     P = A + u(AB) + v(AC).
fn barycentric_coordinates(
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    p: Point2<f32>,
) -> Point3<f32> {
    let a = Vector3::new(p2.x - p0.x, p1.x - p0.x, p0.x - p.x);
    let b = Vector3::new(p2.y - p0.y, p1.y - p0.y, p0.y - p.y);

    let u = a.cross(&b);
    if u.z.abs() < 1.0 {
        return Point3::new(-1.0, 1.0, 1.0);
    }

    Point3::new(1.0 - ((u.x + u.y) / u.z), u.y / u.z, u.x / u.z)
}

pub fn draw_triangle<P: Pixel + 'static, C: DerefMut<Target = [P::Subpixel]>>(
    image: &mut ImageBuffer<P, C>,
    z_buffer: &mut Array2D<f32>,
    colour: P,
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
) {
    // Calculate bounding box
    let mut min_x: i32 = image.width() as i32 - 1;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = image.height() as i32 - 1;
    let mut max_y: i32 = 0;
    for p in [p0, p1, p2] {
        min_x = max(0, min(min_x, p.x as i32));
        max_x = min(image.width() as i32 - 1, max(max_x, p.x as i32 + 1));
        min_y = max(0, min(min_y, p.y as i32));
        max_y = min(image.height() as i32 - 1, max(max_y, p.y as i32 + 1));
    }

    // Fill in triangle
    for x in min_x..max_x {
        for y in min_y..max_y {
            let p = Point2::new(x as f32, y as f32);
            let b = barycentric_coordinates(p0, p1, p2, p);

            if b.x < 0.0 || b.y < 0.0 || b.z < 0.0 {
                continue;
            }

            let z = (p0.z * b.x) + (p1.z * b.y) + (p2.z * b.z);
            if &z < z_buffer.get(x as usize, y as usize).unwrap() {
                z_buffer.set(x as usize, y as usize, z).unwrap();
                image.put_pixel(x as u32, y as u32, colour);
            }
        }
    }
}
