#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn scale(self, t: f32) -> Self {
        Self {
            r: (self.r as f32 * t) as u8,
            g: (self.g as f32 * t) as u8,
            b: (self.b as f32 * t) as u8,
        }
    }
}

pub trait DrawingTarget {
    fn set_pixel(&mut self, x: u32, y: u32, color: Colour);
    fn clear(&mut self);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
