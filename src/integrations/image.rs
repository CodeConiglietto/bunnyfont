use image::{RgbaImage, Rgba};

use crate::{
    char::BunnyChar,
    char_transforms::{CharMirror, CharRotation},
    font::BunnyFont,
    traits::{color::Color, pixel_indexable::PixelIndexable, source_image::SourceImage},
};

pub type ImageBunnyFont = BunnyFont<RgbaImage>;
pub type ImageBunnyChar = BunnyChar<Rgba<u8>>;

impl Color for Rgba<u8> {}

impl SourceImage for RgbaImage {
    type Color = Rgba<u8>;

    fn get_pixel_dimensions(&self) -> (usize, usize) {
        (self.width() as usize, self.height() as usize)
    }
}

impl PixelIndexable<Rgba<u8>> for RgbaImage {
    fn get_pixel_at(&self, x: usize, y: usize) -> &Rgba<u8> {
        self.get_pixel(x as u32, y as u32)
    }
}