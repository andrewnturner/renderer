use std::ops::DerefMut;

use array2d::Array2D;
use image::{ImageBuffer, Pixel, Rgb, Rgba};
use nalgebra::{Matrix4, Rotation3, Vector2, Vector3, Vector4};

use crate::drawing::{draw_line, draw_triangle};
use crate::model::Model;

pub enum RenderMode {
    Wireframe,
    Solid,
}

impl RenderMode {
    fn draw_triangle<P: Pixel + 'static, C: DerefMut<Target = [P::Subpixel]>>(
        &self,
        image: &mut ImageBuffer<P, C>,
        z_buffer: &mut Array2D<f32>,
        colour: P,
        p0: Vector3<f32>,
        p1: Vector3<f32>,
        p2: Vector3<f32>,
    ) {
        let a0 = Vector2::new(p0.x as i32, p0.y as i32);
        let a1 = Vector2::new(p1.x as i32, p1.y as i32);
        let a2 = Vector2::new(p2.x as i32, p2.y as i32);

        match &self {
            Self::Wireframe => {
                draw_line(image, colour, a0, a1);
                draw_line(image, colour, a1, a2);
                draw_line(image, colour, a2, a0);
            }
            Self::Solid => {
                draw_triangle(image, z_buffer, colour, p0, p1, p2);
            }
        }
    }
}

pub fn render_scene<C: DerefMut<Target = [u8]>>(
    mut image: &mut ImageBuffer<Rgba<u8>, C>,
    colour: Rgb<u8>,
    reverse_colour: Rgb<u8>,
    model: &Model,
    render_mode: RenderMode,
    rotation: f32,
) {
    let width = 400.0;
    let height = 300.0;

    let rotation_matrix = Rotation3::from_axis_angle(&Vector3::y_axis(), rotation);

    let camera = Vector3::new(2.0, 1.0, 5.0);
    let centre = Vector3::new(0.0, 0.0, 0.0);

    let light = camera.normalize();

    let model_view = Matrix4::look_at_lh(&camera.into(), &centre.into(), &Vector3::y_axis());
    let projection = projection_matrix(2.0, 8.0, 0.5, 0.5);
    let scaling = scaling_matrix(width, height);

    let mut z_buffer = Array2D::fill_with(
        f32::INFINITY,
        image.width() as usize,
        image.height() as usize,
    );

    for face in model.faces() {
        let world_0 = rotation_matrix * model.vertex(face.i0).as_vector3();
        let world_1 = rotation_matrix * model.vertex(face.i1).as_vector3();
        let world_2 = rotation_matrix * model.vertex(face.i2).as_vector3();

        let screen_0 = scaling * projection * model_view * to_homog(&world_0);
        let screen_1 = scaling * projection * model_view * to_homog(&world_1);
        let screen_2 = scaling * projection * model_view * to_homog(&world_2);

        let normal = (world_2 - world_0).cross(&(world_1 - world_0)).normalize();
        let intensity = normal.dot(&light);

        // If intensity positive then outward facing, if negative then inward facing.
        // If zero then perpendicular so don't need to draw.
        if intensity > 0.0 {
            let rgb = colour.to_rgb();
            let channels = rgb.channels();
            let r = (channels[0] as f32 * intensity) as u8;
            let g = (channels[1] as f32 * intensity) as u8;
            let b = (channels[2] as f32 * intensity) as u8;
            let colour_out = Rgba([r, g, b, 255]);

            render_mode.draw_triangle(
                &mut image,
                &mut z_buffer,
                colour_out,
                to_affine(&screen_0).into(),
                to_affine(&screen_1).into(),
                to_affine(&screen_2).into(),
            );
        } else if intensity < 0.0 {
            let rgb = reverse_colour.to_rgb();
            let channels = rgb.channels();
            let r = (channels[0] as f32 * -intensity) as u8;
            let g = (channels[1] as f32 * -intensity) as u8;
            let b = (channels[2] as f32 * -intensity) as u8;
            let colour_out = Rgba([r, g, b, 255]);

            render_mode.draw_triangle(
                &mut image,
                &mut z_buffer,
                colour_out,
                to_affine(&screen_0).into(),
                to_affine(&screen_1).into(),
                to_affine(&screen_2).into(),
            );
        }
    }
}

fn to_homog(a: &Vector3<f32>) -> Vector4<f32> {
    Vector4::new(a.x, a.y, a.z, 1.0)
}

fn to_affine(a: &Vector4<f32>) -> Vector3<f32> {
    Vector3::new(a.x / a.w, a.y / a.w, a.z / a.w)
}

fn projection_matrix(near: f32, far: f32, near_width: f32, near_height: f32) -> Matrix4<f32> {
    let mut matrix = Matrix4::identity();

    matrix.m11 = near / near_width;
    matrix.m22 = near / near_height;

    matrix.m33 = far / (far - near);
    matrix.m34 = -(far * near) / (far - near);
    matrix.m43 = 1.0;
    matrix.m44 = 0.0;

    matrix
}

fn scaling_matrix(width: f32, height: f32) -> Matrix4<f32> {
    let mut matrix = Matrix4::identity();

    matrix.m11 = width;
    matrix.m22 = height;

    matrix.m14 = width / 2.0;
    matrix.m24 = height / 2.0;

    matrix
}
