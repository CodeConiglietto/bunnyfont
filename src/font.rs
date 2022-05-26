use crate::traits::into_scalar::IntoScalar;
use crate::traits::lerpable::Lerpable;
use crate::traits::pixel_indexable::PixelIndexable;
use crate::char_transforms::{CharMirror, CharRotation};
use crate::{char::BunnyChar, traits::source_image::SourceImage};

pub struct BunnyFont<T> {
    texture: T,
    char_width: usize,
    char_height: usize,
}

impl<T> BunnyFont<T>
where
    T: SourceImage,
{
    pub fn texture(&self) -> &T {
        &self.texture
    }

    pub fn new(source_image: T, char_size: (usize, usize)) -> Self {
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

        Self {
            texture: source_image,
            char_width,
            char_height,
        }
    }

    pub fn len(&self) -> usize {
        let (texture_width, texture_height) = self.texture.get_pixel_dimensions();

        (texture_width / self.char_width) * (texture_height / self.char_height)
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

    pub fn total_char_indices(&self) -> usize {
        let (charset_width, charset_height) = self.charset_dimensions();

        charset_width * charset_height
    }

    pub fn highest_char_index(&self) -> usize {
        self.total_char_indices() - 1
    }
}

impl <T> BunnyFont<T> 
    where T: SourceImage + PixelIndexable<T::Color>,
        T::Color: Lerpable + IntoScalar
{
    pub fn char_pixel(&self, bunny_char: &BunnyChar<T::Color>, x: usize, y: usize) -> T::Color {
        let (char_x, char_y) = self.get_char_pos_from_index(bunny_char.index);
        let (char_width, char_height) = self.char_dimensions();

        assert!(x < char_width);
        assert!(y < char_height);

        let (rot_cw, rot_ch) = (char_width - 1, char_height - 1);
        let (x_inv, y_inv) = (rot_cw - x, rot_ch - y);

        let (x, y) = match bunny_char.rotation {
            CharRotation::None => (x, y),
            CharRotation::Rotation90 => (y_inv, x),
            CharRotation::Rotation180 => (x_inv, y_inv),
            CharRotation::Rotation270 => (y, x_inv),
        };

        let (x, y) = match bunny_char.mirror {
            CharMirror::None => (x, y),
            CharMirror::MirrorX => (x_inv, y),
            CharMirror::MirrorY => (x, y_inv),
            CharMirror::MirrorBoth => (x_inv, y_inv),
        };

        assert!(x < char_width && y < char_height, "char coordinates are out of bounds for char, this may be caused by rotating a non-square char");

        let (char_pix_x, char_pix_y) = (x + char_x * char_width, y + char_y * char_height);

        let texture_pixel = self.texture.get_pixel_at(char_pix_x, char_pix_y);
        let scalar = texture_pixel.into_scalar();
        let foreground = &bunny_char.foreground;

        if let Some(background) = &bunny_char.background {
            Lerpable::lerp(background, foreground, scalar)
        } else {
            Lerpable::lerp(texture_pixel, &foreground, scalar)
        }
    }
}