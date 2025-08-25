use std::{f32::consts::PI, fs::File, io::BufWriter};

use gif::{DisposalMethod, Encoder, Frame, Repeat};
use image::{imageops::flip_vertical, ImageBuffer, Rgba, RgbaImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    cli::RendererArgs,
    drawing_target::{Colour, DrawingTarget},
    model::Model,
    render::{render_scene, RenderMode},
};

pub fn run_static(args: RendererArgs) {
    let num_frames = 100;

    let model = Model::new_from_obj(&args.model).unwrap();

    let file = File::create(args.out.as_ref().unwrap()).unwrap();

    // TODO: Global palette will speed up frames?
    let writer = BufWriter::new(file);
    let mut encoder = Encoder::new(writer, args.width as u16, args.height as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let frames = (0..num_frames)
        .into_par_iter()
        .map(|i| render_one_frame(i, num_frames, args.width, args.height, args.mode, &model))
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
    render_mode: RenderMode,
    model: &Model,
) -> Frame<'static> {
    let mut image = RgbaImage::new(width, height);

    render_scene(
        &mut image,
        Colour::new(0, 255, 255),
        Colour::new(255, 255, 0),
        &model,
        render_mode,
        (2.0 * PI * i as f32) / num_frames as f32,
    );
    image = flip_vertical(&image);

    let mut pixels = image.into_raw();
    let mut frame = Frame::from_rgba_speed(width as u16, height as u16, pixels.as_mut_slice(), 30);
    frame.dispose = DisposalMethod::Background;

    frame
}

impl DrawingTarget for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn set_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        self.put_pixel(x, y, Rgba([colour.r, colour.g, colour.b, 0]));
    }

    fn clear(&mut self) {
        panic!("Not implemented");
    }

    fn width(&self) -> u32 {
        self.width()
    }
    fn height(&self) -> u32 {
        self.height()
    }
}
