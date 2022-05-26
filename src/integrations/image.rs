use image::{RgbaImage, Rgba};

use crate::{
    char::BunnyChar,
    font::BunnyFont,
    traits::{color::Color, into_scalar::IntoScalar, lerpable::Lerpable, pixel_indexable::PixelIndexable, source_image::SourceImage},
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

impl Lerpable for Rgba<u8> {
    fn lerp(a: &Self, b: &Self, scalar: f32) -> Self {
        [
            (a.0[0] as f32 * (1.0 - scalar) + b.0[0] as f32 * scalar) as u8,
            (a.0[1] as f32 * (1.0 - scalar) + b.0[1] as f32 * scalar) as u8,
            (a.0[2] as f32 * (1.0 - scalar) + b.0[2] as f32 * scalar) as u8,
            (a.0[3] as f32 * (1.0 - scalar) + b.0[3] as f32 * scalar) as u8,
        ].into()
    }
}

impl IntoScalar for Rgba<u8> {
    fn into_scalar(&self) -> f32 {
        let scalar = self.0[0] as f32 / 256.0 +
        self.0[1] as f32 / 256.0 +
        self.0[2] as f32 / 256.0 +
        self.0[3] as f32 / 256.0;

        assert!(scalar >= 0.0);
        assert!(scalar <= 1.0);

        scalar
    }
}