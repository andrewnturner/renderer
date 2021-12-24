mod drawing;
mod matrix;
mod model;
mod point;
mod render;

use image::imageops::flip_vertical;
use image::{ImageFormat, Rgb, RgbImage};

use model::Model;
use render::{draw_wireframe, draw_solid};

fn main() {
    let mode = "solid";

    let model = Model::new_from_obj("assets/african_head.obj").unwrap();

    let mut image = RgbImage::new(500, 500);
    match mode {
        "wireframe" => draw_wireframe(&mut image, Rgb([255, 255, 255]), &model),
        "solid" => draw_solid(&mut image, Rgb([255, 255, 255]), &model),
        _ => panic!("Unknown mode"),
    };
    image = flip_vertical(&image);

    let path = "out.png";
    image.save_with_format(path, ImageFormat::Png).unwrap();
}
