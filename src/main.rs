mod drawing;

use image::{ImageFormat, RgbImage, Rgb};

use drawing::line::draw_line;

fn main() {
    let mut image = RgbImage::new(32, 32);

    draw_line(&mut image, Rgb([255, 0, 0]), 10, 10, 20, 25);

    let path = "out.png";
    image.save_with_format(path, ImageFormat::Png);
}
