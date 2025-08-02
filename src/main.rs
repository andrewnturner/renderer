mod drawing;
mod model;
mod render;

use image::imageops::flip_vertical;
use image::{ImageFormat, Rgb, RgbImage};

use model::Model;
use render::{render_scene, RenderMode};

fn main() {
    let mode = "solid";

    let model = Model::new_from_obj("assets/african_head.obj").unwrap();

    let mut image = RgbImage::new(400, 300);
    match mode {
        "wireframe" => render_scene(
            &mut image,
            Rgb([0, 255, 255]),
            &model,
            RenderMode::Wireframe,
        ),
        "solid" => render_scene(&mut image, Rgb([0, 255, 255]), &model, RenderMode::Solid),
        _ => panic!("Unknown mode"),
    };
    image = flip_vertical(&image);

    let path = "out.png";
    image.save_with_format(path, ImageFormat::Png).unwrap();
}
