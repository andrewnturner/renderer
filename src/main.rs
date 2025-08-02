mod drawing;
mod model;
mod render;

use std::f32::consts::PI;
use std::fs::File;
use std::io::BufWriter;

use gif::{DisposalMethod, Encoder, Frame, Repeat};
use image::imageops::flip_vertical;
use image::{ImageFormat, Rgb, RgbImage, RgbaImage};

use model::Model;
use render::{render_scene, RenderMode};

fn main() {
    let mode = "solid";
    let width = 400;
    let height = 300;

    let num_frames = 40;

    let model = Model::new_from_obj("assets/african_head.obj").unwrap();

    let path = "out.gif";
    let file = File::create(path).unwrap();

    // TODO: Global palette will speed up frames?
    let writer = BufWriter::new(file);
    let mut encoder = Encoder::new(writer, width as u16, height as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for i in 0..num_frames {
        println!("Frame {}", i);

        let mut image = RgbaImage::new(width, height);

        match mode {
            "wireframe" => render_scene(
                &mut image,
                Rgb([0, 255, 255]),
                Rgb([255, 255, 0]),
                &model,
                RenderMode::Wireframe,
                0.0,
            ),
            "solid" => render_scene(
                &mut image,
                Rgb([0, 255, 255]),
                Rgb([255, 255, 0]),
                &model,
                RenderMode::Solid,
                (2.0 * PI * i as f32) / num_frames as f32,
            ),
            _ => panic!("Unknown mode"),
        };
        image = flip_vertical(&image);

        let mut pixels = image.into_raw();
        let mut frame =
            Frame::from_rgba_speed(width as u16, height as u16, pixels.as_mut_slice(), 20);
        frame.dispose = DisposalMethod::Background;

        encoder.write_frame(&frame).unwrap();
    }
}
