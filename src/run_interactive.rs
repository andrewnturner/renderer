use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureAccess;

use crate::cli::RendererArgs;
use crate::drawing_target::{Colour, DrawingTarget};
use crate::model::Model;
use crate::render::{render_scene, RenderMode};

pub fn run_interactive(args: RendererArgs) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("renderer", args.width, args.height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Streaming,
            args.width,
            args.height,
        )
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let model = Model::new_from_obj(&args.model).unwrap();

    let mut rotation_counter = 0;

    'running: loop {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 30, 30));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let mut drawing_target =
                    TextureBufferAdapter::new(buffer, pitch, args.width, args.height);

                drawing_target.clear();
                render_scene(
                    &mut drawing_target,
                    Colour::new(0, 255, 255),
                    Colour::new(255, 255, 0),
                    &model,
                    RenderMode::Solid,
                    (2.0 * PI * rotation_counter as f32) / 100.0,
                );
            })
            .unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();

        rotation_counter = (rotation_counter + 1) % 100;

        sleep(Duration::from_millis(50));
    }
}

pub struct TextureBufferAdapter<'a> {
    buffer: &'a mut [u8],
    pitch: usize,
    width: u32,
    height: u32,
}

impl<'a> TextureBufferAdapter<'a> {
    fn new(buffer: &'a mut [u8], pitch: usize, width: u32, height: u32) -> Self {
        Self {
            buffer,
            pitch,
            width,
            height,
        }
    }
}

impl<'a> DrawingTarget for TextureBufferAdapter<'a> {
    fn set_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        let offset = y as usize * self.pitch + x as usize * 3;

        self.buffer[offset] = colour.r;
        self.buffer[offset + 1] = colour.g;
        self.buffer[offset + 2] = colour.b;
    }

    fn clear(&mut self) {
        self.buffer.fill(0);
    }

    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}
