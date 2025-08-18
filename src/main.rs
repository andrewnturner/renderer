mod drawing;
mod model;
mod render;

use std::f32::consts::PI;
use std::fs::File;
use std::io::BufWriter;

use clap::{Parser, ValueHint};
use gif::{DisposalMethod, Encoder, Frame, Repeat};
use image::imageops::flip_vertical;
use image::{Rgb, RgbaImage};

use model::Model;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use render::{render_scene, RenderMode};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of OBJ file to render
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    model: String,

    /// Path to render GIF file to
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    out: String,
}

fn main() {
    let args = Args::parse();

    let mode = "solid";
    let width = 400;
    let height = 300;

    let num_frames = 100;

    let model = Model::new_from_obj(&args.model).unwrap();

    let file = File::create(args.out).unwrap();

    // TODO: Global palette will speed up frames?
    let writer = BufWriter::new(file);
    let mut encoder = Encoder::new(writer, width as u16, height as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let frames = (0..num_frames)
        .into_par_iter()
        .map(|i| render_one_frame(i, num_frames, width, height, mode, &model))
        .collect::<Vec<Frame<'static>>>();

    for frame in frames {
        encoder.write_frame(&frame).unwrap();
    }
}

fn render_one_frame(
    i: usize,
    num_frames: usize,
    width: u32,
    height: u32,
    mode: &str,
    model: &Model,
) -> Frame<'static> {
    println!("Frame {}", i);

    let mut image = RgbaImage::new(width, height);

    match mode {
        "wireframe" => render_scene(
            &mut image,
            Rgb([0, 255, 255]),
            Rgb([255, 255, 0]),
            &model,
            RenderMode::Wireframe,
            (2.0 * PI * i as f32) / num_frames as f32,
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
    let mut frame = Frame::from_rgba_speed(width as u16, height as u16, pixels.as_mut_slice(), 30);
    frame.dispose = DisposalMethod::Background;

    frame
}
