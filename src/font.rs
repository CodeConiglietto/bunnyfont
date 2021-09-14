use std::path::Path;

use crate::{
    char::BunnyChar,
    traits::{graphics_backend::GraphicsBackend, source_image::SourceImage},
};

pub struct BunnyFont<B>
where
    B: GraphicsBackend,
{
    texture: B::Texture,
    char_width: usize,
    char_height: usize,
}

impl<B> BunnyFont<B>
where
    B: GraphicsBackend,
{
    pub fn texture(&self) -> &B::Texture {
        &self.texture
    }

    pub fn new(source_image: B::Texture, char_size: (usize, usize)) -> Result<Self, B::Error>
    {
        let (char_width, char_height) = char_size;

        let (texture_width, texture_height) = source_image.get_pixel_dimensions();

        assert_eq!(
            texture_width % char_width,
            0,
            "Font width {} is not multiple of char width {}",
            texture_width,
            char_width,
        );

        assert_eq!(
            texture_height % char_height,
            0,
            "Font height {} is not multiple of char height {}",
            texture_height,
            char_height,
        );

        Ok(Self {
            texture: source_image,
            char_width,
            char_height,
        })
    }

    //gets coordinates of a character in a rectangle in (x, y, w, h) format
    pub fn get_src_uvs(&self, index: usize) -> (f32, f32, f32, f32) {
        let (texture_width, texture_height) = self.texture.get_pixel_dimensions();

        let float_char_width = self.char_width as f32 / texture_width as f32;
        let float_char_height = self.char_height as f32 / texture_height as f32;

        let (x_index, y_index) = self.get_char_pos_from_index(index);

        (
            x_index as f32 * float_char_width,
            y_index as f32 * float_char_height,
            float_char_width,
            float_char_height,
        )
    }

    //Get the color of the pixel at index in (r, g, b, a)
    //index should be a valid index for the font, and x and y should be 0..char_width, 0..char_height
    pub fn get_char_pixel_at(&self, _char: BunnyChar<B::Color>, _x: usize, _y: usize) -> B::Color {
        todo!();
    }

    pub fn get_index_from_char_pos(&self, x: usize, y: usize) -> usize {
        let (charset_width, _charset_height) = self.charset_dimensions();
        y * charset_width + x
    }

    pub fn get_char_pos_from_index(&self, index: usize) -> (usize, usize) {
        let (charset_width, _charset_height) = self.charset_dimensions();

        (index % charset_width, index / charset_width)
    }

    pub fn char_dimensions(&self) -> (usize, usize) {
        (self.char_width, self.char_height)
    }

    //The dimensions of the font, in chars
    pub fn charset_dimensions(&self) -> (usize, usize) {
        let (texture_width, texture_height) = self.texture.get_pixel_dimensions();

        (
            texture_width / self.char_width,
            texture_height / self.char_height,
        )
    }
}
