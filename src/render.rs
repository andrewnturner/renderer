use std::ops::DerefMut;

use array2d::Array2D;
use image::{ImageBuffer, Pixel, Rgb};

use crate::drawing::{draw_line, draw_triangle};
use crate::matrix::Matrix44;
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

    let viewport = viewport_matrix(width / 8.0, height / 8.0, width * 0.75, height / 0.75);
    let projection = projection_matrix(3.0);

    let mut z_buffer = Array2D::fill_with(f32::NEG_INFINITY, image.width() as usize, image.height() as usize);

    for face in model.faces() {
        let world_0 = model.vertex(face.i0).to_point3();
        let world_1 = model.vertex(face.i1).to_point3();
        let world_2 = model.vertex(face.i2).to_point3();

        let screen_0 = ((viewport * projection) * world_0.to_homog()).to_affine();
        let screen_1 = (viewport * projection * world_1.to_homog()).to_affine();
        let screen_2 = (viewport * projection * world_2.to_homog()).to_affine();

        let normal = (world_2 - world_0).cross(&(world_1 - world_0)).normalise();
        let intensity = normal.dot(&light);

        let rgb = colour.to_rgb();
        let channels = rgb.channels();
        let r = (channels[0] as f32 * intensity) as u8;
        let g = (channels[1] as f32 * intensity) as u8;
        let b = (channels[2] as f32 * intensity) as u8;
        let colour_out = Rgb([r, g, b]);

        // Cull any faces which point away from the camera.
        if intensity > 0.0 {
            draw_triangle(&mut image, &mut z_buffer, colour_out, screen_0, screen_1, screen_2);
        }
    }
}

// Scales to the shape of the viewport, and translate to the position.
fn viewport_matrix(x: f32, y: f32, w: f32, h: f32) -> Matrix44<f32> {
    let depth = 255.0;
    
    let mut matrix = Matrix44::identity();

    // Translate
    matrix.m[0][3] = x + (w / 2.0);
    matrix.m[1][3] = y + (h / 2.0);
    matrix.m[2][3] = depth / 2.0;

    // Scale
    matrix.m[0][0] = w / 2.0;
    matrix.m[1][1] = h / 2.0;
    matrix.m[2][2] = depth / 2.0;

    matrix
}

// Camera on z-axis at dstance c from origin.
fn projection_matrix(c: f32) -> Matrix44<f32> {
    let mut matrix = Matrix44::identity();
    matrix.m[3][2] = -1.0 / c;

    matrix
}
