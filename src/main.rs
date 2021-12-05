use image::{RgbImage, Rgb, ImageFormat};

fn main() {
    let mut image = RgbImage::new(32, 32);

    image.put_pixel(10, 10, Rgb([255, 0, 0]));

    let path = "out.png";
    image.save_with_format(path, ImageFormat::Png);
}
