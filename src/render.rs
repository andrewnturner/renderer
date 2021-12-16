use std::ops::DerefMut;

use image::{ImageBuffer, Pixel, Rgb};

use crate::drawing::{draw_line, draw_triangle};
use crate::model::Model;
use crate::point::{Point2, Point3};

pub fn draw_wireframe<P: Pixel + 'static, C: DerefMut<Target = [P::Subpixel]>>(
    mut image: &mut ImageBuffer<P, C>,
    colour: P,
    model: &Model,
) {
    let width = 300.0;
    let height = 300.0;

    for face in model.faces() {
        for (i0, i1) in face.lines() {
            let v0 = model.vertex(i0);
            let v1 = model.vertex(i1);

            // println!("Verts ({}, {}, {}) -> ({}, {}, {})", v0.x, v0.y, v0.z, v1.x, v1.y, v1.z);

            let p0 = Point2::new(
                ((v0.x + 1.0) * (width / 2.0)) as i32,
                ((v0.y + 1.0) * (height / 2.0)) as i32,
            );
            let p1 = Point2::new(
                ((v1.x + 1.0) * (width / 2.0)) as i32,
                ((v1.y + 1.0) * (height / 2.0)) as i32,
            );

            draw_line(
                &mut image, colour, p0, p1,
            );
        }
    }
}

pub fn draw_solid<C: DerefMut<Target = [u8]>>(
    mut image: &mut ImageBuffer<Rgb<u8>, C>,
    colour: Rgb<u8>,
    model: &Model,
) {
    let width = 300.0;
    let height = 300.0;

    let light = Point3::new(1.0, -1.0, -1.0).normalise();

    for face in model.faces() {
        let v0 = model.vertex(face.i0).to_point3();
        let v1 = model.vertex(face.i1).to_point3();
        let v2 = model.vertex(face.i2).to_point3();

        let p0 = Point2::new(
            ((v0.x + 1.0) * (width / 2.0)) as i32,
            ((v0.y + 1.0) * (height / 2.0)) as i32,
        );
        let p1 = Point2::new(
            ((v1.x + 1.0) * (width / 2.0)) as i32,
            ((v1.y + 1.0) * (height / 2.0)) as i32,
        );
        let p2 = Point2::new(
            ((v2.x + 1.0) * (width / 2.0)) as i32,
            ((v2.y + 1.0) * (height / 2.0)) as i32,
        );

        let normal = (v2 - v0).cross(&(v1 - v0)).normalise();
        let intensity = normal.dot(&light);

        let rgb = colour.to_rgb();
        let channels = rgb.channels();
        let r = (channels[0] as f32 * intensity) as u8;
        let g = (channels[1] as f32 * intensity) as u8;
        let b = (channels[2] as f32 * intensity) as u8;
        let colour_out = Rgb([r, g, b]);

        // Cull any faces which point away from the camera.
        if intensity > 0.0 {
            draw_triangle(&mut image, colour_out, p0, p1, p2);
        }
    }
}
