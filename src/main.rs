mod drawing;
mod model;

use std::ops::DerefMut;

use image::imageops::flip_vertical;
use image::{ImageBuffer, ImageFormat, Pixel, Rgb, RgbImage};

use drawing::line::draw_line;
use model::Model;

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

            let x0 = (v0.x + 1.0) * (width / 2.0);
            let y0 = (v0.y + 1.0) * (height / 2.0);
            let x1 = (v1.x + 1.0) * (width / 2.0);
            let y1 = (v1.y + 1.0) * (height / 2.0);

            draw_line(
                &mut image, colour, x0 as i32, y0 as i32, x1 as i32, y1 as i32,
            );
        }
    }
}

fn main() {
    let model = Model::new_from_obj("assets/african_head.obj").unwrap();

    let mut image = RgbImage::new(500, 500);
    draw_wireframe(&mut image, Rgb([255, 255, 255]), &model);
    image = flip_vertical(&image);

    let path = "out.png";
    image.save_with_format(path, ImageFormat::Png).unwrap();
}
